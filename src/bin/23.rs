advent_of_code::solution!(23);

struct Registers {
    a: u32,
    b: u32,
    c: u32,
    d: u32,
}

impl Registers {
    fn new() -> Self {
        Self {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
        }
    }

    fn get(&self, register: &str) -> u32 {
        match register {
            "a" => self.a,
            "b" => self.b,
            "c" => self.c,
            "d" => self.d,
            _ => panic!("Unknown register: {}", register),
        }
    }

    fn set(&mut self, register: &str, value: u32) {
        match register {
            "a" => self.a = value,
            "b" => self.b = value,
            "c" => self.c = value,
            "d" => self.d = value,
            _ => panic!("Unknown register: {}", register),
        }
    }
}

struct Instruction {
    instruction: String,
    arg1: String,
    arg2: String,
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let instruction = parts.next().unwrap();
            let arg1 = parts.next().unwrap();
            let arg2 = parts.next().unwrap_or("");
            Instruction {
                instruction: instruction.to_string(),
                arg1: arg1.to_string(),
                arg2: arg2.to_string(),
            }
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let instructions = parse_input(input);
    let mut registers = Registers::new();
    let mut pc = 0;

    while pc < instructions.len() {
        let instruction = &instructions[pc];
        match instruction.instruction.as_str() {
            "hlf" => {
                let value = registers.get(&instruction.arg1) / 2;
                registers.set(&instruction.arg1, value);
                pc += 1;
            }
            "tpl" => {
                let value = registers.get(&instruction.arg1) * 3;
                registers.set(&instruction.arg1, value);
                pc += 1;
            }
            "inc" => {
                let value = registers.get(&instruction.arg1) + 1;
                registers.set(&instruction.arg1, value);
                pc += 1;
            }
            "jmp" => {
                let value = instruction.arg1.parse::<i32>().unwrap();
                pc = (pc as i32 + value) as usize;
            }
            "jie" => {
                let value = registers.get(&instruction.arg1[0..1]);
                let offset = instruction.arg2.parse::<i32>().unwrap();
                if value % 2 == 0 {
                    pc = (pc as i32 + offset) as usize;
                } else {
                    pc += 1;
                }
            }
            "jio" => {
                let value = registers.get(&instruction.arg1[0..1]);
                let offset = instruction.arg2.parse::<i32>().unwrap();
                if value == 1 {
                    pc = (pc as i32 + offset) as usize;
                } else {
                    pc += 1;
                }
            }
            _ => panic!("Unknown instruction: {}", instruction.instruction),
        }
    }

    Some(registers.get("b"))
}

pub fn part_two(input: &str) -> Option<u32> {
    let instructions = parse_input(input);
    let mut registers = Registers::new();
    let mut pc = 0;

    registers.set("a", 1);

    while pc < instructions.len() {
        let instruction = &instructions[pc];
        match instruction.instruction.as_str() {
            "hlf" => {
                let value = registers.get(&instruction.arg1) / 2;
                registers.set(&instruction.arg1, value);
                pc += 1;
            }
            "tpl" => {
                let value = registers.get(&instruction.arg1) * 3;
                registers.set(&instruction.arg1, value);
                pc += 1;
            }
            "inc" => {
                let value = registers.get(&instruction.arg1) + 1;
                registers.set(&instruction.arg1, value);
                pc += 1;
            }
            "jmp" => {
                let value = instruction.arg1.parse::<i32>().unwrap();
                pc = (pc as i32 + value) as usize;
            }
            "jie" => {
                let value = registers.get(&instruction.arg1[0..1]);
                let offset = instruction.arg2.parse::<i32>().unwrap();
                if value % 2 == 0 {
                    pc = (pc as i32 + offset) as usize;
                } else {
                    pc += 1;
                }
            }
            "jio" => {
                let value = registers.get(&instruction.arg1[0..1]);
                let offset = instruction.arg2.parse::<i32>().unwrap();
                if value == 1 {
                    pc = (pc as i32 + offset) as usize;
                } else {
                    pc += 1;
                }
            }
            _ => panic!("Unknown instruction: {}", instruction.instruction),
        }
    }

    Some(registers.get("b"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
