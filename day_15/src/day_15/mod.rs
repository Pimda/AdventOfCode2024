use std::collections::{HashMap, HashSet};

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
        let steps_dict = get_steps_dict();
        let mut bot_pos = get_bot_pos(&board);

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
        }

        get_sum_crate_hashes(board, 'O')
    }

    fn part_2(&self, (board, steps): &(Board<char>, Vec<char>)) -> i32 {
        let mut board = widen_board(board);
        let steps_dict = get_steps_dict();
        let mut bot_pos = get_bot_pos(&board);

        for step in steps {
            let direction = steps_dict.get(step).unwrap();
            let next_pos = bot_pos + *direction;

            //check if space is free
            if *board.get(next_pos) == '.' {
                board.set(bot_pos, '.');
                bot_pos = next_pos;
                board.set(bot_pos, '@');
            } else if direction.y == 0 {
                bot_pos = move_horizontal(&mut board, bot_pos, *direction);
            } else {
                bot_pos = move_vertical(&mut board, bot_pos, *direction);
            }
        }

        get_sum_crate_hashes(board, '[')
    }
}

fn move_horizontal(board: &mut Board<char>, bot_pos: Vec2D, direction: Vec2D) -> Vec2D {
    let mut check_pos = bot_pos;

    loop {
        let char = *board.get(check_pos);

        if char == '#' {
            return bot_pos;
        }

        if char == '.' {
            loop {
                let cur_pos = check_pos;
                check_pos = check_pos - direction;
                let char = *board.get(check_pos);
                board.set(cur_pos, char);

                if char == '@' {
                    board.set(check_pos, '.');
                    return cur_pos;
                }
            }
        }

        check_pos = check_pos + direction;
    }
}

fn move_vertical(board: &mut Board<char>, bot_pos: Vec2D, direction: Vec2D) -> Vec2D {
    let mut positions_to_check = HashSet::new();
    positions_to_check.insert(bot_pos + direction);

    let mut hash_set = HashSet::new();
    hash_set.insert(bot_pos);

    let mut positions_to_move = vec![hash_set];

    loop {
        let mut new_positions_to_check = HashSet::new();
        let mut new_positions_to_move = HashSet::new();

        if positions_to_check.is_empty() {
            break;
        }

        for position in positions_to_check {
            match board.get(position) {
                '[' => {
                    new_positions_to_check.insert(position + direction);
                    new_positions_to_check.insert(position + direction + Vec2D::new(1, 0));
                    new_positions_to_move.insert(position);
                    new_positions_to_move.insert(position + Vec2D::new(1, 0));
                }
                ']' => {
                    new_positions_to_check.insert(position + direction);
                    new_positions_to_check.insert(position + direction + Vec2D::new(-1, 0));
                    new_positions_to_move.insert(position);
                    new_positions_to_move.insert(position + Vec2D::new(-1, 0));
                }
                '#' => return bot_pos,
                '.' => (), //do nothing
                _ => panic!("unknown char"),
            }
        }

        positions_to_check = new_positions_to_check;
        positions_to_move.push(new_positions_to_move);
    }

    for line in positions_to_move.iter().rev() {
        for pos in line {
            let char = board.get(*pos);
            board.set(*pos + direction, *char);
            board.set(*pos, '.')
        }
    }

    bot_pos + direction
}

fn get_sum_crate_hashes(board: Board<char>, char: char) -> i32 {
    board
        .iter_all_coordinates()
        .map(|coord| {
            if *board.get(coord) == char {
                coord.y * 100 + coord.x
            } else {
                0
            }
        })
        .sum()
}

fn get_bot_pos(board: &Board<char>) -> Vec2D {
    board
        .iter_all_coordinates()
        .find(|coord| *board.get(*coord) == '@')
        .unwrap()
}

fn get_steps_dict() -> HashMap<char, Vec2D> {
    let mut steps_dict = HashMap::new();
    steps_dict.insert('^', Vec2D::new(0, -1));
    steps_dict.insert('>', Vec2D::new(1, 0));
    steps_dict.insert('v', Vec2D::new(0, 1));
    steps_dict.insert('<', Vec2D::new(-1, 0));
    steps_dict
}

fn widen_board(board: &Board<char>) -> Board<char> {
    Board::new(
        board
            .clone()
            .get_vec()
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .flat_map(|char| match char {
                        '.' => vec!['.', '.'],
                        '#' => vec!['#', '#'],
                        '@' => vec!['@', '.'],
                        'O' => vec!['[', ']'],
                        _ => panic!("unknown char"),
                    })
                    .collect()
            })
            .collect(),
    )
}
