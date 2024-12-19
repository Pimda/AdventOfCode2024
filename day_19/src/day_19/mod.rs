use std::string::ToString;

use aoc_helper::{collections::MemoizerCollection, Day};

#[cfg(test)]
mod test;

pub(crate) struct Impl {}

impl Day<(Vec<String>, Vec<String>), usize, usize> for Impl {
    fn parse(&self, input: String) -> (Vec<String>, Vec<String>) {
        let mut lines = input.lines();

        let parts = lines
            .next()
            .unwrap()
            .split(", ")
            .map(ToString::to_string)
            .collect();

        lines.next();
        (parts, lines.map(ToString::to_string).collect())
    }

    fn part_1(&self, (parts, lines): &(Vec<String>, Vec<String>)) -> usize {
        lines
            .iter()
            .filter(|line| can_match(line, line.len(), 0, parts))
            .count()
    }

    fn part_2(&self, (parts, lines): &(Vec<String>, Vec<String>)) -> usize {
        lines
            .iter()
            .map(|line| {
                let mut memoizer = MemoizerCollection::new();
                count_matches(line, line.len(), 0, parts, &mut memoizer)
            })
            .sum()
    }
}

fn can_match(line: &String, line_len: usize, matched_len: usize, parts: &Vec<String>) -> bool {
    if line_len == matched_len {
        return true;
    }

    parts.iter().any(|part| {
        let line_chars = line.chars().skip(matched_len);
        let part_chars = part.chars();

        if iters_match(line_chars, part_chars) {
            can_match(line, line_len, matched_len + part.len(), parts)
        } else {
            false
        }
    })
}

fn count_matches(
    line: &String,
    line_len: usize,
    matched_len: usize,
    parts: &[String],
    memoizer: &mut MemoizerCollection<usize, usize>,
) -> usize {
    if line_len == matched_len {
        return 1;
    }

    if let Some(count) = memoizer.get(&matched_len) {
        return *count;
    }

    let count = parts
        .iter()
        .map(|part| {
            let line_chars = line.chars().skip(matched_len);
            let part_chars = part.chars();

            if iters_match(line_chars, part_chars) {
                count_matches(line, line_len, matched_len + part.len(), parts, memoizer)
            } else {
                0
            }
        })
        .sum();

    memoizer.add_and_return(matched_len, count)
}

fn iters_match(
    mut line_chars: std::iter::Skip<std::str::Chars<'_>>,
    mut part_chars: std::str::Chars<'_>,
) -> bool {
    loop {
        let line_char = line_chars.next();
        let part_char = part_chars.next();

        match (line_char, part_char) {
            (Some(line_char), Some(part_char)) if line_char != part_char => {
                return false;
            }
            (None | Some(_), None) => {
                return true;
            }
            (None, Some(_)) => {
                return false;
            }
            _ => {}
        }
    }
}
