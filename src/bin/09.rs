use itertools::Itertools;
use std::collections::HashMap;

fn parse_distance(line: &str) -> Option<((&str, &str), u32)> {
    let mut parts = line.split(' ');
    let from = parts.next()?;
    let to = parts.nth(1)?;
    let distance = parts.nth(1)?.parse::<u32>().ok()?;
    Some(((from, to), distance))
}

fn calculate_distance(distances: &HashMap<(&str, &str), u32>, cities: Vec<&&str>) -> u32 {
    let mut distance = 0;
    for i in 0..cities.len() - 1 {
        if distances.contains_key(&(cities[i], cities[i + 1])) {
            distance += distances.get(&(cities[i], cities[i + 1])).unwrap();
        } else {
            distance += distances.get(&(cities[i + 1], cities[i])).unwrap();
        }
    }
    distance
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

    cities
        .iter()
        .permutations(cities.len())
        .unique()
        .map(|permutation| calculate_distance(&distances, permutation))
        .min()
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

    cities
        .iter()
        .permutations(cities.len())
        .unique()
        .map(|permutation| calculate_distance(&distances, permutation))
        .max()
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
