pub enum Command {
    TurnOn,
    TurnOff,
    Toggle,
}

pub struct Instruction {
    command: Command,
    start: (u32, u32),
    end: (u32, u32),
}

pub fn parse_line(line: &str) -> Option<Instruction> {
    let parts: Vec<&str> = line.split_whitespace().collect();

    // Verify we have enough parts to parse
    if parts.len() < 4 {
        return None;
    }

    let command = match parts[0] {
        "toggle" => Command::Toggle,
        "turn" => match parts[1] {
            "on" => Command::TurnOn,
            "off" => Command::TurnOff,
            _ => return None,
        },
        _ => return None,
    };

    // This slices the vector depending on the command to get the coordinates parts
    let coords_parts = if parts[0] == "toggle" {
        &parts[1..]
    } else {
        &parts[2..]
    };

    let start_coords: Vec<&str> = coords_parts[0].split(',').collect();
    let end_coords: Vec<&str> = coords_parts[2].split(',').collect();

    // Verify we can actually split into coordinates
    if start_coords.len() != 2 || end_coords.len() != 2 {
        return None;
    }

    let start_x: u32 = start_coords[0].parse().ok()?;
    let start_y: u32 = start_coords[1].parse().ok()?;
    let end_x: u32 = end_coords[0].parse().ok()?;
    let end_y: u32 = end_coords[1].parse().ok()?;

    Some(Instruction {
        command,
        start: (start_x, start_y),
        end: (end_x, end_y),
    })
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid: Vec<Vec<u32>> = vec![vec![0; 1000]; 1000];

    for line in input.lines() {
        match parse_line(line) {
            Some(instruction) => {
                for x in instruction.start.0..=instruction.end.0 {
                    for y in instruction.start.1..=instruction.end.1 {
                        match instruction.command {
                            Command::TurnOn => grid[x as usize][y as usize] = 1,
                            Command::TurnOff => grid[x as usize][y as usize] = 0,
                            Command::Toggle => grid[x as usize][y as usize] ^= 1,
                        }
                    }
                }
            }
            None => println!("Invalid instruction: {}", line),
        }
    }

    Some(grid.iter().flatten().filter(|&&x| x == 1).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid: Vec<Vec<u32>> = vec![vec![0; 1000]; 1000];

    for line in input.lines() {
        match parse_line(line) {
            Some(instruction) => {
                for x in instruction.start.0..=instruction.end.0 {
                    for y in instruction.start.1..=instruction.end.1 {
                        match instruction.command {
                            Command::TurnOn => grid[x as usize][y as usize] += 1,
                            Command::TurnOff => {
                                if grid[x as usize][y as usize] > 0 {
                                    grid[x as usize][y as usize] -= 1
                                }
                            }
                            Command::Toggle => grid[x as usize][y as usize] += 2,
                        }
                    }
                }
            }
            None => println!("Invalid instruction: {}", line),
        }
    }

    Some(grid.iter().flatten().sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), None);
    }
}
