mod day_15;

use aoc_helper::runner::Runner;
use day_15::Impl;

fn main() {
    let input = include_str!("../files/input.txt").to_string();
    let runner = Runner::from_string(input, &Impl {});
    // let runner = Runner::from_test_file(&Impl {});
    runner.part_1();
    runner.part_2();
}
