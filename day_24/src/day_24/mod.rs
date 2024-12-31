use aoc_helper::Day;

#[cfg(test)]
mod test;

pub(crate) struct Impl {}

impl Day<(Vec<Value>, Vec<Gate>), u64, usize> for Impl {
    fn parse(&self, input: String) -> (Vec<Value>, Vec<Gate>) {
        let mut lines_iter = input.lines();
        let mut inputs = Vec::new();
        let mut gates = Vec::new();

        for line in &mut lines_iter {
            if line.is_empty() {
                break;
            }

            inputs.push(Value::from_string(line));
        }

        for line in &mut lines_iter {
            gates.push(Gate::from_string(line));
        }

        (inputs, gates)
    }

    fn part_1(&self, (inputs, gates): &(Vec<Value>, Vec<Gate>)) -> u64 {
        let mut gates = gates.to_vec();
        let mut inputs = inputs.to_vec();

        let mut non_complete_gates: Vec<&mut Gate> = gates.iter_mut().collect();

        while !non_complete_gates.is_empty() {
            // apply all inputs
            for input in inputs.iter() {
                for gate in non_complete_gates.iter_mut() {
                    if gate.uses_input(&input) {
                        gate.apply_input(&input);
                    }
                }
            }

            for gate in non_complete_gates {
                if gate.has_value() {
                    inputs.push(Value {
                        name: gate.output.to_string(),
                        value: gate.get_output(),
                    });
                }
            }

            non_complete_gates = gates.iter_mut().filter(|gate| !gate.has_value()).collect();
        }

        inputs.sort_by_key(|input| input.name.to_string());

        let mut binary = 0u64;
        let mut multiplier = 1;
        for input in inputs.iter().filter(|input| input.name.starts_with('z')) {
            let val_u64: u64 = input.value.into();
            binary += val_u64 * multiplier;
            multiplier *= 2;
        }

        binary
    }

    fn part_2(&self, board: &(Vec<Value>, Vec<Gate>)) -> usize {
        todo!()
    }
}

#[derive(Clone)]
pub struct Value {
    name: String,
    value: u8,
}
impl Value {
    fn from_string(string: &str) -> Self {
        let split = string.split_once(": ").unwrap();
        Self {
            name: split.0.to_string(),
            value: split.1.parse().unwrap(),
        }
    }
}

#[derive(Clone)]
pub struct Gate {
    input_1: String,
    input_2: String,
    output: String,
    op: String,

    input_1_val: Option<u8>,
    input_2_val: Option<u8>,
}

impl Gate {
    fn from_string(string: &str) -> Self {
        if let Some((gate, out)) = string.split_once(" -> ") {
            let split: Vec<&str> = gate.split(' ').collect();

            if let [in_1, op, in_2] = split[..] {
                return Self {
                    input_1: in_1.to_string(),
                    input_2: in_2.to_string(),
                    output: out.to_string(),
                    op: op.to_string(),
                    input_1_val: None,
                    input_2_val: None,
                };
            }
        }

        panic!("Gate invalid")
    }

    fn uses_input(&self, input: &Value) -> bool {
        *input.name == self.input_1 || *input.name == self.input_2
    }

    fn apply_input(&mut self, input: &Value) {
        if input.name == self.input_1 {
            self.input_1_val = Some(input.value);
        }
        if input.name == self.input_2 {
            self.input_2_val = Some(input.value);
        }
    }

    fn has_value(&self) -> bool {
        self.input_1_val.is_some() && self.input_2_val.is_some()
    }

    fn get_output(&self) -> u8 {
        let val_1 = self.input_1_val.unwrap();
        let val_2 = self.input_2_val.unwrap();

        match self.op.as_str() {
            "AND" => val_1 & val_2,
            "XOR" => val_1 ^ val_2,
            "OR" => val_1 | val_2,
            _ => panic!(),
        }
    }
}
