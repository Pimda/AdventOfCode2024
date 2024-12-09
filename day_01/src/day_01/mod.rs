use aoc_helper::{collections::CountCollection, Day};

#[cfg(test)]
mod test;

pub(crate) struct Impl {}

impl Day<(Vec<i32>, Vec<i32>), i32, i32> for Impl {
    fn parse(&self, input: String) -> (Vec<i32>, Vec<i32>) {
        input
            .lines()
            .map(|line| {
                if let Some((left, right)) = line.split_once("   ") {
                    let lhs: i32 = left.parse().unwrap();
                    let rhs: i32 = right.parse().unwrap();
                    (lhs, rhs)
                } else {
                    panic!("invalid input")
                }
            })
            .unzip()
    }

    fn part_1(&self, (vec1, vec2): &(Vec<i32>, Vec<i32>)) -> i32 {
        let mut vec1 = vec1.clone();
        let mut vec2 = vec2.clone();
        vec1.sort_unstable();
        vec2.sort_unstable();

        vec1.iter().zip(vec2).map(distance).sum()
    }

    fn part_2(&self, (numbers, counts): &(Vec<i32>, Vec<i32>)) -> i32 {
        let counts = CountCollection::from_vec(counts);
        numbers
            .iter()
            .map(|number| score(*number, counts.count(number).try_into().unwrap()))
            .sum()
    }
}

fn distance(pair: (&i32, i32)) -> i32 {
    (pair.0 - pair.1).abs()
}

fn score(key: i32, count: i32) -> i32 {
    key * count
}
