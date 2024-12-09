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
            .filter(|line| {
                // load the first num before going into recursion
                let num = get_num(&line.1, 0);
                is_valid_1(line.0, num, 1, &line.1)
            })
            .map(|v| v.0)
            .sum()
    }

    fn part_2(&self, lines: &Vec<(u64, Vec<u64>)>) -> u64 {
        lines
            .iter()
            .filter(|line| {
                // load the first num before going into recursion
                let num = get_num(&line.1, 0);
                is_valid_2(line.0, num, 1, &line.1)
            })
            .map(|v| v.0)
            .sum()
    }
}

fn is_valid_1(result: u64, accumulator: u64, index: usize, numbers: &[u64]) -> bool {
    if index == numbers.len() || accumulator > result {
        return accumulator == result;
    }

    let num = get_num(numbers, index);

    is_valid_1(result, num + accumulator, index + 1, numbers)
        || is_valid_1(result, num * accumulator, index + 1, numbers)
}

fn is_valid_2(result: u64, accumulator: u64, index: usize, numbers: &[u64]) -> bool {
    if index == numbers.len() || accumulator > result {
        return accumulator == result;
    }

    let num = get_num(numbers, index);

    is_valid_2(result, num + accumulator, index + 1, numbers)
        || is_valid_2(result, num * accumulator, index + 1, numbers)
        || is_valid_2(
            result,
            concat_numbers(accumulator, num),
            index + 1,
            numbers,
        )
}

// required because somehow adding two nums as strings and parsing it again is very slow
fn concat_numbers(accumulator: u64, num: u64) -> u64 {
    let mut mul = 10;
    
    while mul <= num{
        mul *= 10;
    }

    accumulator * mul + num
}

fn get_num(numbers: &[u64], index: usize) -> u64 {
    numbers.iter().nth(index).unwrap().to_owned()
}
