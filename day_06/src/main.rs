mod day_06;

use aoc_helper::runner::Runner;
use day_06::Impl;

fn main() {
    let runner = Runner::from_input_file(&Impl {});
    runner.part_1();
    runner.part_2();
}
