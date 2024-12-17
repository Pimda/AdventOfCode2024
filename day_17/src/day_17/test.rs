use super::Impl;
use aoc_helper::runner::Runner;

#[test]
fn part_1_works_for_test() {
    Runner::from_test_file(&Impl {}).assert_part_1("4,6,3,5,6,3,5,2,1,0".to_string());
}

#[test]
fn part_1_works_for_input() {
    Runner::from_input_file(&Impl {}).assert_part_1("6,1,6,4,2,4,7,3,5".to_string());
}

// #[test]
// fn part_2_works_for_test() {
//     Runner::from_file("files/test2.txt", &Impl {}).assert_part_2(117440);
// }

#[test]
fn part_2_works_for_input() {
    Runner::from_input_file(&Impl {}).assert_part_2(202975183645226);
}
