use std::collections::HashMap;

advent_of_code::solution!(7);

enum Input {
    Value(u16),
    Wire(String),
}

enum Gate {
    And(Input, Input),
    Or(Input, Input),
    LShift(Input, u16),
    RShift(Input, u16),
    Not(Input),
}

enum Signal {
    Input(Input),
    Gate(Gate),
}

struct Circuit {
    wires: HashMap<String, Signal>,
}

impl Circuit {
    fn get_signal(&self, input: &Input, cache: &mut HashMap<String, u16>) -> u16 {
        match input {
            Input::Value(value) => *value,
            Input::Wire(wire) => self.get_wire(wire, cache),
        }
    }

    fn get_wire(&self, wire: &str, cache: &mut HashMap<String, u16>) -> u16 {
        if let Some(value) = cache.get(wire) {
            return *value;
        }

        let value = match self.wires.get(wire).unwrap() {
            Signal::Input(input) => self.get_signal(input, cache),
            Signal::Gate(gate) => self.get_gate(gate, cache),
        };

        cache.insert(wire.to_string(), value);
        value
    }

    fn get_gate(&self, gate: &Gate, cache: &mut HashMap<String, u16>) -> u16 {
        match gate {
            Gate::And(input, other) => {
                self.get_signal(input, cache) & self.get_signal(other, cache)
            }
            Gate::Or(input, other) => self.get_signal(input, cache) | self.get_signal(other, cache),
            Gate::LShift(input, other) => self.get_signal(input, cache) << other,
            Gate::RShift(input, other) => self.get_signal(input, cache) >> other,
            Gate::Not(input) => !self.get_signal(input, cache),
        }
    }
}

fn parse_input(input: &str) -> Input {
    match input.parse::<u16>() {
        Ok(value) => Input::Value(value),
        Err(_) => Input::Wire(input.to_string()),
    }
}

fn parse_line(line: &str) -> (String, Signal) {
    let mut parts = line.split(" -> ");
    let input = parts.next().unwrap();
    let wire = parts.next().unwrap();

    let signal = match input.split(' ').collect::<Vec<_>>()[..] {
        [input] => Signal::Input(parse_input(input)),
        [input, "AND", other] => Signal::Gate(Gate::And(parse_input(input), parse_input(other))),
        [input, "OR", other] => Signal::Gate(Gate::Or(parse_input(input), parse_input(other))),
        [input, "LSHIFT", other] => {
            Signal::Gate(Gate::LShift(parse_input(input), other.parse().unwrap()))
        }
        [input, "RSHIFT", other] => {
            Signal::Gate(Gate::RShift(parse_input(input), other.parse().unwrap()))
        }
        ["NOT", input] => Signal::Gate(Gate::Not(parse_input(input))),
        _ => panic!("Invalid input: {}", input),
    };

    (wire.to_string(), signal)
}

fn run_signals(input: &str, wire: &str) -> Option<u16> {
    let mut circuit = Circuit {
        wires: HashMap::new(),
    };

    let mut cache: HashMap<String, u16> = HashMap::new();

    for line in input.lines() {
        let (wire, signal) = parse_line(line);
        circuit.wires.insert(wire, signal);
    }

    Some(circuit.get_wire(wire, &mut cache))
}

pub fn part_one(input: &str) -> Option<u16> {
    run_signals(input, "a")
}

pub fn part_two(input: &str) -> Option<u16> {
    let part_one = part_one(input).unwrap();

    let mut circuit = Circuit {
        wires: HashMap::new(),
    };

    let mut cache: HashMap<String, u16> = HashMap::new();

    for line in input.lines() {
        let (wire, signal) = parse_line(line);
        circuit.wires.insert(wire, signal);
    }

    circuit.wires.insert(
        "b".to_string(),
        Signal::Input(Input::Value(part_one as u16)),
    );

    Some(circuit.get_wire("a", &mut cache))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_running_signals() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(run_signals(&input, "h"), Some(65412));
    }
}
