use std::{
    collections::{HashMap, VecDeque},
    fmt,
};

use shared::types_2d::{Coords, Grid, NeighbourIter};

/// This trait allows Grids to be in charge of whether cells are valid neighbours for pathfinding purposes
// I could add "cost" to the output of neighbours so that a pathfinding algorithm can choose "easier" paths
// but for the purposes of day12 every move is equal cost so it isn't required.
pub trait Neighbours<'a> {
    type Idx: std::cmp::Eq + std::hash::Hash + Copy + fmt::Debug;
    type Iter: Iterator<Item = Self::Idx> + 'a;

    fn neighbours(&'a self, index: Self::Idx) -> Self::Iter;
}

impl<'a, T> Neighbours<'a> for Grid<T> {
    type Idx = Coords;
    type Iter = NeighbourIter<'a>;

    fn neighbours(&'a self, index: Coords) -> Self::Iter {
        self.neighbours(index)
    }
}

/// implemented using the A* algorithm
pub fn shortest_path<'a, G>(
    graph: &'a G,
    start: <G as Neighbours<'a>>::Idx,
    goal: <G as Neighbours<'a>>::Idx,
) -> Vec<<G as Neighbours<'a>>::Idx>
where
    G: Neighbours<'a>,
{
    // frontier is a "queue" of steps to search for next
    let mut frontier = VecDeque::new();
    frontier.push_back(start);

    // steps_taken is a HashMap to track the steps we took as we take them
    // key: value => "where we are": "where we came from"
    let mut steps_taken =
        HashMap::<<G as Neighbours<'a>>::Idx, Option<<G as Neighbours<'a>>::Idx>>::new();
    steps_taken.insert(start, None);

    // search for the goal by checking neighbours
    'search: while let Some(current) = frontier.pop_front() {
        for neighbour in graph.neighbours(current) {
            if current == goal {
                break 'search;
            }
            steps_taken.entry(neighbour).or_insert_with(|| {
                frontier.push_back(neighbour);
                Some(current)
            });
        }
    }

    // follow the steps taken back to the start to build the path
    let mut path = VecDeque::new();
    let mut retraced_step = Some(goal);
    while let Some(step) = retraced_step {
        retraced_step = steps_taken[&step];
        path.push_back(step);
    }
    path.into()
}

/// implemented using the A* algorithm
pub fn shortest_path_to_dynamic_goal<'a, G>(
    graph: &'a G,
    start: <G as Neighbours<'a>>::Idx,
    satisfies_goal: fn(<G as Neighbours<'a>>::Idx, &G) -> bool,
) -> Vec<<G as Neighbours<'a>>::Idx>
where
    G: Neighbours<'a>,
{
    // the goal will be discovered during the course of pathfinding
    let mut goal: Option<<G as Neighbours<'a>>::Idx> = None;

    // frontier is a "queue" of steps to search for next
    let mut frontier = VecDeque::new();
    frontier.push_back(start);

    // steps_taken is a HashMap to track the steps we took as we take them
    // key: value => "where we are": "where we came from"
    let mut steps_taken =
        HashMap::<<G as Neighbours<'a>>::Idx, Option<<G as Neighbours<'a>>::Idx>>::new();
    steps_taken.insert(start, None);

    // search for the goal by checking neighbours
    'search: while let Some(current) = frontier.pop_front() {
        for neighbour in graph.neighbours(current) {
            if satisfies_goal(current, graph) {
                goal = Some(current);
                break 'search;
            }
            steps_taken.entry(neighbour).or_insert_with(|| {
                frontier.push_back(neighbour);
                Some(current)
            });
        }
    }

    // follow the steps taken back to the start to build the path
    let mut path = VecDeque::new();
    let mut retraced_step = goal;
    while let Some(step) = retraced_step {
        retraced_step = steps_taken[&step];
        path.push_back(step);
    }
    path.into()
}
