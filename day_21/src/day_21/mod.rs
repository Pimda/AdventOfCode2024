use aoc_helper::Day;

#[cfg(test)]
mod test;

pub(crate) struct Impl {}

impl Day<Vec<Vec<char>>, usize, usize> for Impl {
    fn parse(&self, input: String) -> Vec<Vec<char>> {
        input.lines().map(|line| line.chars().collect()).collect()
    }

    fn part_1(&self, board: &Vec<Vec<char>>) -> usize {
        todo!()
    }

    fn part_2(&self, board: &Vec<Vec<char>>) -> usize {
        todo!()
    }
}
