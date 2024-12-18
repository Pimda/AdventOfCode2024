use aoc_helper::{board::Board, vectors::Vec2D, Day};

#[cfg(test)]
mod test;

pub(crate) struct Impl {}

impl Day<Board<char>, usize, usize> for Impl {
    fn parse(&self, input: String) -> Board<char> {
        Board::new(input.lines().map(|line| line.chars().collect()).collect())
    }

    fn part_1(&self, board: &Board<char>) -> usize {
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
        board
            .iter_all_coordinates()
            .filter(|coord| *board.get(*coord) == 'X')
            .map(|coord| {
                directions
                    .into_iter()
                    .filter(|direction| is_word(board, coord, *direction, "XMAS"))
                    .count()
            })
            .sum()
    }

    fn part_2(&self, board: &Board<char>) -> usize {
        let offset_directions = [
            (Vec2D::new(1, 1), Vec2D::new(-1, -1)),
            (Vec2D::new(1, -1), Vec2D::new(-1, 1)),
            (Vec2D::new(-1, -1), Vec2D::new(1, 1)),
            (Vec2D::new(-1, 1), Vec2D::new(1, -1)),
        ];

        board
            .iter_all_coordinates()
            .filter(|coord| *board.get(*coord) == 'A')
            .filter(|coord| is_x_mas(offset_directions, board, *coord))
            .count()
    }
}

fn is_x_mas(offset_directions: [(Vec2D, Vec2D); 4], board: &Board<char>, position: Vec2D) -> bool {
    offset_directions
        .iter()
        .filter(|(offset, direction)| is_word(board, position + *offset, *direction, "MAS"))
        .count()
        == 2
}

fn is_word(board: &Board<char>, mut position: Vec2D, direction: Vec2D, string: &str) -> bool {
    for c in string.chars() {
        // Do a bounds check before retrieving the char
        if !board.is_in_bounds(position) {
            return false;
        }

        if c != *board.get(position) {
            return false;
        }

        position = position + direction;
    }
    true
}
