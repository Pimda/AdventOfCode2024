use std::collections::HashMap;

use aoc_helper::{board::Board, vectors::Vec2D, Day};

#[cfg(test)]
mod test;

pub(crate) struct Impl {}

impl Day<(Board<char>, Vec<char>), i32, i32> for Impl {
    fn parse(&self, input: String) -> (Board<char>, Vec<char>) {
        let mut lines_iter = input.lines();
        let mut board = vec![];

        for line in &mut lines_iter {
            if line.is_empty() {
                break;
            }
            board.push(line.chars().collect());
        }

        let steps = lines_iter.flat_map(|line| line.chars()).collect();
        (Board::new(board), steps)
    }

    fn part_1(&self, (board, steps): &(Board<char>, Vec<char>)) -> i32 {
        let mut board = board.clone();

        let mut steps_dict = HashMap::new();
        steps_dict.insert('^', Vec2D::new(0, -1));
        steps_dict.insert('>', Vec2D::new(1, 0));
        steps_dict.insert('v', Vec2D::new(0, 1));
        steps_dict.insert('<', Vec2D::new(-1, 0));

        let mut bot_pos = board
            .iter_all_coordinates()
            .find(|coord| *board.get(*coord) == '@')
            .unwrap();

        for step in steps {
            let direction = steps_dict.get(step).unwrap();
            let next_pos = bot_pos + *direction;

            //check if space is free
            if *board.get(next_pos) == '.' {
                board.set(bot_pos, '.');
                bot_pos = next_pos;
                board.set(bot_pos, '@');
                continue;
            }

            let mut check_pos = next_pos;

            //otherwise push crates
            loop {
                let char = *board.get(check_pos);

                if char == '#' {
                    break;
                }

                if char == '.' {
                    board.set(check_pos, 'O');
                    board.set(next_pos, '.');
                    board.set(bot_pos, '.');
                    bot_pos = next_pos;
                    board.set(bot_pos, '@');
                    break;
                }

                check_pos = check_pos + *direction;
            }

            // board.print();
        }

        board
            .iter_all_coordinates()
            .map(|coord| {
                if *board.get(coord) == 'O' {
                    coord.y * 100 + coord.x
                } else {
                    0
                }
            })
            .sum()
    }

    fn part_2(&self, _robots: &(Board<char>, Vec<char>)) -> i32 {
        todo!()
    }
}
