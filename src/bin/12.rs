use regex::Regex;
use std::str::FromStr;

fn parse_numbers(input: &str) -> i32 {
    let re = Regex::new(r"([+-]?\d+)").unwrap();
    re.captures_iter(input)
        .map(|cap| i32::from_str(&cap[1]).unwrap())
        .sum()
}

pub fn part_one(input: &str) -> Option<i32> {
    let result = input.lines().map(|line| parse_numbers(line)).sum();

    Some(result)
}

pub fn part_two(_input: &str) -> Option<i32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(6));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), Some(4));
    }
}
