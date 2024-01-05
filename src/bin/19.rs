use std::collections::{HashSet, VecDeque};

advent_of_code::solution!(19);

#[derive(Debug)]
struct Replacement {
    from: String,
    to: String,
}

fn parse_input(input: &str) -> (Vec<Replacement>, String) {
    let mut replacements = Vec::new();
    let mut molecule = String::new();

    for line in input.lines() {
        if line.contains(" => ") {
            let mut parts = line.split(" => ");
            let from = parts.next().unwrap().to_string();
            let to = parts.next().unwrap().to_string();
            replacements.push(Replacement { from, to });
        } else if !line.is_empty() {
            molecule = line.to_string();
        }
    }

    (replacements, molecule)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (replacements, molecule) = parse_input(input);

    let mut molecules = HashSet::new();

    for replacement in replacements {
        let mut start = 0;
        while let Some(pos) = molecule[start..].find(&replacement.from) {
            let mut new_molecule = molecule.clone();
            new_molecule.replace_range(
                start + pos..start + pos + replacement.from.len(),
                &replacement.to,
            );
            molecules.insert(new_molecule);
            start += pos + replacement.from.len();
        }
    }

    Some(molecules.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (replacements, molecule) = parse_input(input);

    let electron = "e".to_string();

    let mut queue = VecDeque::new();
    queue.push_back((molecule, 0));

    while let Some((molecule, steps)) = queue.pop_front() {
        if molecule == electron {
            return Some(steps);
        }

        for replacement in &replacements {
            let mut start = 0;
            while let Some(pos) = molecule[start..].find(&replacement.to) {
                let mut new_molecule = molecule.clone();
                new_molecule.replace_range(
                    start + pos..start + pos + replacement.to.len(),
                    &replacement.from,
                );
                queue.push_back((new_molecule, steps + 1));
                start += pos + replacement.to.len();
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(3));

        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(6));
    }
}
