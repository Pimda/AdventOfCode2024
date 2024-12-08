use aoc_helper::Day;

#[cfg(test)]
mod test;

pub(crate) struct Impl {}

impl Day<String, u32, u32> for Impl {
    fn parse(&self, input: String) -> String {
        input
    }

    fn part_1(&self, text: &String) -> u32 {
        parse(text, State::new(false))
    }

    fn part_2(&self, text: &String) -> u32 {
        parse(text, State::new(true))
    }
}

fn parse(text: &str, mut state: State) -> u32 {
    for char in text.chars() {
        match (
            state.buffer.as_str(),
            &mut state.num1,
            &mut state.num2,
            char,
        ) {
            ("", None, None, 'm')
            | ("m", None, None, 'u')
            | ("mu", None, None, 'l')
            | ("mul", None, None, '(')
            | ("mul(", Some(_), None, ',') => state.buffer.push(char),
            // buffer num1
            ("mul(", num1, None, char) if char.is_ascii_digit() => {
                add_digit(num1, char.to_digit(10).unwrap())
            }
            // buffer num2
            ("mul(,", Some(_), num2, digit) if digit.is_ascii_digit() => {
                add_digit(num2, digit.to_digit(10).unwrap())
            }
            // add to total and reset
            ("mul(,", Some(num1), Some(num2), ')') => {
                let score = *num1 * *num2;

                state.add_total(score);
                state.reset()
            }
            ("", None, None, 'd')
            | ("d", None, None, 'o')
            | ("do", None, None, 'n')
            | ("don", None, None, '\'')
            | ("don'", None, None, 't')
            | ("don't", None, None, '(')
            | ("do", None, None, '(') => {
                state.buffer.push(char);
            }
            // disable counting
            ("don't(", None, None, ')') => {
                state.enabled = false;
                state.reset();
            }
            // enable counting
            ("do(", None, None, ')') => {
                state.enabled = true;
                state.reset();
            }
            _ => state.reset(),
        }
    }

    state.total
}

fn add_digit(num: &mut Option<u32>, char: u32) {
    if let Some(v) = num {
        *num = Some(*v * 10 + char)
    } else {
        *num = Some(char)
    }
}

struct State {
    buffer: String,
    num1: Option<u32>,
    num2: Option<u32>,
    total: u32,
    enabled: bool,
    check_enabled: bool,
}

impl State {
    fn new(check_enabled: bool) -> Self {
        Self {
            buffer: "".to_string(),
            num1: None,
            num2: None,
            total: 0,
            enabled: true,
            check_enabled,
        }
    }

    fn reset(&mut self) {
        self.buffer = "".to_string();
        self.num1 = None;
        self.num2 = None;
    }

    fn add_total(&mut self, value: u32) {
        if !self.check_enabled || self.enabled {
            self.total += value;
        }
    }
}
