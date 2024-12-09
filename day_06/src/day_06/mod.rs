use aoc_helper::{board::Board, collections::ContainsCollection, vectors::Vec2D, Day};

#[cfg(test)]
mod test;

pub(crate) struct Impl {}

impl Day<Board<char>, usize, usize> for Impl {
    fn parse(&self, input: String) -> Board<char> {
        Board::new(input.lines().map(|line| line.chars().collect()).collect())
    }

    fn part_1(&self, board: &Board<char>) -> usize {
        let mut guard = find_guard(board);
        let mut visited_fields = ContainsCollection::new();
        let mut direction = Vec2D::new(0, -1);

        loop {
            visited_fields.add_if_not_contains(guard);

            let next_pos = guard + direction;

            if !board.is_in_bounds(next_pos){
                break;
            }

            if *board.get(next_pos) != '#' {
                guard = next_pos;
            } else {
                // we need to rotate right, but since the coordinate system is flipped, rotating left should be correct
                direction.rotate_left();
            }
        }

        visited_fields.len()
    }

    fn part_2(&self, board: &Board<char>) -> usize {
        let mut guard = find_guard(board);
        let mut direction = Vec2D::new(0, -1);
        let mut loops = ContainsCollection::new();

        let start_pos = guard;
        let start_dir = direction;

        loop {
            let next_pos = guard + direction;

            if !board.is_in_bounds(next_pos){
                break;
            }

            if *board.get(next_pos) != '#' {
                guard = next_pos;

                if is_loop(board, start_pos, start_dir, next_pos) {
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

fn find_guard(board: &Board<char>) -> Vec2D {
    for y in 0..board.get_bounds().y {
        for x in 0..board.get_bounds().x {
            let pos = Vec2D::new(x, y);
            if *board.get(pos) == '^' {
                return pos;
            }
        }
    }

    panic!("Guard not found")
}

fn is_loop(
    board: &Board<char>,
    mut pos: Vec2D,
    mut dir: Vec2D,
    obstacle: Vec2D,
) -> bool {
    let mut visited = ContainsCollection::new();

    while !visited.contains(&(pos, dir)) {
        visited.add_if_not_contains((pos, dir));

        let next_pos = pos + dir;

        if !board.is_in_bounds(next_pos){
            return false;
        }

        // check for wall or extra obstacle
        if *board.get(next_pos) != '#' && next_pos != obstacle {
            pos = next_pos;
        } else {
            // we need to rotate right, but since the coordinate system is flipped, rotating left should be correct
            dir.rotate_left();
        }
    }

    true
}
