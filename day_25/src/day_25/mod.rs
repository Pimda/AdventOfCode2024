use aoc_helper::Day;

#[cfg(test)]
mod test;

pub(crate) struct Impl {}

impl Day<(Vec<Vec<i32>>, Vec<Vec<i32>>), usize, u32> for Impl {
    fn parse(&self, input: String) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
        let mut keys = Vec::new();
        let mut locks = Vec::new();

        let mut buffer: Vec<Vec<char>> = Vec::new();

        for line in input.lines() {
            if line.is_empty() {
                parse_locks_or_keys(&mut keys, &mut locks, &buffer);

                buffer = Vec::new();
            } else {
                buffer.push(line.chars().collect());
            }
        }

        parse_locks_or_keys(&mut keys, &mut locks, &buffer);

        (locks, keys)
    }

    fn part_1(&self, (locks, keys): &(Vec<Vec<i32>>, Vec<Vec<i32>>)) -> usize {
        let mut count = 0;

        for lock in locks {
            for key in keys {
                if lock
                    .iter()
                    .zip(key.iter())
                    .all(|(lock, key)| lock + key <= 7)
                {
                    // println!("{lock:?}, {key:?} fit");

                    count += 1;
                } else {
                    // println!("{lock:?}, {key:?} overlap");
                }
            }
        }

        count
    }

    fn part_2(&self, (_, _): &(Vec<Vec<i32>>, Vec<Vec<i32>>)) -> u32 {
        0
    }
}

fn parse_locks_or_keys(
    keys: &mut Vec<Vec<i32>>,
    locks: &mut Vec<Vec<i32>>,
    buffer: &Vec<Vec<char>>,
) {
    let mut column_lengths = Vec::new();

    for x in 0..5 {
        let mut column_length = 0;
        for y in 0..7 {
            let char = buffer[y][x];
            if char == '#' {
                column_length += 1;
            }
        }
        column_lengths.push(column_length);
    }

    if buffer[0].iter().all(|char| *char == '#') {
        locks.push(column_lengths);
    } else {
        keys.push(column_lengths);
    }
}
