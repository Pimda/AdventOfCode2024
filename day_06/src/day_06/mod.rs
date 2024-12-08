use aoc_helper::{collections::ContainsCollection, vectors::Vec2D, Day};

#[cfg(test)]
mod test;

pub(crate) struct Impl {}

impl Day<Vec<Vec<char>>, usize, usize> for Impl {
    fn parse(&self, input: String) -> Vec<Vec<char>> {
        input.lines().map(|line| line.chars().collect()).collect()
    }

    fn part_1(&self, board: &Vec<Vec<char>>) -> usize {
        let bounds = get_bounds(board);
        let mut guard = find_guard(board, bounds);
        let mut visited_fields = ContainsCollection::new();
        let mut direction = Vec2D::new(0, -1);

        loop {
            visited_fields.add_if_not_contains(guard);

            let next_pos = guard + direction;

            if !next_pos.is_in_bounds(bounds) {
                break;
            }

            if get_char(board, next_pos) != '#' {
                guard = next_pos;
            } else {
                // we need to rotate right, but since the coordinate system is flipped, rotating left should be correct
                direction.rotate_left();
            }
        }

        visited_fields.len()
    }

    fn part_2(&self, board: &Vec<Vec<char>>) -> usize {
        let bounds = get_bounds(board);
        let mut guard = find_guard(board, bounds);
        let mut direction = Vec2D::new(0, -1);
        let mut loops = ContainsCollection::new();

        let start_pos = guard;
        let start_dir = direction;

        loop {
            let next_pos = guard + direction;

            if !next_pos.is_in_bounds(bounds) {
                break;
            }

            if get_char(board, next_pos) != '#' {
                guard = next_pos;

                if is_loop(board, start_pos, start_dir, bounds, next_pos) {
                    loops.add_if_not_contains(next_pos);
                }
            } else {
                // we need to rotate right, but since the coordinate system is flipped, rotating left should be correct
                direction.rotate_left();
            }
        }

        loops.len()
    }
}

fn find_guard(board: &[Vec<char>], bounds: Vec2D) -> Vec2D {
    for y in 0..bounds.y {
        for x in 0..bounds.x {
            let pos = Vec2D::new(x, y);
            if get_char(board, pos) == '^' {
                return pos;
            }
        }
    }

    panic!("Guard not found")
}

fn get_char(board: &[Vec<char>], pos: Vec2D) -> char {
    *board
        .iter()
        .nth(pos.y.try_into().unwrap())
        .unwrap()
        .iter()
        .nth(pos.x.try_into().unwrap())
        .unwrap()
}

fn get_bounds(board: &[Vec<char>]) -> Vec2D {
    let width = board.first().unwrap().len();
    let height = board.len();

    Vec2D::new(width.try_into().unwrap(), height.try_into().unwrap())
}

fn is_loop(
    board: &[Vec<char>],
    mut pos: Vec2D,
    mut dir: Vec2D,
    bounds: Vec2D,
    obstacle: Vec2D,
) -> bool {
    let mut visited = ContainsCollection::new();

    while !visited.contains(&(pos, dir)) {
        visited.add_if_not_contains((pos, dir));

        let next_pos = pos + dir;

        if !next_pos.is_in_bounds(bounds) {
            return false;
        }

        // check for wall or extra obstacle
        if get_char(board, next_pos) != '#' && next_pos != obstacle {
            pos = next_pos;
        } else {
            // we need to rotate right, but since the coordinate system is flipped, rotating left should be correct
            dir.rotate_left();
        }
    }

    true
}
