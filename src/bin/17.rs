use itertools::Itertools;

advent_of_code::solution!(17);

fn parse_input(input: &str) -> Vec<u32> {
    input
        .lines()
        .filter_map(|line| line.parse().ok())
        .collect::<Vec<u32>>()
}

fn find_combinations(nums: &[u32], excess_eggnog: u32) -> Option<u32> {
    let mut combinations = 0;

    for i in 0..nums.len() {
        nums.iter().combinations(i).for_each(|c| {
            if c.into_iter().sum::<u32>() == excess_eggnog {
                combinations += 1;
            }
        });
    }

    Some(combinations)
}

fn find_min_combinations(nums: &[u32], excess_eggnog: u32) -> Option<u32> {
    let mut combinations = 0;

    for i in 0..nums.len() {
        nums.iter().combinations(i).for_each(|c| {
            if c.into_iter().sum::<u32>() == excess_eggnog {
                combinations += 1;
            }
        });

        if combinations > 0 {
            break;
        }
    }

    Some(combinations)
}

pub fn part_one(input: &str) -> Option<u32> {
    let nums = parse_input(input);

    find_combinations(&nums, 150)
}

pub fn part_two(input: &str) -> Option<u32> {
    let nums = parse_input(input);

    find_min_combinations(&nums, 150)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let nums = parse_input(&advent_of_code::template::read_file("examples", DAY));
        let result = find_combinations(&nums, 25);
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two() {
        let nums = parse_input(&advent_of_code::template::read_file("examples", DAY));
        let result = find_min_combinations(&nums, 25);
        assert_eq!(result, Some(3));
    }
}
