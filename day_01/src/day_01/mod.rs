use std::collections::HashMap;

use aoc_helper::Day;

#[cfg(test)]
mod test;

pub(crate) struct Impl {}

impl Day<[Vec<i32>; 2], i32, i32> for Impl {
    fn parse(&self, input: String) -> [Vec<i32>; 2] {
        let vec1 = input
            .lines()
            .map(|line| line.split("   ").nth(0).unwrap().parse().unwrap())
            .collect();
        let vec2 = input
            .lines()
            .map(|line| line.split("   ").nth(1).unwrap().parse().unwrap())
            .collect();

        [vec1, vec2]
    }

    fn part_1(&self, lists: &[Vec<i32>; 2]) -> i32 {
        let [vec1, vec2] = lists;

        let mut vec1 = vec1.to_vec();
        let mut vec2 = vec2.to_vec();
        vec1.sort();
        vec2.sort();

        vec1.iter().zip(vec2).map(dist).sum()
    }

    fn part_2(&self, lists: &[Vec<i32>; 2]) -> i32 {
        let [vec1, vec2] = lists;

        let mut test: HashMap<i32, i32> = vec1.iter().map(|key| (key.to_owned(), 0)).collect();

        for key2 in vec2 {
            if let Some(count_ref) = test.get_mut(key2) {
                *count_ref += 1;
            }
        }

        vec1.iter()
            .map(|key| score(*key, *test.get(key).unwrap()))
            .sum()
    }
}

fn dist(pair: (&i32, i32)) -> i32 {
    (pair.0 - pair.1).abs()
}

fn score(key: i32, count: i32) -> i32 {
    key * count
}
