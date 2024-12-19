use std::{collections::HashMap, iter::Skip, str, string::ToString};

use aoc_helper::{collections::MemoizerCollection, Day};

#[cfg(test)]
mod test;

pub(crate) struct Impl {}

impl Day<(HashMap<char, Vec<String>>, Vec<String>), usize, usize> for Impl {
    fn parse(&self, input: String) -> (HashMap<char, Vec<String>>, Vec<String>) {
        let mut lines = input.lines();

        let parts: Vec<String> = lines
            .next()
            .unwrap()
            .split(", ")
            .map(ToString::to_string)
            .collect();

        let mut map: HashMap<char, Vec<String>> = HashMap::new();

        for part in parts {
            let char = part.chars().next().unwrap();

            if let Some(vec) = map.get_mut(&char) {
                vec.push(part);
            } else {
                map.insert(char, vec![part]);
            }
        }

        lines.next();
        (map, lines.map(ToString::to_string).collect())
    }

    fn part_1(&self, (parts, lines): &(HashMap<char, Vec<String>>, Vec<String>)) -> usize {
        lines
            .iter()
            .filter(|line| can_match(line, 0, parts))
            .count()
    }

    fn part_2(&self, (parts, lines): &(HashMap<char, Vec<String>>, Vec<String>)) -> usize {
        lines
            .iter()
            .map(|line| {
                let mut memoizer = MemoizerCollection::new();
                count_matches(line, 0, parts, &mut memoizer)
            })
            .sum()
    }
}

fn can_match(line: &String, matched_len: usize, parts: &HashMap<char, Vec<String>>) -> bool {
    if line.len() == matched_len {
        return true;
    }

    let char = line.chars().nth(matched_len).unwrap();

    if let Some(possible_parts) = parts.get(&char) {
        possible_parts.iter().any(|part| {
            let line_chars = line.chars().skip(matched_len);
            let part_chars = part.chars();

            if iters_match(line_chars, part_chars) {
                can_match(line, matched_len + part.len(), parts)
            } else {
                false
            }
        })
    } else {
        false
    }
}

fn count_matches(
    line: &String,
    matched_len: usize,
    parts: &HashMap<char, Vec<String>>,
    memoizer: &mut MemoizerCollection<usize, usize>,
) -> usize {
    if line.len() == matched_len {
        return 1;
    }

    if let Some(count) = memoizer.get(&matched_len) {
        return *count;
    }

    let char = line.chars().nth(matched_len).unwrap();
    if let Some(possible_parts) = parts.get(&char) {
        let count = possible_parts
            .iter()
            .map(|part| {
                let line_chars = line.chars().skip(matched_len);
                let part_chars = part.chars();

                if iters_match(line_chars, part_chars) {
                    count_matches(line, matched_len + part.len(), parts, memoizer)
                } else {
                    0
                }
            })
            .sum();

        memoizer.add_and_return(matched_len, count)
    } else {
        0
    }
}

fn iters_match(mut line_chars: Skip<str::Chars<'_>>, mut part_chars: str::Chars<'_>) -> bool {
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
