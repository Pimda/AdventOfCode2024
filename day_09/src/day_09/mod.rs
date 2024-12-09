use aoc_helper::Day;

#[cfg(test)]
mod test;

pub(crate) struct Impl {}

impl Day<Vec<u32>, usize, usize> for Impl {
    fn parse(&self, input: String) -> Vec<u32> {
        input.chars().map(|c| c.to_digit(10).unwrap()).collect()
    }

    fn part_1(&self, files: &Vec<u32>) -> usize {
        let mut disk = vec![];
        let mut id = 0;

        // fill disk
        for window in files.chunks(2) {
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
            id += 1;
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

    fn part_2(&self, files: &Vec<u32>) -> usize {
        let mut files = vec![];
        let mut empty_space = vec![];
        let mut id = 0;

        // fill disk
        for window in files.chunks(2) {
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
            id += 1;
        }

        todo!()
    }
}

struct File {
    id: usize,
    size: u32,
    index: usize,
}

struct FreeSpace {
    index: usize,
    size: u32,
}
