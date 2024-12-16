use std::collections::HashMap;

use aoc_helper::{board::Board, collections::PriorityQueue, vectors::Vec2D, Day};

#[cfg(test)]
mod test;

pub(crate) struct Impl {}

impl Day<Board<char>, i32, i32> for Impl {
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

        queue.push((start_pos, start_direction, 0), 0);

        loop {
            let (pos, current_dir, score) = queue.pop_lowest().expect("No item on queue");

            if *board.get(pos) == 'E' {
                return score;
            }

            add_next_step(pos, 1, current_dir, board, score, &mut lowest_cost_to_reach, &mut queue);

            let mut cloned_dir = current_dir.clone();
            cloned_dir.rotate_left();
            add_next_step(pos, 1001, cloned_dir, board, score, &mut lowest_cost_to_reach, &mut queue);

            let mut cloned_dir = current_dir.clone();
            cloned_dir.rotate_right();
            add_next_step(pos, 1001, cloned_dir, board, score, &mut lowest_cost_to_reach, &mut queue);
        }
    }

    fn part_2(&self, board: &Board<char>) -> i32 {
        todo!()
    }
}

fn add_next_step(pos: Vec2D, cost: i32, dir: Vec2D, board: &Board<char>, score: i32, lowest_cost_to_reach: &mut HashMap<(Vec2D, Vec2D), i32>, queue: &mut PriorityQueue<i32, (Vec2D, Vec2D, i32)>) {
    let next_pos = pos + dir;
    if *board.get(next_pos) != '#' {
        let score = score + cost;
        push_if_lowest_score(lowest_cost_to_reach, next_pos, dir, score, queue);
    }
}

fn push_if_lowest_score(lowest_cost_to_reach: &mut HashMap<(Vec2D, Vec2D), i32>, pos: Vec2D, dir: Vec2D, score: i32, queue: &mut PriorityQueue<i32, (Vec2D, Vec2D, i32)>) {
    if let Some(prev_score) = lowest_cost_to_reach.get_mut(&(pos, dir)) {
        if score < *prev_score{
            *prev_score = score;

            queue.push((pos, dir, score), score);
        }
    }
    else{
        lowest_cost_to_reach.insert((pos, dir), score);
        queue.push((pos, dir, score), score);
    }
}

fn push_if_equals_lowest_score(lowest_cost_to_reach: &mut HashMap<(Vec2D, Vec2D), i32>, pos: Vec2D, dir: Vec2D, score: i32, queue: &mut PriorityQueue<i32, (Vec2D, Vec2D, i32)>) {
    if let Some(prev_score) = lowest_cost_to_reach.get_mut(&(pos, dir)) {
        if score <= *prev_score{
            *prev_score = score;

            queue.push((pos, dir, score), score);
        }
    }
    else{
        lowest_cost_to_reach.insert((pos, dir), score);
        queue.push((pos, dir, score), score);
    }
}
