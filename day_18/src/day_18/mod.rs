use std::collections::HashMap;

use aoc_helper::{board::Board, collections::PriorityQueue, navigation, vectors::Vec2D, Day};

#[cfg(test)]
mod test;

pub(crate) struct Impl {}

impl Day<Vec<Vec2D>, u32, Vec2D> for Impl {
    fn parse(&self, input: String) -> Vec<Vec2D> {
        input
            .lines()
            .map(|line| {
                if let Some((val1, val2)) = line.split_once(',') {
                    Vec2D::new(val1.parse().unwrap(), val2.parse().unwrap())
                } else {
                    panic!("invalid input")
                }
            })
            .collect()
    }

    fn part_1(&self, bytes: &Vec<Vec2D>) -> u32 {
        let (size, iterations) = determine_params(bytes);

        solve_maze(bytes, size, iterations).unwrap()
    }

    fn part_2(&self, bytes: &Vec<Vec2D>) -> Vec2D {
        let (size, skip) = determine_params(bytes);

        // from part 1 we know that we can at least skip 1024 iterations
        for (i, _byte) in bytes.iter().enumerate().skip(skip) {
            if solve_maze(bytes, size, i + 1).is_none() {
                return bytes[i];
            }
        }

        panic!("No blocks found")
    }
}

fn determine_params(bytes: &Vec<Vec2D>) -> (usize, usize) {
    if bytes.len() < 1000 {
        return (7, 12);
    }
    return (71, 1024);
}

fn solve_maze(bytes: &Vec<Vec2D>, size: usize, iterations: usize) -> Option<u32> {
    let board_content = std::iter::repeat(std::iter::repeat('.').take(size).collect())
        .take(size)
        .collect();
    let mut board = Board::new(board_content);
    for (i, byte) in bytes.iter().enumerate() {
        if i == iterations {
            break;
        }
        board.set(*byte, '#');
    }

    let start_pos = Vec2D::new(0, 0);
    let end: i32 = size.try_into().unwrap();
    let target = Vec2D::new(end - 1, end - 1);
    let mut priority_queue = PriorityQueue::new();
    priority_queue.push((start_pos, 0), 0);
    let mut visited_spaces = HashMap::new();
    visited_spaces.insert(start_pos, 0);
    let directions = navigation::get_adjecent_directions();
    loop {
        let lowest = priority_queue.pop_lowest();

        if let Some((pos, score)) = lowest {
            for direction in directions {
                let new_pos = pos + direction;
                let new_score = score + 1;

                if !board.is_in_bounds(new_pos) || *board.get(new_pos) == '#' {
                    continue;
                }

                if new_pos == target {
                    return Some(new_score);
                }

                if let Some(old_score) = visited_spaces.get(&new_pos) {
                    if new_score >= *old_score {
                        continue;
                    }
                }

                if let Some(old_score) = visited_spaces.get(&new_pos) {
                    if score < *old_score {
                        visited_spaces.insert(new_pos, new_score);
                        priority_queue.push((new_pos, new_score), new_score);
                    }
                } else {
                    visited_spaces.insert(new_pos, new_score);
                    priority_queue.push((new_pos, new_score), new_score);
                }
            }
        } else {
            return None;
        }
    }
}
