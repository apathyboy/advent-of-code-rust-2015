advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<i32> {
    let sum = input
        .chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => panic!("Unexpected character"),
        })
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut sum = 0;

    input
        .chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => panic!("Unexpected character"),
        })
        .enumerate()
        .find_map(|(idx, x)| {
            sum += x;

            if sum == -1 {
                Some(idx + 1)
            } else {
                None
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(part_one(&input), Some(-1));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(part_two(&input), Some(3));
    }
}
