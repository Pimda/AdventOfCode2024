use aoc_helper::Day;

#[cfg(test)]
mod test;

pub(crate) struct Impl {}

impl Day<Vec<Vec<i32>>, usize, usize> for Impl {
    fn parse(&self, input: String) -> Vec<Vec<i32>> {
        input
            .lines()
            .map(|line| {
                line.split(" ")
                    .map(|val| val.parse().expect("not a number"))
                    .collect()
            })
            .collect()
    }

    fn part_1(&self, lists: &Vec<Vec<i32>>) -> usize {
        lists.iter().filter(is_smoothly_moving).count()
    }

    fn part_2(&self, lists: &Vec<Vec<i32>>) -> usize {
        lists.iter().filter(is_smoothly_moving_with_buffer).count()
    }
}

fn is_smoothly_moving(list: &&Vec<i32>) -> bool {
    list.windows(2)
        .map(two_array_to_tuple)
        .all(is_smoothly_increasing)
        || list
            .windows(2)
            .map(two_array_to_tuple)
            .all(is_smoothly_decreasing)
}

fn is_smoothly_moving_with_buffer(list: &&Vec<i32>) -> bool {
    for i in 0..list.len() {
        let option = list
            .iter()
            .enumerate()
            .filter(|(index, _)| *index != i)
            .map(|(_, val)| *val)
            .collect();

        if is_smoothly_moving(&&option) {
            return true;
        }
    }

    false
}

fn two_array_to_tuple(pair: &[i32]) -> (i32, i32) {
    if let [lhs, rhs] = pair {
        (*lhs, *rhs)
    } else {
        panic!("Supplied array is not of length 2")
    }
}

fn is_smoothly_increasing((lhs, rhs): (i32, i32)) -> bool {
    (1..=3).contains(&(rhs - lhs))
}

fn is_smoothly_decreasing((lhs, rhs): (i32, i32)) -> bool {
    (1..=3).contains(&(lhs - rhs))
}
