mod branch;
mod network;
mod node_id;

use std::sync::{mpsc, Arc};

use branch::{Branch, BranchId, Branches, Step};
use network::{Network, Node};
use node_id::NodeId;
use regex::{Captures, Regex};
use shared::{read_input, receive_answers, run_part_threaded};

type Answer = usize;

fn main() {
    let (tx, rx) = mpsc::channel();
    let input = read_input(2022, 16);
    let shared_input = Arc::new(input);

    run_part_threaded(1, shared_input.clone(), part1, tx.clone());
    run_part_threaded(2, shared_input, part2, tx);

    receive_answers(rx);
}

fn parse_input(input: &str) -> Network {
    let re = Regex::new(
        r#"Valve (?P<id>[A-Z]+) has flow rate=(?P<flow_rate>[0-9]+); tunnels? leads? to valves? (?P<connections>(?:[A-Z]+(?:, )?)+)"#
    ).unwrap();
    let parse_id = |cap: &Captures| cap.name("id").expect("valid input").as_str().into();
    let parse_flow_rate = |cap: &Captures| {
        cap.name("flow_rate")
            .expect("valid input")
            .as_str()
            .parse::<usize>()
            .expect("valid input")
    };
    let parse_connections = |cap: &Captures| {
        cap.name("connections")
            .expect("valid input")
            .as_str()
            .split(", ")
            .map(NodeId::from)
            .collect::<Vec<_>>()
    };

    let mut network = Network::new();

    input
        .lines()
        .map(|line| {
            let cap = re.captures(line).expect("valid input");
            Node::new(
                parse_id(&cap),
                parse_flow_rate(&cap),
                parse_connections(&cap),
            )
        })
        .for_each(|node| {
            network.add_node(node);
        });

    network
}

// OK what we want right, is a depth first search of a tree of posisble moves between working valves
// we try the highest yield path first, all the way to the end, and record its final score
// then we try different paths, starting with the alternative decision with the next highest yield:

// e.g.
//
//            A
//     B=4        C=5
//  D=9   C=5  B=4   E=1
//
// we'd try A->C->B = 9 first and have the following alternatives: AB=4 CE=1, so we try AB next
// following that decision we get A->B->D = 13 with the following alternatives: CE=1 (from the previous step), BC=5 (from this step) so we try BC next
// following that decision we get A->B->C = 9 with the following alternatives: CE = 1 (still there), so we try that next
// following that decision we get A->C->E = 6 with no alternatives remaining
//                                  ^^^ - we had to recall which "decision tree" (oh gosh I bet that's what decision trees are) this decision belonged to to rebuild this
//
// Concerns are there are a STUPID number of possible decisions, so it would be necessary to terminate early by deciding that all the remaining alternatives must lead to lower scores.
// it might be easier/safer to prune entire trees with a Breadth first search.
//
// calculate the yields of all the possible choices from the current node and then... eliminate only the lowest yield, then continue until there is only one path remaining?
// we could explore all bar the lowest yield branch each step, and also eliminate the lowest total yield branch each step?
//
// or we operate a kind of frontier queue of branches to explore. We explore the branch whose next step has the highest yield - taking into account the
// remaining duration would strongly favour exploring breadth of choices (which is good, we won't miss out on a winning strategy) but may also pursue non-optimal paths
// to nearly their full extent (which is bad because it will take ages). I think it's worth a shot!

// Frontier exploring queue:
// Each item in the queue will be a Vec<NodeId> of steps taken, and a BranchId into an Arena of all the branches we are exploring and a total yield so far, and all
// the possible next steps along with their yield: Vec<(NodeId, yield)> plus a time_remaining.
// no hang on that's too much... The queue only needs to be BranchId and next step (NodeId) and the step's yield - a queu of next steps to take
//
// the `Branch`s in the Arena of branches will hold the Vec<NodeId> of steps taken, the total yield so far and the time remaining (and the set of open valves).
// Once a branch runs out of time/valves to open (i.e. finishes) it will calculate its total score and add it to a HashMap of BranchId: usize `branch_scores`.
// and remove itself from the Arena of branches (or otherwise signal that it no longer needs to be considered for exploration)

// Then the algoritham is thus:
//
// 0. * Make an initial branch that includes the starting node, a total yield of 0 and all of the remaining time and an empty set of open valves.
//    * calculate the yield for each connection of the starting node, and set the branch id of each to the initial branch
//    * push these into the frontier queue
//    * set the current node to the starting node
//
// 1. * pop the highest yield item from the frontier queue
//    * set the current node to the item's next step node id
//    * Add a new branch to the Arena that extends the item's branch (looked up by the item's BranchId)
//
// 2. * calculate the yield for each connection of the current node (that hasn't been opened!), and set the branch id to the new branch
//    * push these into the frontier queue
//
// repeat steps 1 and 2 until... crap! This will take ages because it never prunes any low yield branches, it just does them last.
//
// 3. * remove the n lowest yield items from the frontier queue until the length of the frontier queue is SOME TUNEABLE PARAMETER (lower is faster, but might get the wrong answer!)

fn step_from_connection(
    (distance, id): &(usize, NodeId),
    branch_id: BranchId,
    branches: &Branches,
    network: &Network,
) -> Option<Step> {
    // we have to explicitly reference this so it isn't moved into the closure
    let network = &network;
    let branches = &branches;
    let branch = branches.get(branch_id).expect("branch exists!");
    if branch.opened.contains(id) {
        return None;
    }
    let flow_rate = network.get(*id).expect("connected node exists").flow_rate;
    let duration = distance + 1;
    let n = branch.remaining_time.checked_sub(duration)?;
    let yld = n * flow_rate;

    Some(Step::new(*id, yld, duration, branch_id))
}

fn part1(input: &str) -> Answer {
    let network = parse_input(input);

    // start at AA
    let current_node_id = NodeId::AA;
    let network = network.consolidate(&current_node_id);

    let mut branches = Branches::new();
    let mut frontier = Vec::new();

    // this is the tuneable parameter to speed things up
    // I've simply experimented a little to end up with this number - 20_000 got me the right answer but was quite slow
    // 3000 gets the same answer faster, 2000 gets a lower answer
    let frontier_limit = 3000;

    let initial_branch = branches.add_branch(Branch::new(30));

    let current_node = network.get(current_node_id).expect("current node exists");
    current_node
        .connections
        .iter()
        .filter_map(|item| step_from_connection(item, initial_branch, &branches, &network))
        .for_each(|step| frontier.push(step));

    loop {
        frontier.sort_by_key(|step| step.yld);
        let step = frontier.pop();
        if step.is_none() {
            break;
        }
        let step = step.unwrap();
        // try the step
        let current_node = network.get(step.node).expect("current node exists");
        let branch = branches.get(step.branch).expect("branch exists");
        let branch = Branch::extend(branch, step.clone());
        let branch_id = branches.add_branch(branch);

        current_node
            .connections
            .iter()
            .filter_map(|item| step_from_connection(item, branch_id, &branches, &network))
            .for_each(|step| {
                if frontier.len() < frontier_limit {
                    frontier.push(step)
                } else {
                    // dbg!("Frontier limit reached: {frontier_limit}");
                }
            });
    }

    let solution = branches
        .0
        .into_iter()
        .max_by(|a, b| a.total_yield.cmp(&b.total_yield))
        .expect("single solution");

    solution.total_yield
}

// The paired algoritham might be:
//
// 0. * Make an initial branch that includes the starting node, a total yield of 0 and all of the remaining time and an empty set of open valves.
//    * calculate the yield for each connection of the starting node, and set the branch id of each to the initial branch
//    * push every pair of these into the frontier queue
//    * set your current node and your elephant's current node to the starting node
//
// 1. * pop the highest yield item (a move for you and your elephant) from frontier queue
//    * set your current node to the item's next human step node id
//    * set your elephant's current node to the item's next elephant step node id
//    * Add a new branch to the Arena that extends the item's branch (looked up by the item's BranchId)
//
// 2. * calculate the yield for each connection of each current node (that hasn't been opened!), and set the branch id to the new branch
//    * push these into the frontier queue - all combinations of human and elephant next steps.
//
// 3. * remove the n lowest yield items from the frontier queue until the length of the frontier queue is SOME TUNEABLE PARAMETER (lower is faster, but might get the wrong answer!)
fn part2(input: &str) -> Answer {
    let _ = input;
    todo!("fix part 2")
    // let network = parse_input(input);

    // // start at AA
    // let current_node_id = NodeId::AA;
    // let network = network.consolidate(&current_node_id);

    // let mut branches = Branches::new();
    // let mut frontier = Vec::new();

    // // this is the tuneable parameter to speed things up
    // let frontier_limit = 1_000_000;

    // let initial_branch = branches.add_branch(Branch::new(26));

    // let current_node = network.get(current_node_id).expect("current node exists");
    // current_node
    //     .connections
    //     .iter()
    //     .filter_map(|item| step_from_connection(item, initial_branch, &branches, &network))
    //     .for_each(|step| frontier.push(step));

    // loop {
    //     frontier.sort_by_key(|step| step.yld);
    //     let step = frontier.pop();
    //     if step.is_none() {
    //         break;
    //     }
    //     let step = step.unwrap();
    //     // try the step
    //     let current_node = network.get(step.node).expect("current node exists");
    //     let branch = branches.get(step.branch).expect("branch exists");
    //     let branch = Branch::extend(branch, step.clone());
    //     let branch_id = branches.add_branch(branch);

    //     current_node
    //         .connections
    //         .iter()
    //         .filter_map(|item| step_from_connection(item, branch_id, &branches, &network))
    //         .for_each(|step| {
    //             if frontier.len() < frontier_limit {
    //                 frontier.push(step)
    //             } else {
    //                 // dbg!("Frontier limit reached: {frontier_limit}");
    //             }
    //         });
    // }

    // let solution = branches
    //     .0
    //     .into_iter()
    //     .max_by(|a, b| a.total_yield.cmp(&b.total_yield))
    //     .expect("single solution");

    // solution.total_yield
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use super::*;

    const INPUT: &str = r"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

    #[test]
    fn test_part1_moves_yield_expected_total_flow() {
        let network = parse_input(INPUT);

        // start at AA
        let mut current_node_id = NodeId::AA;
        let mut remaining_time: usize = 30;

        let network = network.consolidate(&current_node_id);
        let mut opened = HashSet::<NodeId>::new();
        let mut total: usize = 0;

        // rigging this for testing purposes
        use NodeId::*;
        let moves = [DD, BB, JJ, HH, EE, CC, AA, AA, AA, AA, AA, AA];

        for mv in moves {
            let current_node = network.get(current_node_id).expect("current node exists");
            if let Some((_highest_yield, id, distance)) = current_node
                .connections
                .iter()
                .filter_map(|(distance, id)| {
                    if opened.contains(id) {
                        return None;
                    }
                    // calculate yields based on the remaining time, flow rates and CURRENT distances (plus the time to open)
                    let flow_rate = network.get(*id).expect("connected node exists").flow_rate;
                    let n = remaining_time.checked_sub(*distance + 1)?;
                    let this_yield = n * flow_rate;
                    Some((this_yield, *id, distance))
                })
                .find(|tup| tup.1 == mv)
            {
                dbg!(distance);
                current_node_id = dbg!(id);

                // tick down time (+1 to open)
                let time_taken: usize = distance + 1;
                remaining_time -= time_taken;

                let flow: usize = opened
                    .iter()
                    .map(|node_id| {
                        network
                            .get(*node_id)
                            .expect("closed valve to exist")
                            .flow_rate
                    })
                    .sum::<usize>();

                // open the valve
                opened.insert(current_node_id);

                total += dbg!(flow) * dbg!(time_taken);
            } else {
                let flow: usize = opened
                    .iter()
                    .map(|node_id| {
                        network
                            .get(*node_id)
                            .expect("closed valve to exist")
                            .flow_rate
                    })
                    .sum::<usize>();
                total += dbg!(flow);
                remaining_time -= 1;
            }

            if remaining_time == 0 {
                break;
            }
        }

        assert_eq!(total, 1651);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 1651);
    }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(INPUT), 45000);
    // }
}
