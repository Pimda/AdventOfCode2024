use aoc_helper::Day;

#[cfg(test)]
mod test;

pub(crate) struct Impl {}

impl Day<Vec<u64>, usize, u64> for Impl {
    fn parse(&self, input: String) -> Vec<u64> {
        input
            .chars()
            .map(|c| c.to_digit(10).unwrap().into())
            .collect()
    }

    fn part_1(&self, files: &Vec<u64>) -> usize {
        let mut disk = vec![];

        // fill disk
        for (id, window) in files.chunks(2).enumerate() {
            if let [file, space] = window {
                for _ in 0..*file {
                    disk.push(Some(id));
                }
                for _ in 0..*space {
                    disk.push(None);
                }
            } else if let [file] = window {
                for _ in 0..*file {
                    disk.push(Some(id));
                }
            }
        }

        let mut rev_iter = disk.iter().enumerate().rev();
        let mut checksum: usize = 0;
        let mut rev_id = usize::MAX;
        let mut rev_file_id: &Option<usize>;

        for (index, f) in disk.iter().enumerate() {
            if index == rev_id {
                return checksum;
            }

            if let Some(file_id) = f {
                checksum += index * file_id;
            } else {
                (rev_id, rev_file_id) = rev_iter.next().unwrap();

                while rev_file_id.is_none() {
                    if rev_id == index {
                        return checksum;
                    }

                    (rev_id, rev_file_id) = rev_iter.next().unwrap();
                }

                checksum += index * rev_file_id.unwrap();
            }
        }

        checksum
    }

    fn part_2(&self, map: &Vec<u64>) -> u64 {
        let mut files = vec![];
        let mut empty_space = vec![];
        let mut index = 0;

        // fill disk
        for (id, window) in map.chunks(2).enumerate() {
            if let [size, space] = window {
                files.push(File {
                    id: id.try_into().unwrap(),
                    index,
                    size: *size,
                });
                index += size;

                empty_space.push(FreeSpace {
                    index,
                    size: *space,
                });
                index += space;
            } else if let [size] = window {
                files.push(File {
                    id: id.try_into().unwrap(),
                    index,
                    size: *size,
                });
                index += size;
            }
        }

        // move files
        for file in files.iter_mut().rev() {
            for space in &mut empty_space {
                if space.index >= file.index {
                    break;
                }

                if space.size >= file.size {
                    file.index = space.index;
                    space.size -= file.size;
                    space.index += file.size;
                }
            }
        }

        files
            .iter()
            .map(|file| {
                (0..file.size)
                    .map(|i| (i + file.index) * file.id)
                    .sum::<u64>()
            })
            .sum()
    }
}

struct File {
    id: u64,
    index: u64,
    size: u64,
}

struct FreeSpace {
    index: u64,
    size: u64,
}
