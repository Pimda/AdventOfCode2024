use std::collections::{HashMap, HashSet, VecDeque};

use aoc_helper::Day;

#[cfg(test)]
mod test;

pub(crate) struct Impl {}

impl Day<Vec<i64>, i64, i64> for Impl {
    fn parse(&self, input: String) -> Vec<i64> {
        input.lines().map(|line| line.parse().unwrap()).collect()
    }

    fn part_1(&self, nums: &Vec<i64>) -> i64 {
        nums.iter()
            .map(|num| {
                let mut num = *num;
                for _ in 0..2000 {
                    num = hash(num);
                }
                num
            })
            .sum()
    }

    fn part_2(&self, nums: &Vec<i64>) -> i64 {
        let mut scores = HashMap::new();
        let mut best_score = 0;

        for num in nums {
            let mut unique_windows = HashSet::new();
            let mut num = *num;
            let mut prev_price = price(num);
            let mut window = Window::new();

            for _ in 1..2000 {
                num = hash(num);
                let price = price(num);
                let diff = price - prev_price;
                window.push(diff);

                if window.is_filled() {
                    let key = window.get_key();
                    if !unique_windows.contains(&key) {
                        unique_windows.insert(key);
                        scores
                            .entry(key)
                            .and_modify(|score| {
                                *score += price;
                                if *score > best_score {
                                    best_score = *score;
                                }
                            })
                            .or_insert(price);
                    }
                }

                prev_price = price;
            }
        }

        best_score
    }
}

fn hash(mut secret: i64) -> i64 {
    secret = prune(mix(secret, secret * 64));
    secret = prune(mix(secret, secret / 32));
    secret = prune(mix(secret, secret * 2048));

    secret
}

fn mix(num1: i64, num2: i64) -> i64 {
    num1 ^ num2
}

fn prune(num: i64) -> i64 {
    num % 16777216
}

fn price(num: i64) -> i64 {
    num % 10
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Window {
    items: VecDeque<i64>,
}

impl Window {
    fn new() -> Self {
        Self {
            items: VecDeque::new(),
        }
    }

    fn push(&mut self, item: i64) {
        self.items.push_back(item);
        if self.items.len() > 4 {
            _ = self.items.pop_front();
        }
    }

    fn is_filled(&self) -> bool {
        self.items.len() == 4
    }

    fn get_key(&self) -> i64 {
        (self.items[0] + 9) * 6859
            + (self.items[1] + 9) * 361
            + (self.items[2] + 9) * 19
            + (self.items[3] + 9)
    }
}
