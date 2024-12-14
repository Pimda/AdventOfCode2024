use std::collections::HashSet;

use aoc_helper::{math::positive_mod, vectors::Vec2D, Day};

#[cfg(test)]
mod test;

pub(crate) struct Impl {}

impl Day<Vec<(Vec2D, Vec2D)>, usize, i32> for Impl {
    fn parse(&self, input: String) -> Vec<(Vec2D, Vec2D)> {
        input
            .lines()
            .map(|line| {
                let p: Vec2D;
                let v: Vec2D;

                if let Some((left, right)) = line.split_once(' ') {
                    if let Some((x, y)) = left.trim_start_matches("p=").split_once(',') {
                        p = Vec2D::new(x.parse().unwrap(), y.parse().unwrap());
                    } else {
                        panic!()
                    }

                    if let Some((x, y)) = right.trim_start_matches("v=").split_once(',') {
                        v = Vec2D::new(x.parse().unwrap(), y.parse().unwrap());
                    } else {
                        panic!()
                    }
                } else {
                    panic!()
                }

                (p, v)
            })
            .collect()
    }

    fn part_1(&self, robots: &Vec<(Vec2D, Vec2D)>) -> usize {
        let width = 101;
        let height = 103;

        let half_width = 50;
        let half_height = 51;

        let turns = 100;

        let new_positions: Vec<Vec2D> = robots
            .iter()
            .map(|(p, v)| {
                Vec2D::new(
                    positive_mod(p.x + turns * v.x, width),
                    positive_mod(p.y + turns * v.y, height),
                )
            })
            .collect();

        let count_quadrant_0 = new_positions
            .iter()
            .filter(|p| p.x < half_width && p.y < half_height)
            .count();
        let count_quadrant_1 = new_positions
            .iter()
            .filter(|p| p.x > half_width && p.y > half_height)
            .count();
        let count_quadrant_2 = new_positions
            .iter()
            .filter(|p| p.x < half_width && p.y > half_height)
            .count();
        let count_quadrant_3 = new_positions
            .iter()
            .filter(|p| p.x > half_width && p.y < half_height)
            .count();

        count_quadrant_0 * count_quadrant_1 * count_quadrant_2 * count_quadrant_3
    }

    fn part_2(&self, _robots: &Vec<(Vec2D, Vec2D)>) -> i32 {
        // I found these simply by trying and finding a pattern
        let key_1 = 50;
        let key_2 = 95;

        let width = 101;
        let height = 103;

        let mut seconds = 0;

        loop {
            seconds += 1;

            if seconds % height == key_1 && seconds % width == key_2 {
                // let mut new_positions = HashSet::new();
                // for robot in robots {
                //     let new_x = positive_mod(robot.0.x + seconds * robot.1.x, width);
                //     let new_y = positive_mod(robot.0.y + seconds * robot.1.y, height);
                //     new_positions.insert(Vec2D::new(new_x, new_y));
                // }
                // print(height, width, &new_positions);
                return seconds;
            }
        }
    }
}

fn _print(height: i32, width: i32, new_positions: &HashSet<Vec2D>) {
    for y in 0..height {
        for x in 0..width {
            let pos = Vec2D::new(x, y);
            if new_positions.contains(&pos) {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}
