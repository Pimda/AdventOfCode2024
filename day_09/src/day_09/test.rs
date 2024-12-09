use super::Impl;
use aoc_helper::runner::Runner;

#[test]
fn part_1_works_for_test() {
    Runner::from_test_file(&Impl {}).assert_part_1(1928);
}

#[test]
fn part_1_works_for_input() {
    Runner::from_input_file(&Impl {}).assert_part_1(6432869891895);
}

#[test]
fn part_2_works_for_test() {
    Runner::from_test_file(&Impl {}).assert_part_2(2858);
}

#[test]
fn part_2_works_for_input() {
    Runner::from_input_file(&Impl {}).assert_part_2(6467290479134);
}
