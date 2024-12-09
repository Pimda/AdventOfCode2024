use std::fmt::Display;

use crate::{day::Day, timer};

pub struct Runner<'a, I, O1, O2> {
    parsed: Option<I>,
    day: &'a dyn Day<I, O1, O2>,
}

impl<'a, I, O1, O2> Runner<'a, I, O1, O2> {
    pub fn from_input_file<D>(day: &'a D) -> Self
    where
        D: Day<I, O1, O2>,
    {
        let filename = "files/input.txt";

        if !std::path::Path::new(filename).is_file() {
            println!("Skipping test because the test file is missing");
            return Self { parsed: None, day };
        }

        Self::from_file(filename, day)
    }

    pub fn from_test_file<D>(day: &'a D) -> Self
    where
        D: Day<I, O1, O2>,
    {
        Self::from_file("files/test.txt", day)
    }

    pub fn from_file<D>(path: &str, day: &'a D) -> Self
    where
        D: Day<I, O1, O2>,
    {
        let now = timer::start_timer();

        println!("Opening input file: {}", path);

        let input = std::fs::read_to_string(path).expect("File could not be read");
        let parsed = day.parse(input);

        timer::stop_timer_and_write(now, "Read input and parse");
        println!();

        Self {
            parsed: Some(parsed),
            day,
        }
    }

    pub fn part_1(&self)
    where
        O1: Display,
    {
        if let Some(input) = &self.parsed {
            let now = timer::start_timer();
            let result = self.day.part_1(&input);
            timer::stop_timer_and_write(now, "Part 1");

            println!("{}", result);
            println!();
        }
    }

    pub fn part_2(&self)
    where
        O2: Display,
    {
        if let Some(input) = &self.parsed {
            let now = timer::start_timer();
            let result = self.day.part_2(input);
            timer::stop_timer_and_write(now, "Part 2");

            println!("{}", result);
            println!();
        }
    }

    pub fn assert_part_1(&self, expected: O1)
    where
        O1: std::fmt::Display + PartialEq + std::fmt::Debug,
    {
        if let Some(input) = &self.parsed {
            let now = timer::start_timer();
            let result = self.day.part_1(input);
            timer::stop_timer_and_write(now, "Part 1");

            assert_eq!(result, expected);
        }
    }

    pub fn assert_part_2(&self, expected: O2)
    where
        O2: std::fmt::Display + PartialEq + std::fmt::Debug,
    {
        if let Some(input) = &self.parsed {
            let now = timer::start_timer();
            let result = self.day.part_2(input);
            timer::stop_timer_and_write(now, "Part 2");

            assert_eq!(result, expected);
        }
    }
}
