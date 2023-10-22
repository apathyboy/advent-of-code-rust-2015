use itertools::Itertools;
use std::collections::HashMap;

fn parse_distance(line: &str) -> Option<((&str, &str), u32)> {
    let mut parts = line.split(' ');
    let from = parts.next()?;
    let to = parts.nth(1)?;
    let distance = parts.nth(1)?.parse::<u32>().ok()?;
    Some(((from, to), distance))
}

fn generate_permutations<T: Clone>(items: &[T]) -> Vec<Vec<T>> {
    if items.len() == 1 {
        return vec![items.to_vec()];
    }

    let mut permutations = Vec::new();
    for (i, item) in items.iter().enumerate() {
        let mut items = items.to_vec();
        items.remove(i);
        let mut sub_permutations = generate_permutations(&items);
        for sub_permutation in &mut sub_permutations {
            sub_permutation.insert(0, item.clone());
        }
        permutations.append(&mut sub_permutations);
    }
    permutations
}

pub fn part_one(input: &str) -> Option<u32> {
    let distances = input
        .lines()
        .map(parse_distance)
        .collect::<Option<HashMap<(&str, &str), u32>>>()?;
    let cities = distances
        .keys()
        .flat_map(|(from, to)| vec![*from, *to])
        .unique()
        .collect_vec();

    let mut min_distance = u32::max_value();

    for permutation in generate_permutations(&cities) {
        let mut distance = 0;
        for i in 0..permutation.len() - 1 {
            if distances.contains_key(&(permutation[i], permutation[i + 1])) {
                distance += distances
                    .get(&(permutation[i], permutation[i + 1]))
                    .unwrap();
            } else {
                distance += distances
                    .get(&(permutation[i + 1], permutation[i]))
                    .unwrap();
            }
        }
        if distance < min_distance {
            min_distance = distance;
        }
    }

    Some(min_distance)
}

pub fn part_two(input: &str) -> Option<u32> {
    let distances = input
        .lines()
        .map(parse_distance)
        .collect::<Option<HashMap<(&str, &str), u32>>>()?;
    let cities = distances
        .keys()
        .flat_map(|(from, to)| vec![*from, *to])
        .unique()
        .collect_vec();

    let mut max_distance = 0;

    for permutation in generate_permutations(&cities) {
        let mut distance = 0;
        for i in 0..permutation.len() - 1 {
            if distances.contains_key(&(permutation[i], permutation[i + 1])) {
                distance += distances
                    .get(&(permutation[i], permutation[i + 1]))
                    .unwrap();
            } else {
                distance += distances
                    .get(&(permutation[i + 1], permutation[i]))
                    .unwrap();
            }
        }
        if distance > max_distance {
            max_distance = distance;
        }
    }

    Some(max_distance)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(982));
    }
}
