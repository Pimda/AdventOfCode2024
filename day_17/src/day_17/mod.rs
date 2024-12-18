use aoc_helper::Day;

#[cfg(test)]
mod test;

pub(crate) struct Impl {}

impl Day<(u64, u64, u64, Vec<u64>), String, u64> for Impl {
    fn parse(&self, input: String) -> (u64, u64, u64, Vec<u64>) {
        let mut lines = input.lines();

        let reg_a = lines
            .next()
            .unwrap()
            .trim_start_matches("Register A: ")
            .parse()
            .unwrap();

        let reg_b = lines
            .next()
            .unwrap()
            .trim_start_matches("Register B: ")
            .parse()
            .unwrap();

        let reg_c = lines
            .next()
            .unwrap()
            .trim_start_matches("Register C: ")
            .parse()
            .unwrap();

        lines.next();

        let commands = lines
            .next()
            .unwrap()
            .trim_start_matches("Program: ")
            .split(',')
            .map(|num| num.parse().unwrap())
            .collect();

        (reg_a, reg_b, reg_c, commands)
    }

    fn part_1(&self, (reg_a, reg_b, reg_c, commands): &(u64, u64, u64, Vec<u64>)) -> String {
        to_string(&execute(*reg_a, *reg_b, *reg_c, commands, 0))
    }

    fn part_2(&self, (_reg_a, reg_b, reg_c, commands): &(u64, u64, u64, Vec<u64>)) -> u64 {
        //target 2,4,1,1,7,5,0,3,1,4,4,4,5,5,3,0

        //too high, but correct ending ;P 12990411753294847

        // So what I did is iterate per trillions first, only matching the last fourish numbers
        // Then looking both ways i tried looking with lower steps if I could find more and more numbers
        // Until in the end I found this value (and some much larger ones, since counting is hard ;P)

        // let mut i: u64 = 202975183645226;

        // Following algorithm somewhat looks like what I did manually
        // Parameters were set to optimize for speed
        // This is unlikely to find all answers for all versions of the input
        let mut step = 7_000_000_000_000u64;
        let mut skip = 14;

        let mut i = 0;

        let target = to_string(commands);

        loop {
            let result = execute(i, *reg_b, *reg_c, commands, 0);
            let part: Vec<u64> = commands.iter().map(|v| v.to_owned()).skip(skip).collect();

            if to_string(&result) == target {
                return i;
            }

            if result.len() == 16 && result.ends_with(&part[..]) {
                skip -= 1;
                i -= step;
                step /= 10;
                step = step.max(1);
            }

            i += step;
        }
    }
}

fn execute(
    mut reg_a: u64,
    mut reg_b: u64,
    mut reg_c: u64,
    commands: &[u64],
    mut inst_pointer: usize,
) -> Vec<u64> {
    let mut output = vec![];

    loop {
        if inst_pointer >= commands.len() {
            break;
        }

        let opcode = commands[inst_pointer];
        let combo = commands[inst_pointer + 1];

        perform_opcode(
            opcode,
            combo,
            &mut inst_pointer,
            &mut reg_a,
            &mut reg_b,
            &mut reg_c,
            &mut output,
        );
    }

    output
}

fn to_string(output: &Vec<u64>) -> String {
    format!("{output:?}")
        .replace(" ", "")
        .replace("[", "")
        .replace("]", "")
}

fn combo_operand(op: u64, reg_a: &u64, reg_b: &u64, reg_c: &u64) -> u64 {
    match op {
        0..=3 => op,
        4 => *reg_a,
        5 => *reg_b,
        6 => *reg_c,
        7 => panic!("RESERVED"),
        _ => panic!("INVALID OPCODE"),
    }
}

fn perform_opcode(
    opcode: u64,
    operand: u64,
    inst_pointer: &mut usize,
    reg_a: &mut u64,
    reg_b: &mut u64,
    reg_c: &mut u64,
    output: &mut Vec<u64>,
) {
    match opcode {
        0 => {
            *reg_a /= 2u64.pow(
                combo_operand(operand, reg_a, reg_b, reg_c)
                    .try_into()
                    .unwrap(),
            )
        }
        1 => *reg_b ^= operand,
        2 => *reg_b = combo_operand(operand, reg_a, reg_b, reg_c) % 8,
        3 => {
            if *reg_a != 0 {
                let operand_u: usize = operand.try_into().unwrap();
                *inst_pointer = operand_u;
                return;
            }
        }
        4 => *reg_b ^= *reg_c,
        5 => output.push(combo_operand(operand, reg_a, reg_b, reg_c) % 8),
        6 => {
            *reg_b = *reg_a
                / 2u64.pow(
                    combo_operand(operand, reg_a, reg_b, reg_c)
                        .try_into()
                        .unwrap(),
                )
        }
        7 => {
            *reg_c = *reg_a
                / 2u64.pow(
                    combo_operand(operand, reg_a, reg_b, reg_c)
                        .try_into()
                        .unwrap(),
                )
        }
        _ => panic!("Invalid opcode"),
    }

    *inst_pointer += 2;
}
