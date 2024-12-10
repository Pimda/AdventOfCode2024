use aoc_helper::{board::Board, collections::ContainsCollection, vectors::Vec2D, Day};

#[cfg(test)]
mod test;

pub(crate) struct Impl {}

impl Day<Board<u32>, u32, u32> for Impl {
    fn parse(&self, input: String) -> Board<u32> {
        Board::new(
            input
                .lines()
                .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
                .collect(),
        )
    }

    fn part_1(&self, board: &Board<u32>) -> u32 {
        let mut trail_count = 0;

        for y in 0..board.get_bounds().y {
            for x in 0..board.get_bounds().x {
                let mut found_nines = ContainsCollection::new();
                let coordinate = Vec2D::new(x, y);

                if *board.get(coordinate) == 0 {
                    trail_count += count_trails(board, coordinate, 1, &mut found_nines);
                }
            }
        }

        trail_count
    }

    fn part_2(&self, board: &Board<u32>) -> u32 {
        let mut trail_count = 0;

        for y in 0..board.get_bounds().y {
            for x in 0..board.get_bounds().x {
                let coordinate = Vec2D::new(x, y);

                if *board.get(coordinate) == 0 {
                    trail_count += count_trails_2(board, coordinate, 1);
                }
            }
        }

        trail_count
    }
}

fn count_trails(
    board: &Board<u32>,
    coordinate: aoc_helper::vectors::Vec2D,
    height: u32,
    found_nines: &mut ContainsCollection<Vec2D>,
) -> u32 {
    let directions = vec![
        Vec2D::new(0, 1),
        Vec2D::new(1, 0),
        Vec2D::new(0, -1),
        Vec2D::new(-1, 0),
    ];
    let mut count = 0;

    if height == 10 {
        if found_nines.contains(&coordinate) {
            return 0;
        }
        found_nines.add_if_not_contains(coordinate);
        return 1;
    }

    for dir in directions {
        let new_pos = coordinate + dir;

        if board.is_in_bounds(new_pos) && *board.get(new_pos) == height {
            count += count_trails(board, new_pos, height + 1, found_nines)
        }
    }

    count
}

fn count_trails_2(
    board: &Board<u32>,
    coordinate: aoc_helper::vectors::Vec2D,
    height: u32,
) -> u32 {
    let directions = vec![
        Vec2D::new(0, 1),
        Vec2D::new(1, 0),
        Vec2D::new(0, -1),
        Vec2D::new(-1, 0),
    ];
    let mut count = 0;

    if height == 10 {
        return 1;
    }

    for dir in directions {
        let new_pos = coordinate + dir;

        if board.is_in_bounds(new_pos) && *board.get(new_pos) == height {
            count += count_trails_2(board, new_pos, height + 1)
        }
    }

    count
}
