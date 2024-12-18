use aoc_helper::Day;

#[cfg(test)]
mod test;

pub(crate) struct Impl {}

impl Day<Vec<Vec<i32>>, usize, usize> for Impl {
    fn parse(&self, input: String) -> Vec<Vec<i32>> {
        input
            .lines()
            .map(|line| {
                line.split(' ')
                    .map(|val| val.parse().expect("not a number"))
                    .collect()
            })
            .collect()
    }

    fn part_1(&self, lists: &Vec<Vec<i32>>) -> usize {
        lists
            .iter()
            .filter(|list| is_smoothly_moving(list.iter()))
            .count()
    }

    fn part_2(&self, lists: &Vec<Vec<i32>>) -> usize {
        lists.iter().filter(is_smoothly_moving_with_buffer).count()
    }
}

fn is_smoothly_moving<'a, I>(mut iter: I) -> bool
where
    I: Iterator<Item = &'a i32>,
{
    let mut all_increasing = true;
    let mut all_decreasing = true;

    let mut first = iter.next().unwrap();

    for next in iter {
        let second = next;
        let tuple = (*first, *second);

        all_increasing &= is_smoothly_increasing(tuple);
        all_decreasing &= is_smoothly_decreasing(tuple);

        if !all_increasing && !all_decreasing {
            return false;
        }
        first = second;
    }

    true
}

fn is_smoothly_moving_with_buffer(list: &&Vec<i32>) -> bool {
    (0..list.len()).any(|i| {
        is_smoothly_moving(
            list.iter()
                .enumerate()
                .filter(|(index, _)| *index != i)
                .map(|(_, val)| val),
        )
    })
}

fn is_smoothly_increasing((lhs, rhs): (i32, i32)) -> bool {
    (1..=3).contains(&(rhs - lhs))
}

fn is_smoothly_decreasing((lhs, rhs): (i32, i32)) -> bool {
    (1..=3).contains(&(lhs - rhs))
}
