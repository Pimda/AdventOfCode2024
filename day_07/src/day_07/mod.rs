use aoc_helper::Day;

#[cfg(test)]
mod test;

pub(crate) struct Impl {}

impl Day<Vec<(u64, Vec<u64>)>, u64, u64> for Impl {
    fn parse(&self, input: String) -> Vec<(u64, Vec<u64>)> {
        input
            .lines()
            .map(|line| {
                if let [result, rest] = line.split(": ").collect::<Vec<&str>>()[..] {
                    (
                        result.parse().expect("not a number"),
                        rest.split(" ")
                            .map(|num| num.parse().expect("not a number"))
                            .collect(),
                    )
                } else {
                    panic!()
                }
            })
            .collect()
    }

    fn part_1(&self, lines: &Vec<(u64, Vec<u64>)>) -> u64 {
        lines
            .iter()
            .filter(|line| is_valid_1(line.0, 0, 0, &line.1))
            .map(|v| v.0)
            .sum()
    }

    fn part_2(&self, lines: &Vec<(u64, Vec<u64>)>) -> u64 {
        lines
            .iter()
            .filter(|line| is_valid_2(line.0, 0, 0, &line.1))
            .map(|v| v.0)
            .sum()
    }
}

fn is_valid_1(result: u64, accumulator: u64, index: usize, numbers: &[u64]) -> bool {
    if index == numbers.len() || accumulator > result {
        return accumulator == result;
    }

    let num = numbers.iter().nth(index).unwrap().to_owned();

    if index == 0 {
        is_valid_1(result, num, index + 1, numbers)
    } else {
        is_valid_1(result, num + accumulator, index + 1, numbers)
            || is_valid_1(result, num * accumulator, index + 1, numbers)
    }
}

fn is_valid_2(result: u64, accumulator: u64, index: usize, numbers: &[u64]) -> bool {
    if index == numbers.len() || accumulator > result {
        return accumulator == result;
    }

    let num = numbers.iter().nth(index).unwrap().to_owned();

    if index == 0 {
        is_valid_2(result, num, index + 1, numbers)
    } else {
        is_valid_2(result, num + accumulator, index + 1, numbers)
            || is_valid_2(result, num * accumulator, index + 1, numbers)
            || is_valid_2(
                result,
                format!("{}{}", accumulator, num).parse().unwrap(),
                index + 1,
                numbers,
            )
    }
}
