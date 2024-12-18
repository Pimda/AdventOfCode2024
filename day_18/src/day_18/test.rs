use super::Impl;
use aoc_helper::{runner::Runner, vectors::Vec2D};

#[test]
fn part_1_works_for_test() {
    Runner::from_test_file(&Impl {}).assert_part_1(22);
}

#[test]
fn part_1_works_for_input() {
    Runner::from_input_file(&Impl {}).assert_part_1(284);
}

#[test]
fn part_2_works_for_test() {
    Runner::from_test_file(&Impl {}).assert_part_2(Vec2D::new(6, 1));
}

#[test]
fn part_2_works_for_input() {
    Runner::from_input_file(&Impl {}).assert_part_2(Vec2D::new(51, 50));
}
