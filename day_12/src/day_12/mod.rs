use aoc_helper::{board::Board, collections::ContainsCollection, vectors::Vec2D, Day};

#[cfg(test)]
mod test;

pub(crate) struct Impl {}

impl Day<Board<char>, u64, u64> for Impl {
    fn parse(&self, input: String) -> Board<char> {
        Board::new(input.lines().map(|line| line.chars().collect()).collect())
    }

    fn part_1(&self, board: &Board<char>) -> u64 {
        let mut cost = 0;
        let mut visited = ContainsCollection::new();
        let directions = aoc_helper::navigation::get_adjecent_directions();

        for coord in board.iter_all_coordinates() {
            cost += get_cost(board, coord, &mut visited, directions);
        }

        cost
    }

    fn part_2(&self, board: &Board<char>) -> u64 {
        let mut cost = 0;
        let mut visited = ContainsCollection::new();
        let directions = aoc_helper::navigation::get_adjecent_directions();

        for coord in board.iter_all_coordinates() {
            cost += get_discounted_cost(board, coord, &mut visited, directions);
        }

        cost
    }
}

fn get_cost(
    board: &Board<char>,
    coord: aoc_helper::vectors::Vec2D,
    visited: &mut ContainsCollection<Vec2D>,
    directions: [Vec2D; 4],
) -> u64 {
    // The field was already checked
    if visited.contains(&coord) {
        return 0;
    }

    let char = board.get(coord);
    let mut stack = vec![coord];
    let mut sides = 0;
    let mut area = 0;

    while let Some(coord) = stack.pop() {
        if visited.contains(&coord) {
            continue;
        }

        area += 1;
        visited.add_if_not_contains(coord);

        for direction in directions {
            let new_coord = coord + direction;

            // found a border, don't remember it because it can be surrounded by more parts of this field
            if !board.is_in_bounds(new_coord) || board.get(new_coord) != char {
                sides += 1;
                continue;
            }

            if visited.contains(&new_coord) {
                continue;
            }

            stack.push(new_coord);
        }
    }

    // println!("{}, {}, {}", char, area, sides);

    area * sides
}

fn get_discounted_cost(
    board: &Board<char>,
    coord: aoc_helper::vectors::Vec2D,
    visited: &mut ContainsCollection<Vec2D>,
    directions: [Vec2D; 4],
) -> u64 {
    // The field was already checked
    if visited.contains(&coord) {
        return 0;
    }

    let char = board.get(coord);
    let mut stack = vec![coord];
    let mut area = 0;
    let mut side_pieces = vec![];

    while let Some(coord) = stack.pop() {
        if visited.contains(&coord) {
            continue;
        }

        area += 1;
        visited.add_if_not_contains(coord);

        for direction in directions {
            let new_coord = coord + direction;

            // found a border, don't remember it because it can be surrounded by more parts of this field
            if !board.is_in_bounds(new_coord) || board.get(new_coord) != char {
                side_pieces.push((coord, direction));
                continue;
            }

            if visited.contains(&new_coord) {
                continue;
            }

            stack.push(new_coord);
        }
    }

    let mut sides = 0;

    let first_selector: &dyn Fn((Vec2D, Vec2D)) -> i32 = &|(coord, _)| coord.x;
    let second_selector: &dyn Fn((Vec2D, Vec2D)) -> i32 = &|(coord, _)| coord.y;
    sides += count_edges(
        &side_pieces,
        Vec2D::new(1, 0),
        first_selector,
        second_selector,
    );
    sides += count_edges(
        &side_pieces,
        Vec2D::new(-1, 0),
        first_selector,
        second_selector,
    );
    sides += count_edges(
        &side_pieces,
        Vec2D::new(0, -1),
        second_selector,
        first_selector,
    );
    sides += count_edges(
        &side_pieces,
        Vec2D::new(0, 1),
        second_selector,
        first_selector,
    );

    sides * area

    // area * sides
}

fn count_edges(
    side_pieces: &[(Vec2D, Vec2D)],
    direction: Vec2D,
    column_selector: &dyn Fn((Vec2D, Vec2D)) -> i32,
    row_selector: &dyn Fn((Vec2D, Vec2D)) -> i32,
) -> u64 {
    let mut all: Vec<&(Vec2D, Vec2D)> = side_pieces
        .iter()
        .filter(|(_, d)| *d == direction)
        .collect();
    all.sort_by_key(|left| column_selector(**left));

    let mut buffer: Vec<&(Vec2D, Vec2D)> = vec![];
    let mut column_id = -1;
    let mut sides = 0;

    for coord in all {
        if column_selector(*coord) != column_id {
            if !buffer.is_empty() {
                sides += count_sides(buffer, row_selector);
            }

            buffer = vec![coord];
            column_id = column_selector(*coord)
        } else {
            buffer.push(coord);
        }
    }
    if !buffer.is_empty() {
        sides += count_sides(buffer, row_selector);
    }

    sides
}

fn count_sides(
    mut column: Vec<&(Vec2D, Vec2D)>,
    row_selector: &dyn Fn((Vec2D, Vec2D)) -> i32,
) -> u64 {
    column.sort_by_key(|left| row_selector(**left));

    let mut count = 0;
    let mut row_id = -2;
    for coordinate in column {
        if row_selector(*coordinate) == row_id + 1 {
            row_id += 1;
            continue;
        }
        count += 1;
        row_id = row_selector(*coordinate);
    }

    count
}
