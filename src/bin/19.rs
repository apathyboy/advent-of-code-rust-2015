use std::collections::HashSet;

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
    let (_, molecule) = parse_input(input);

    let symbol_count = molecule.chars().filter(|c| c.is_uppercase()).count();
    let rn_count = molecule.matches("Rn").count();
    let ar_count = molecule.matches("Ar").count();
    let y_count = molecule.matches('Y').count();

    Some(symbol_count as u32 - rn_count as u32 - ar_count as u32 - 2 * y_count as u32 - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
