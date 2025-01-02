mod day_23;

use aoc_helper::runner::Runner;
use day_23::Impl;

fn main() {
    let input = include_str!("../files/input.txt").to_string();
    let runner = Runner::from_string(input, &Impl {});
    runner.part_1();
    runner.part_2();
}