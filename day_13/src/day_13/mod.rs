use aoc_helper::{vectors::Vec2D, Day};

#[cfg(test)]
mod test;

pub(crate) struct Impl {}

impl Day<Vec<(Vec2D, Vec2D, Vec2D)>, i64, i64> for Impl {
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

    fn part_1(&self, puzzles: &Vec<(Vec2D, Vec2D, Vec2D)>) -> i64 {
        puzzles
            .iter()
            .map(|(button_a, button_b, target)| {
                calculate_answer(button_a, button_b, target.x.into(), target.y.into())
            })
            .map(|cost| if let Some(cost) = cost { cost } else { 0 })
            .sum()
    }

    fn part_2(&self, puzzles: &Vec<(Vec2D, Vec2D, Vec2D)>) -> i64 {
        puzzles
            .iter()
            .map(|(button_a, button_b, target)| {
                calculate_answer(
                    button_a,
                    button_b,
                    target.x as i64 + 10_000_000_000_000i64,
                    target.y as i64 + 10_000_000_000_000i64,
                )
            })
            .map(|cost| if let Some(cost) = cost { cost } else { 0 })
            .sum()
    }
}

fn _seek_answer((button_a, button_b, target): (Vec2D, Vec2D, Vec2D)) -> Option<i32> {
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

fn calculate_answer(
    button_a: &Vec2D,
    button_b: &Vec2D,
    target_x: i64,
    target_y: i64,
) -> Option<i64> {
    let button_a_x: i64 = button_a.x.try_into().unwrap();
    let button_a_y: i64 = button_a.y.try_into().unwrap();
    let button_b_x: i64 = button_b.x.try_into().unwrap();
    let button_b_y: i64 = button_b.y.try_into().unwrap();

    let (a1, b1, c1) = vector_to_linear_equation(button_a_x, button_a_y, 0, 0);
    let (a2, b2, c2) = vector_to_linear_equation(button_b_x, button_b_y, target_x, target_y);

    // Calculate the determinant of the system
    let det = a1 * b2 - a2 * b1;

    if det == 0 {
        // Lines are parallel
        return None;
    }

    // Using Cramer's Rule to find x and y
    let x = (b1 * c2 - b2 * c1) / det;
    let y = (a2 * c1 - a1 * c2) / det;

    // intersection out of range
    if x < 0 || y < 0 || x > target_x || y > target_y {
        return None;
    }

    let count_a = x / button_a_x;
    let count_b = (target_x - x) / button_b_x;

    if target_x != count_a * button_a_x + count_b * button_b_x
        || target_y != count_a * button_a_y + count_b * button_b_y
    {
        return None;
    }

    Some(count_a * 3 + count_b)
}

fn vector_to_linear_equation(
    vector_x: i64,
    vector_y: i64,
    target_x: i64,
    target_y: i64,
) -> (i64, i64, i64) {
    // Calculate the coefficients of the equation a1 * x + b1 * y + c1 = 0
    let a1 = vector_y;
    let b1 = -vector_x;
    let c1 = -(a1 * target_x + b1 * target_y);

    (a1, b1, c1)
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
