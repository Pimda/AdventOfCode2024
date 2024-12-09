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
        let mut nodes = ContainsCollection::new();

        for y in 0..bounds.y {
            for x in 0..bounds.x {
                let index = Vec2D::new(x, y);
                let char = get_char(board, index);
                if char != '.' {
                    let new_nodes = find_anti_nodes(board, bounds, index, char);

                    for node in new_nodes {
                        nodes.add_if_not_contains(node);
                    }
                }
            }
        }

        nodes.len()
    }

    fn part_2(&self, board: &Vec<Vec<char>>) -> usize {
        let bounds = get_bounds(board);
        let mut nodes = ContainsCollection::new();

        for y in 0..bounds.y {
            for x in 0..bounds.x {
                let index = Vec2D::new(x, y);
                let char = get_char(board, index);
                if char != '.' {
                    let new_nodes = find_coninuous_anti_nodes(board, bounds, index, char);

                    for node in new_nodes {
                        nodes.add_if_not_contains(node);
                    }
                }
            }
        }

        nodes.len()
    }
}

fn _print_board(bounds: Vec2D, nodes: &ContainsCollection<Vec2D>, board: &[Vec<char>]) {
    for y in 0..bounds.y {
        for x in 0..bounds.x {
            let index = Vec2D::new(x, y);

            if nodes.contains(&index) {
                print!("#");
            } else {
                print!("{}", get_char(board, index))
            }
        }
        println!()
    }
}

fn find_anti_nodes(board: &[Vec<char>], bounds: Vec2D, index: Vec2D, char: char) -> Vec<Vec2D> {
    let mut nodes = vec![];

    for y in 0..bounds.y {
        for x in 0..bounds.x {
            let check_index = Vec2D::new(x, y);
            if check_index != index && get_char(board, check_index) == char {
                let offset = check_index - index;
                let node = check_index + offset;

                if node.is_in_bounds(bounds) {
                    nodes.push(node);
                }
            }
        }
    }

    nodes
}

fn find_coninuous_anti_nodes(
    board: &[Vec<char>],
    bounds: Vec2D,
    index: Vec2D,
    char: char,
) -> Vec<Vec2D> {
    let mut nodes = vec![];

    for y in 0..bounds.y {
        for x in 0..bounds.x {
            let check_index = Vec2D::new(x, y);
            if check_index != index && get_char(board, check_index) == char {
                let offset = check_index - index;
                let mut node = check_index;

                while node.is_in_bounds(bounds) {
                    nodes.push(node);
                    node = node + offset;
                }
            }
        }
    }

    nodes
}

fn get_bounds(board: &[Vec<char>]) -> Vec2D {
    let width = board.first().unwrap().len().try_into().unwrap();
    let height = board.len().try_into().unwrap();

    Vec2D::new(width, height)
}

fn get_char(board: &[Vec<char>], index: Vec2D) -> char {
    let index_u = index.to_uvec2d_or_throw();
    board
        .get(index_u.y)
        .unwrap()
        .get(index_u.x)
        .unwrap()
        .to_owned()
}
