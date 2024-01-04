use itertools::Itertools;

advent_of_code::solution!(17);

pub fn part_one(input: &str) -> Option<u32> {
    let mut combinations = 0;

    let nums = input
        .lines()
        .filter_map(|line| line.parse().ok())
        .collect::<Vec<u32>>();

    for i in 0..nums.len() {
        nums.iter().combinations(i).for_each(|c| {
            if c.into_iter().sum::<u32>() == 150 {
                combinations += 1;
            }
        });
    }

    Some(combinations)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut combinations = 0;

    let nums = input
        .lines()
        .filter_map(|line| line.parse().ok())
        .collect::<Vec<u32>>();

    for i in 0..nums.len() {
        nums.iter().combinations(i).for_each(|c| {
            if c.into_iter().sum::<u32>() == 150 {
                combinations += 1;
            }
        });

        if combinations > 0 {
            break;
        }
    }

    Some(combinations)
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
