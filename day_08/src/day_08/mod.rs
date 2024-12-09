use aoc_helper::{board::Board, collections::ContainsCollection, vectors::Vec2D, Day};

#[cfg(test)]
mod test;

pub(crate) struct Impl {}

impl Day<Board<char>, usize, usize> for Impl {
    fn parse(&self, input: String) -> Board<char> {
        Board::new(input.lines().map(|line| line.chars().collect()).collect())
    }

    fn part_1(&self, board: &Board<char>) -> usize {
        let mut nodes = ContainsCollection::new();

        for y in 0..board.get_bounds().y {
            for x in 0..board.get_bounds().x {
                let index = Vec2D::new(x, y);
                let char = *board.get(index);
                if char != '.' {
                    let new_nodes = find_anti_nodes(board, index, char);

                    for node in new_nodes {
                        nodes.add_if_not_contains(node);
                    }
                }
            }
        }

        nodes.len()
    }

    fn part_2(&self, board: &Board<char>) -> usize {
        let mut nodes = ContainsCollection::new();

        for y in 0..board.get_bounds().y {
            for x in 0..board.get_bounds().x {
                let index = Vec2D::new(x, y);
                let char = *board.get(index);
                if char != '.' {
                    let new_nodes = find_coninuous_anti_nodes(board, index, char);

                    for node in new_nodes {
                        nodes.add_if_not_contains(node);
                    }
                }
            }
        }

        nodes.len()
    }
}

fn _print_board(board: &Board<char>, nodes: &ContainsCollection<Vec2D>) {
    for y in 0..board.get_bounds().y {
        for x in 0..board.get_bounds().x {
            let index = Vec2D::new(x, y);

            if nodes.contains(&index) {
                print!("#");
            } else {
                print!("{}", board.get(index))
            }
        }
        println!()
    }
}

fn find_anti_nodes(board: &Board<char>, index: Vec2D, char: char) -> Vec<Vec2D> {
    let mut nodes = vec![];

    for y in 0..board.get_bounds().y {
        for x in 0..board.get_bounds().x {
            let check_index = Vec2D::new(x, y);
            if check_index != index && *board.get(check_index) == char {
                let offset = check_index - index;
                let node = check_index + offset;

                if board.is_in_bounds(node) {
                    nodes.push(node);
                }
            }
        }
    }

    nodes
}

fn find_coninuous_anti_nodes(board: &Board<char>, index: Vec2D, char: char) -> Vec<Vec2D> {
    let mut nodes = vec![];

    for y in 0..board.get_bounds().y {
        for x in 0..board.get_bounds().x {
            let check_index = Vec2D::new(x, y);
            if check_index != index && *board.get(check_index) == char {
                let offset = check_index - index;
                let mut node = check_index;

                while board.is_in_bounds(node) {
                    nodes.push(node);
                    node = node + offset;
                }
            }
        }
    }

    nodes
}
