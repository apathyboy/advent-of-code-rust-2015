use itertools::Itertools;
use std::collections::HashMap;

fn parse_line(line: &str) -> Option<(String, String, i32)> {
    let parts = line.split(" ").collect::<Vec<&str>>();

    if parts.len() != 11 {
        return None;
    }

    let guest1 = parts[0].to_string();
    let guest2 = parts[10]
        .chars()
        .take(parts[10].len() - 1)
        .collect::<String>();
    let happiness = parts[3].parse::<i32>().ok()?;

    let result = match parts[2] {
        "gain" => Some((guest1, guest2, happiness)),
        "lose" => Some((guest1, guest2, -happiness)),
        _ => return None,
    };

    result
}

fn parse_guest_list(input: &str) -> HashMap<String, HashMap<String, i32>> {
    let mut guest_map = std::collections::HashMap::new();

    for line in input.lines() {
        if let Some((guest1, guest2, happiness)) = parse_line(line) {
            guest_map
                .entry(guest1)
                .or_insert_with(std::collections::HashMap::new)
                .insert(guest2, happiness);
        }
    }

    guest_map
}

fn find_max_happiness(guest_map: &HashMap<String, HashMap<String, i32>>) -> i32 {
    let mut max_happiness = 0;

    for permutation in guest_map.keys().permutations(guest_map.len()) {
        let mut happiness = 0;

        for i in 0..permutation.len() {
            let guest1 = &permutation[i];
            let guest2 = &permutation[(i + 1) % permutation.len()];

            happiness += guest_map[*guest1][*guest2];
            happiness += guest_map[*guest2][*guest1];
        }

        if happiness > max_happiness {
            max_happiness = happiness;
        }
    }

    max_happiness
}

pub fn part_one(input: &str) -> Option<i32> {
    let guest_map = parse_guest_list(input);

    Some(find_max_happiness(&guest_map))
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut guest_map = parse_guest_list(input);

    let guests = guest_map.keys().cloned().collect::<Vec<String>>();

    for guest in guests {
        guest_map
            .entry("me".to_string())
            .or_insert_with(std::collections::HashMap::new)
            .insert(guest.to_string(), 0);
        guest_map
            .entry(guest.to_string())
            .or_insert_with(std::collections::HashMap::new)
            .insert("me".to_string(), 0);
    }

    Some(find_max_happiness(&guest_map))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(330));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input), None);
    }
}
