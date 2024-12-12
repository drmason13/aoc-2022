use std::collections::BTreeMap;

#[derive(Debug, Clone, Default)]
pub struct Network {
    nodes: BTreeMap<NodeId, Node>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
    pub flow_rate: usize,
    pub open: bool,
    pub id: NodeId,
    pub connections: Vec<(usize, NodeId)>,
}

impl Node {
    pub fn new(id: NodeId, flow_rate: usize, connections: Vec<NodeId>) -> Self {
        Node {
            flow_rate,
            open: false,
            id,
            connections: connections.into_iter().map(|s| (1, s)).collect(),
        }
    }
}

use shared::pathfinding::{self, Neighbours};

use crate::node_id::NodeId;

impl<'a> Neighbours<'a> for &Network {
    type Idx = NodeId;
    type Iter = NeighboursIter<'a>;

    fn neighbours(&'a self, index: Self::Idx) -> Self::Iter {
        NeighboursIter {
            iter: Box::new(
                self.get(index)
                    .expect("node exists when asking for its neighbours")
                    .connections
                    .iter()
                    .map(|(_weight, id)| *id),
            ),
        }
    }
}

pub struct NeighboursIter<'a> {
    iter: Box<dyn Iterator<Item = NodeId> + 'a>,
}

impl Iterator for NeighboursIter<'_> {
    type Item = NodeId;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl Network {
    pub fn new() -> Self {
        Network {
            nodes: BTreeMap::new(),
        }
    }

    pub fn get(&self, id: NodeId) -> Option<&Node> {
        self.nodes.get(&id)
    }

    pub fn get_mut(&mut self, id: NodeId) -> Option<&mut Node> {
        self.nodes.get_mut(&id)
    }

    pub fn new_node(&mut self, flow_rate: usize, id: &str, connections: &[&str]) {
        let node = Node::new(
            id.into(),
            flow_rate,
            connections.iter().map(|&s| s.into()).collect(),
        );

        self.nodes.insert(id.into(), node);
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.id, node);
    }

    /// returns a new Network with nodes with a flow_rate of 0 removed, and the remaining nodes' connections updated to the length of the shortest path between them
    pub fn consolidate(&self, start: &NodeId) -> Network {
        let working_valves: BTreeMap<NodeId, Node> = self
            .nodes
            .iter()
            .filter(|(id, node)| node.flow_rate > 0 || id == &start)
            .map(|(id, node)| (*id, node.clone()))
            .collect();

        let mut consolidated = Network::new();

        for from in working_valves.values() {
            let mut from_node = Node::new(from.id, from.flow_rate, Vec::new());
            for to in working_valves.values() {
                if from == to {
                    continue;
                }
                let shortest_path = pathfinding::shortest_path(&self, from.id, to.id);
                // path includes start and end, so the distance is one less
                let weight = shortest_path.len() - 1;
                from_node.connections.push((weight, to.id));
            }
            consolidated.add_node(from_node);
        }

        consolidated
    }
}
