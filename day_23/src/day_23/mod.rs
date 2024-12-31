use std::collections::{HashMap, HashSet};

use aoc_helper::Day;

#[cfg(test)]
mod test;

pub(crate) struct Impl {}

impl Day<Vec<(String, String)>, usize, usize> for Impl {
    fn parse(&self, input: String) -> Vec<(String, String)> {
        input
            .lines()
            .map(|line| {
                let vec: Vec<String> = line.split('-').map(|s| s.to_owned()).collect();
                if let [left, right] = &vec[..] {
                    (left.clone(), right.clone())
                } else {
                    panic!()
                }
            })
            .collect()
    }

    fn part_1(&self, connections: &Vec<(String, String)>) -> usize {
        let map = create_map(connections);

        let mut triples = HashSet::new();

        for pair in connections {
            let start_node = pair.0.to_string();

            // start with nodes starting with t
            if start_node.starts_with('t') {
                // find all nodes this node is connected to
                let connected = &map[&start_node];

                // now find triplets
                for left_node in connected {
                    let other_connected = &map[left_node];

                    for right_node in other_connected {
                        if **right_node == start_node {
                            continue;
                        }
                        if connected.contains(&right_node) {
                            let mut triple = vec![
                                start_node.to_string(),
                                left_node.to_string(),
                                right_node.to_string(),
                            ];
                            triple.sort();
                            triples.insert(triple);
                        }
                    }
                }
            }
        }

        triples.len()
    }

    fn part_2(&self, connections: &Vec<(String, String)>) -> usize {
        let map = create_map(connections);

        todo!()
    }
}

fn create_map(connections: &Vec<(String, String)>) -> HashMap<&String, Vec<&String>> {
    let mut map: HashMap<&String, Vec<&String>> = HashMap::new();

    for pair in connections {
        map.entry(&pair.0)
            .and_modify(|list| list.push(&pair.1))
            .or_insert(vec![&pair.1]);
        map.entry(&pair.1)
            .and_modify(|list| list.push(&pair.0))
            .or_insert(vec![&pair.0]);
    }
    map
}
