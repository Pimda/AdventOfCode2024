use std::collections::{HashMap, VecDeque};

use aoc_helper::{board::Board, collections::CountCollection, navigation, vectors::Vec2D, Day};

#[cfg(test)]
mod test;

pub(crate) struct Impl {}

impl Day<Board<char>, usize, usize> for Impl {
    fn parse(&self, input: String) -> Board<char> {
        Board::new(input.lines().map(|line| line.chars().collect()).collect())
    }

    fn part_1(&self, board: &Board<char>) -> usize {
        let directions = navigation::get_adjecent_directions();

        let (start, end) = find_start_and_end(board);
        let scores = solve_maze(board, start, end, directions);

        let mut valid_cuts = 0;
        let mut cuts_score = CountCollection::new();
        for coord in board.iter_all_coordinates() {
            for cut in get_cuts_2(board, &scores, coord, 2, directions) {
                cuts_score.add(cut);
                if cut >= 100 {
                    valid_cuts += 1;
                }
            }
        }

        print_scores(cuts_score);

        valid_cuts
    }

    fn part_2(&self, board: &Board<char>) -> usize {
        let directions = navigation::get_adjecent_directions();

        let (start, end) = find_start_and_end(board);
        let scores = solve_maze(board, start, end, directions);

        let mut valid_cuts = 0;
        let mut cuts_score = CountCollection::new();
        for coord in board.iter_all_coordinates() {
            for cut in get_cuts_2(board, &scores, coord, 20, directions) {
                cuts_score.add(cut);
                if cut >= 100 {
                    valid_cuts += 1;
                }
            }
        }

        print_scores(cuts_score);

        valid_cuts
    }
}

fn print_scores(cuts_score: CountCollection<i32>) {
    for i in 1..=100 {
        let count = cuts_score.count(&i);
        if count != 0 {
            println!("{} cuts save {} steps", count, i);
        }
    }
}

fn solve_maze(
    board: &Board<char>,
    start: Vec2D,
    end: Vec2D,
    directions: [Vec2D; 4],
) -> HashMap<Vec2D, i32> {
    let mut scores = HashMap::new();
    scores.insert(start, 0);
    let mut stack = vec![(start, 0)];

    loop {
        let step = stack.pop();

        if let Some((position, score)) = step {
            if position == end {
                // Should be 84 for test input
                println!("Track length: {score}");
                break;
            }

            for direction in directions {
                let new_pos = position + direction;

                // No bounds check required as the maze is surrounded by '#'
                if *board.get(new_pos) == '#' {
                    continue;
                }

                if scores.get(&new_pos).is_some() {
                    // we should never find a lower score, so skip seen positions
                    continue;
                }

                stack.push((new_pos, score + 1));
                scores.insert(new_pos, score + 1);
            }
        }
    }
    scores
}

fn find_start_and_end(board: &Board<char>) -> (Vec2D, Vec2D) {
    let start = board
        .iter_all_coordinates()
        .find(|coord| *board.get(*coord) == 'S')
        .unwrap();
    let end = board
        .iter_all_coordinates()
        .find(|coord| *board.get(*coord) == 'E')
        .unwrap();
    (start, end)
}

fn get_cuts(
    board: &Board<char>,
    scores: &HashMap<Vec2D, i32>,
    coord: Vec2D,
    max_steps: i32,
    directions: [Vec2D; 4],
) -> Vec<i32> {
    if let Some(original_score) = scores.get(&coord) {
        let mut visited_positions = HashMap::new();
        visited_positions.insert(coord, 0);
        let mut queue = VecDeque::new();
        queue.push_back((coord, 0));
        let mut shortcuts = HashMap::new();

        loop {
            if let Some((coord, steps)) = queue.pop_front() {
                if steps >= max_steps {
                    continue;
                }

                for direction in directions {
                    let new_coord = coord + direction;
                    let new_steps = steps + 1;

                    if !board.is_in_bounds(new_coord) {
                        continue;
                    }

                    let char = board.get(new_coord);

                    if *char == '#' {
                        if let Some(old_steps) = visited_positions.get(&new_coord) {
                            if new_steps >= *old_steps {
                                continue;
                            }
                        }

                        visited_positions.insert(new_coord, new_steps);
                        queue.push_back((new_coord, new_steps));
                    } else {
                        if new_steps >= max_steps {
                            continue;
                        }

                        let score = scores.get(&new_coord).unwrap();
                        let diff = score - (original_score + new_steps);

                        if diff > 0 {
                            shortcuts
                                .entry(new_coord)
                                .and_modify(|v| {
                                    if diff > *v {
                                        *v = diff
                                    }
                                })
                                .or_insert(diff);
                        }
                    }
                }
            } else {
                return shortcuts.into_values().collect();
            }
        }
    }
    vec![]
}

fn get_cuts_2(
    board: &Board<char>,
    scores: &HashMap<Vec2D, i32>,
    coord: Vec2D,
    max_steps: i32,
    directions: [Vec2D; 4],
) -> Vec<i32> {
    if let Some(&original_score) = scores.get(&coord) {
        let mut visited_positions = HashMap::new();
        visited_positions.insert(coord, 0);
        let mut queue = VecDeque::new();
        queue.push_back((coord, 0));
        let mut shortcuts = HashMap::new();

        while let Some((current_coord, steps)) = queue.pop_front() {
            if steps >= max_steps {
                continue;
            }

            for direction in directions.iter() {
                let new_coord = current_coord + *direction;
                let new_steps = steps + 1;

                // Skip out-of-bounds
                if !board.is_in_bounds(new_coord) {
                    continue;
                }

                // Ensure no revisits with longer paths
                if visited_positions
                    .get(&new_coord)
                    .map_or(false, |&old_steps| new_steps >= old_steps)
                {
                    continue;
                }

                let char = board.get(new_coord);

                if *char == '#' {
                    // Continue walking through walls
                    visited_positions.insert(new_coord, new_steps);
                    queue.push_back((new_coord, new_steps));
                } else {
                    // Landed on a walkable space, check for a valid cut
                    if let Some(&score) = scores.get(&new_coord) {
                        let diff = score - (original_score + new_steps);
                        if diff > 0 {
                            shortcuts
                                .entry(new_coord)
                                .and_modify(|v: &mut i32| *v = (*v).max(diff))
                                .or_insert(diff);
                        }
                    }
                }
            }
        }
        return shortcuts.into_values().collect();
    }
    vec![]
}
