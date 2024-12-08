mod day_03;

use aoc_helper::runner::Runner;
use day_03::Impl;

fn main() {
    let runner = Runner::from_input_file(&Impl {});
    runner.part_1();
    runner.part_2();
}
