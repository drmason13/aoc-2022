use std::collections::HashSet;

use crate::node_id::NodeId;

#[derive(Debug)]
pub struct Branches(pub Vec<Branch>);

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BranchId(usize);

#[derive(Debug)]
pub struct Branch {
    pub steps_taken: Vec<NodeId>,
    pub total_yield: usize,
    pub remaining_time: usize,
    pub opened: HashSet<NodeId>,
}

#[derive(Debug, Clone)]
pub struct Step {
    pub node: NodeId,
    pub yld: usize,
    pub duration: usize,
    pub branch: BranchId,
}

impl Step {
    pub fn new(node: NodeId, yld: usize, duration: usize, branch: BranchId) -> Self {
        Step {
            node,
            yld,
            duration,
            branch,
        }
    }
}

impl Branch {
    pub fn new(remaining_time: usize) -> Self {
        Branch {
            steps_taken: Vec::new(),
            total_yield: 0,
            remaining_time,
            opened: HashSet::new(),
        }
    }

    pub fn extend(branch: &Branch, step: Step) -> Self {
        let mut steps_taken: Vec<_> = branch.steps_taken.to_vec();
        steps_taken.push(step.node);
        let mut opened: HashSet<_> = branch.opened.iter().copied().collect();
        opened.insert(step.node);
        Branch {
            steps_taken,
            total_yield: branch.total_yield + step.yld,
            remaining_time: branch.remaining_time - step.duration,
            opened,
        }
    }
}

impl Branches {
    pub fn new() -> Self {
        Branches(Vec::new())
    }

    pub fn add_branch(&mut self, branch: Branch) -> BranchId {
        let idx = self.0.len();
        self.0.push(branch);
        BranchId(idx)
    }

    pub fn get(&self, id: BranchId) -> Option<&Branch> {
        self.0.get(id.0)
    }
}

#[derive(Debug, Clone)]
pub struct PairStep {
    pub yld: usize,
    pub my_node: NodeId,
    pub el_node: NodeId,
    pub my_duration: usize,
    pub el_duration: usize,
    pub branch: BranchId,
}
