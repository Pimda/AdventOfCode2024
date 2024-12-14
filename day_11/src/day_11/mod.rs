use aoc_helper::{collections::MemoizerCollection, Day};

#[cfg(test)]
mod test;

pub(crate) struct Impl {}

impl Day<Vec<u64>, usize, usize> for Impl {
    fn parse(&self, input: String) -> Vec<u64> {
        input.split(' ').map(|num| num.parse().unwrap()).collect()
    }

    fn part_1(&self, nums: &Vec<u64>) -> usize {
        let mut memoizer = MemoizerCollection::new();
        nums.iter().map(|n| blink(*n, 25, &mut memoizer)).sum()
    }

    fn part_2(&self, nums: &Vec<u64>) -> usize {
        let mut memoizer = MemoizerCollection::new();
        nums.iter().map(|n| blink(*n, 75, &mut memoizer)).sum()
    }
}

fn blink(node: u64, times: u32, memoizer: &mut MemoizerCollection<(u64, u32), usize>) -> usize {
    if let Some(count) = memoizer.get((node, times)) {
        return *count;
    }

    // base case
    if times == 0 {
        return 1;
    }

    // 0 becomes 1
    if node == 0 {
        let count = blink(1, times - 1, memoizer);
        return memoizer.add_and_return((node, times), count);
    }

    // even digits => split at center
    if let Some((left, right)) = split_if_even_digits(node) {
        let count = blink(left, times - 1, memoizer) + blink(right, times - 1, memoizer);
        return memoizer.add_and_return((node, times), count);
    }

    // default => multiply by 2024
    let count = blink(node * 2024, times - 1, memoizer);
    memoizer.add_and_return((node, times), count)
}

fn split_if_even_digits(num: u64) -> Option<(u64, u64)> {
    let mut check = 1;
    let mut div = 1;

    loop {
        // uneven amount of digits
        check *= 10;
        if num < check {
            return None;
        }

        // even amount of digits
        check *= 10;
        div *= 10;
        if num < check {
            return Some((num / div, num % div));
        }
    }
}
