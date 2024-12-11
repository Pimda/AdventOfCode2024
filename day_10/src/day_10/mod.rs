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
        let directions = aoc_helper::navigation::get_adjecent_directions();

        board
            .iter_all_coordinates()
            .filter(|coordinate| *board.get(*coordinate) == 0)
            .map(|coordinate| {
                count_trails_unique_nines(
                    board,
                    coordinate,
                    1,
                    &mut ContainsCollection::new(),
                    &directions,
                )
            })
            .sum()
    }

    fn part_2(&self, board: &Board<u32>) -> u32 {
        let directions = aoc_helper::navigation::get_adjecent_directions();

        board
            .iter_all_coordinates()
            .filter(|coordinate| *board.get(*coordinate) == 0)
            .map(|coordinate| count_all_trails(board, coordinate, 1, &directions))
            .sum::<u32>()
    }
}

fn count_trails_unique_nines(
    board: &Board<u32>,
    coordinate: aoc_helper::vectors::Vec2D,
    height: u32,
    found_nines: &mut ContainsCollection<Vec2D>,
    directions: &[Vec2D; 4],
) -> u32 {
    let mut count = 0;

    if height == 10 {
        if found_nines.contains(&coordinate) {
            return 0;
        }
        found_nines.add_if_not_contains(coordinate);
        return 1;
    }

    for dir in directions {
        let new_pos = coordinate + *dir;

        if board.is_in_bounds(new_pos) && *board.get(new_pos) == height {
            count += count_trails_unique_nines(board, new_pos, height + 1, found_nines, directions)
        }
    }

    count
}

fn count_all_trails(
    board: &Board<u32>,
    coordinate: aoc_helper::vectors::Vec2D,
    height: u32,
    directions: &[Vec2D; 4],
) -> u32 {
    let mut count = 0;

    if height == 10 {
        return 1;
    }

    for dir in directions {
        let new_pos = coordinate + *dir;

        if board.is_in_bounds(new_pos) && *board.get(new_pos) == height {
            count += count_all_trails(board, new_pos, height + 1, directions)
        }
    }

    count
}
