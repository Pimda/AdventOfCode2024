use super::Impl;
use aoc_helper::runner::Runner;

#[test]
fn part_1_works_for_test() {
    Runner::from_test_file(&Impl {}).assert_part_1(10092);
}

#[test]
fn part_1_works_for_small_test() {
    Runner::from_file("files/small.txt", &Impl {}).assert_part_1(2028);
}

// #[test]
// fn part_1_works_for_input() {
//     Runner::from_input_file(&Impl {}).assert_part_1(0);
// }

// #[test]
// fn part_2_works_for_test() {
//     Runner::from_test_file(&Impl {}).assert_part_2(0);
// }

// #[test]
// fn part_2_works_for_input() {
//     Runner::from_input_file(&Impl {}).assert_part_2(0);
// }
