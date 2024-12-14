use std::collections::HashSet;

use aoc_helper::{board::Board, vectors::Vec2D, Day};

#[cfg(test)]
mod test;

pub(crate) struct Impl {}

impl Day<Board<char>, usize, usize> for Impl {
    fn parse(&self, input: String) -> Board<char> {
        Board::new(input.lines().map(|line| line.chars().collect()).collect())
    }

    fn part_1(&self, board: &Board<char>) -> usize {
        let mut guard = find_guard(board);
        let mut visited_fields = HashSet::new();
        let mut direction = Vec2D::new(0, -1);

        loop {
            visited_fields.insert(guard);

            let next_pos = guard + direction;

            if !board.is_in_bounds(next_pos) {
                break;
            }

            if *board.get(next_pos) == '#' {
                // we need to rotate right, but since the coordinate system is flipped, rotating left should be correct
                direction.rotate_left();
            } else {
                guard = next_pos;
            }
        }

        visited_fields.len()
    }

    fn part_2(&self, board: &Board<char>) -> usize {
        let mut guard = find_guard(board);
        let mut direction = Vec2D::new(0, -1);
        let mut loops = HashSet::new();
        let mut checked_positions = HashSet::new();

        loop {
            let next_pos = guard + direction;

            if !board.is_in_bounds(next_pos) {
                break;
            }

            if *board.get(next_pos) == '#' {
                // we need to rotate right, but since the coordinate system is flipped, rotating left should be correct
                direction.rotate_left();
            } else {
                // only check a position once (also makes sure we're not testing a broken path)
                if !checked_positions.contains(&next_pos) {
                    // use the current position and direction, so we don't need to walk the whole path again
                    if is_loop(board, guard, direction, next_pos) {
                        loops.insert(next_pos);
                    }

                    checked_positions.insert(next_pos);
                }

                guard = next_pos;
            }
        }

        loops.len()
    }
}

fn find_guard(board: &Board<char>) -> Vec2D {
    board
        .iter_all_coordinates()
        .find(|pos| *board.get(*pos) == '^')
        .unwrap()
}

fn is_loop(board: &Board<char>, mut pos: Vec2D, mut dir: Vec2D, obstacle: Vec2D) -> bool {
    let mut visited = HashSet::new();

    loop {
        let next_pos = pos + dir;

        if !board.is_in_bounds(next_pos) {
            return false;
        }

        // check for wall or extra obstacle
        if *board.get(next_pos) != '#' && next_pos != obstacle {
            pos = next_pos;
        } else {
            // Only check if we reached a loop at every corner
            // this saves a lot of processing and will at most let us overlap our loop for a few steps
            if visited.contains(&(pos, dir)) {
                return true;
            }
            visited.insert((pos, dir));

            // we need to rotate right, but since the coordinate system is flipped, rotating left should be correct
            dir.rotate_left();
        }
    }
}
