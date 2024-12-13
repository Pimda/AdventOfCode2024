use aoc_helper::{vectors::Vec2D, Day};

#[cfg(test)]
mod test;

pub(crate) struct Impl {}

impl Day<Vec<(Vec2D, Vec2D, Vec2D)>, i32, u64> for Impl {
    fn parse(&self, input: String) -> Vec<(Vec2D, Vec2D, Vec2D)> {
        let mut lines = input.lines();
        let mut puzzles = vec![];

        loop {
            puzzles.push((
                parse_button(lines.next().unwrap(), "Button A".to_string()),
                parse_button(lines.next().unwrap(), "Button B".to_string()),
                parse_prize(lines.next().unwrap()),
            ));

            if lines.next().is_none() {
                break;
            }
        }

        puzzles
    }

    fn part_1(&self, puzzles: &Vec<(Vec2D, Vec2D, Vec2D)>) -> i32 {
        puzzles
            .iter()
            .map(|puzzle| calculate_1(*puzzle))
            .map(|cost| if let Some(cost) = cost { cost } else { 0 })
            .sum()
    }

    fn part_2(&self, board: &Vec<(Vec2D, Vec2D, Vec2D)>) -> u64 {
        todo![]
    }
}

fn calculate_1((button_a, button_b, target): (Vec2D, Vec2D, Vec2D)) -> Option<i32> {
    let mut lowest_cost = None;

    for count_a in 0..100 {
        for count_b in 0..100 {
            if button_a * count_a + button_b * count_b == target {
                let cost = count_a * 3 + count_b;

                if let Some(lowest) = lowest_cost {
                    if cost < lowest {
                        lowest_cost = Some(cost)
                    }
                } else {
                    lowest_cost = Some(cost)
                }
            }
        }
    }

    lowest_cost
}

fn parse_button(input: &str, button_name: String) -> Vec2D {
    parse_input(input, &(button_name + ": X+"), "Y+")
}

fn parse_prize(input: &str) -> Vec2D {
    parse_input(input, "Prize: X=", "Y=")
}

fn parse_input(input: &str, str_1: &str, str_2: &str) -> Vec2D {
    let (left, right) = input.trim_start_matches(str_1).split_once(", ").unwrap();
    let (left, right) = (
        left.parse().unwrap(),
        right.trim_start_matches(str_2).parse().unwrap(),
    );

    Vec2D::new(left, right)
}
