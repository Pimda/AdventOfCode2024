use std::collections::{HashMap, HashSet};

use aoc_helper::{board::Board, collections::PriorityQueue, vectors::Vec2D, Day};

#[cfg(test)]
mod test;

pub(crate) struct Impl {}

impl Day<Board<char>, i32, usize> for Impl {
    fn parse(&self, input: String) -> Board<char> {
        Board::new(input.lines().map(|line| line.chars().collect()).collect())
    }

    fn part_1(&self, board: &Board<char>) -> i32 {
        let start_pos = board
            .iter_all_coordinates()
            .find(|coord| *board.get(*coord) == 'S')
            .unwrap();
        let start_direction = Vec2D::new(1, 0);
        let mut queue = PriorityQueue::new();
        let mut lowest_cost_to_reach = HashMap::new();
        let mut start_visited_positions = HashSet::new();
        start_visited_positions.insert(start_pos);

        queue.push((start_pos, start_direction, start_visited_positions, 0), 0);

        loop {
            let (pos, current_dir, visited_positions, score) =
                queue.pop_lowest().expect("No item on queue");

            if *board.get(pos) == 'E' {
                return score;
            }

            add_next_step(
                pos,
                1,
                current_dir,
                &visited_positions,
                board,
                score,
                &mut lowest_cost_to_reach,
                &mut queue,
                &|a, b| a < b,
            );

            let mut cloned_dir = current_dir;
            cloned_dir.rotate_left();
            add_next_step(
                pos,
                1001,
                cloned_dir,
                &visited_positions,
                board,
                score,
                &mut lowest_cost_to_reach,
                &mut queue,
                &|a, b| a < b,
            );

            let mut cloned_dir = current_dir;
            cloned_dir.rotate_right();
            add_next_step(
                pos,
                1001,
                cloned_dir,
                &visited_positions,
                board,
                score,
                &mut lowest_cost_to_reach,
                &mut queue,
                &|a, b| a < b,
            );
        }
    }

    fn part_2(&self, board: &Board<char>) -> usize {
        let start_pos = board
            .iter_all_coordinates()
            .find(|coord| *board.get(*coord) == 'S')
            .unwrap();
        let start_direction = Vec2D::new(1, 0);
        let mut queue = PriorityQueue::new();
        let mut lowest_cost_to_reach = HashMap::new();
        let mut start_visited_positions = HashSet::new();
        start_visited_positions.insert(start_pos);

        queue.push((start_pos, start_direction, start_visited_positions, 0), 0);

        let mut lowest_score = None;
        let mut valid_routes = vec![];

        loop {
            let (pos, current_dir, visited_positions, score) =
                queue.pop_lowest().expect("No item on queue");

            if *board.get(pos) == 'E' {
                if let Some(lowest_score_val) = lowest_score {
                    if score == lowest_score_val {
                        valid_routes.push(visited_positions.clone());
                    } else {
                        let unique_visited_points: HashSet<Vec2D> =
                            valid_routes.into_iter().flatten().collect();

                        return unique_visited_points.len();
                    }
                } else {
                    lowest_score = Some(score);
                    valid_routes.push(visited_positions.clone());
                }
            }

            add_next_step(
                pos,
                1,
                current_dir,
                &visited_positions,
                board,
                score,
                &mut lowest_cost_to_reach,
                &mut queue,
                &|a, b| a <= b,
            );

            let mut cloned_dir = current_dir;
            cloned_dir.rotate_left();
            add_next_step(
                pos,
                1001,
                cloned_dir,
                &visited_positions,
                board,
                score,
                &mut lowest_cost_to_reach,
                &mut queue,
                &|a, b| a <= b,
            );

            let mut cloned_dir = current_dir;
            cloned_dir.rotate_right();
            add_next_step(
                pos,
                1001,
                cloned_dir,
                &visited_positions,
                board,
                score,
                &mut lowest_cost_to_reach,
                &mut queue,
                &|a, b| a <= b,
            );
        }
    }
}

fn _print_route(board: &Board<char>, visited_positions: &HashSet<Vec2D>) {
    let mut y = -1;
    for coord in board.iter_all_coordinates() {
        if coord.y != y {
            y = coord.y;
            println!()
        }
        if visited_positions.contains(&coord) {
            print!("O");
        } else {
            print!(".");
        }
    }
}

fn add_next_step(
    pos: Vec2D,
    cost: i32,
    dir: Vec2D,
    visited_positions: &HashSet<Vec2D>,
    board: &Board<char>,
    score: i32,
    lowest_cost_to_reach: &mut HashMap<(Vec2D, Vec2D), i32>,
    queue: &mut PriorityQueue<i32, (Vec2D, Vec2D, HashSet<Vec2D>, i32)>,
    comparison: &dyn Fn(i32, i32) -> bool,
) {
    let next_pos = pos + dir;
    if *board.get(next_pos) != '#' {
        let score = score + cost;
        push_if_lowest_score(
            lowest_cost_to_reach,
            next_pos,
            dir,
            visited_positions,
            score,
            queue,
            comparison,
        );
    }
}

fn push_if_lowest_score(
    lowest_cost_to_reach: &mut HashMap<(Vec2D, Vec2D), i32>,
    pos: Vec2D,
    dir: Vec2D,
    visited_positions: &HashSet<Vec2D>,
    score: i32,
    queue: &mut PriorityQueue<i32, (Vec2D, Vec2D, HashSet<Vec2D>, i32)>,
    comparison: &dyn Fn(i32, i32) -> bool,
) {
    if let Some(prev_score) = lowest_cost_to_reach.get_mut(&(pos, dir)) {
        if comparison(score, *prev_score) {
            *prev_score = score;

            let mut cloned_positions = visited_positions.clone();
            cloned_positions.insert(pos);

            queue.push((pos, dir, cloned_positions, score), score);
        }
    } else {
        lowest_cost_to_reach.insert((pos, dir), score);

        let mut cloned_positions = visited_positions.clone();
        cloned_positions.insert(pos);

        queue.push((pos, dir, cloned_positions, score), score);
    }
}
