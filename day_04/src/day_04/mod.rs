use aoc_helper::{vectors::Vec2D, Day};

#[cfg(test)]
mod test;

pub(crate) struct Impl {}

impl Day<Vec<Vec<char>>, usize, usize> for Impl {
    fn parse(&self, input: String) -> Vec<Vec<char>> {
        input.lines().map(|line| line.chars().collect()).collect()
    }

    fn part_1(&self, board: &Vec<Vec<char>>) -> usize {
        let bounds = get_puzzle_size(board);

        let directions = [
            Vec2D::new(0, 1),
            Vec2D::new(1, 1),
            Vec2D::new(1, 0),
            Vec2D::new(1, -1),
            Vec2D::new(0, -1),
            Vec2D::new(-1, -1),
            Vec2D::new(-1, 0),
            Vec2D::new(-1, 1),
        ];

        // Sum all XMAS counts of all rows
        (0..bounds.y)
            .map(|y| {
                // Count all instances of XMAS and sum them per row
                (0..bounds.x)
                    .map(|x| {
                        let position = Vec2D::new(x, y);

                        directions
                            .into_iter()
                            .filter(|direction| {
                                is_word(board, position, *direction, bounds, "XMAS")
                            })
                            .count()
                    })
                    .sum::<usize>()
            })
            .sum()
    }

    fn part_2(&self, board: &Vec<Vec<char>>) -> usize {
        let bounds = get_puzzle_size(board);

        let offset_directions = [
            (Vec2D::new(1, 1), Vec2D::new(-1, -1)),
            (Vec2D::new(1, -1), Vec2D::new(-1, 1)),
            (Vec2D::new(-1, -1), Vec2D::new(1, 1)),
            (Vec2D::new(-1, 1), Vec2D::new(1, -1)),
        ];

        // Sum the X-MAS counts of all rows
        (0..bounds.y)
            .map(|y| {
                // Filter row for all positions containing an X-MAS and count them
                (0..bounds.x)
                    .filter(|x| is_x_mas(offset_directions, board, Vec2D::new(*x, y), bounds))
                    .count()
            })
            .sum()
    }
}

fn is_x_mas(
    offset_directions: [(Vec2D, Vec2D); 4],
    board: &[Vec<char>],
    position: Vec2D,
    bounds: Vec2D,
) -> bool {
    offset_directions
        .iter()
        .filter(|(offset, direction)| {
            is_word(board, position + *offset, *direction, bounds, "MAS")
        })
        .count()
        == 2
}

fn is_word(
    board: &[Vec<char>],
    mut position: Vec2D,
    direction: Vec2D,
    bounds: Vec2D,
    string: &str,
) -> bool {
    for c in string.chars() {
        // Do a bounds check before retrieving the char
        if !position.is_in_bounds(bounds) {
            return false;
        }

        if c != get_char_from_board(board, position) {
            return false;
        }

        position = position + direction;
    }
    true
}

fn get_char_from_board(board: &[Vec<char>], pos: Vec2D) -> char {
    *board
        .get::<usize>(pos.y.try_into().unwrap())
        .expect("Row not found")
        .get::<usize>(pos.x.try_into().unwrap())
        .expect("Somehow there is not a char at x, y")
}

fn get_puzzle_size(puzzle: &[Vec<char>]) -> Vec2D {
    let height = puzzle.len().try_into().unwrap();
    let width = puzzle
        .first()
        .expect("first line has not characters")
        .len()
        .try_into()
        .unwrap();
    Vec2D::new(width, height)
}
