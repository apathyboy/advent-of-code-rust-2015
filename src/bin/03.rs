use nalgebra::Vector2;
use std::collections::HashSet;

#[must_use]
pub fn part_one(input: &str) -> Option<usize> {
    let current_house = Vector2::new(0, 0);

    let houses = input
        .chars()
        .filter_map(|c| match c {
            '<' => Some(Vector2::new(-1, 0)),
            '>' => Some(Vector2::new(1, 0)),
            '^' => Some(Vector2::new(0, 1)),
            'v' => Some(Vector2::new(0, -1)),
            _ => None,
        })
        .scan(current_house, |state, change| {
            *state += change;
            Some(*state)
        })
        .collect::<HashSet<_>>()
        .len();

    Some(houses)
}

#[must_use]
pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(4));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), None);
    }
}
