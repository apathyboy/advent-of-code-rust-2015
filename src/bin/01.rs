pub fn part_one(input: &str) -> Option<i32> {
    let up_count = input.chars().filter(|&c| c == '(').count() as i32;
    let down_count = input.chars().filter(|&c| c == ')').count() as i32;

    Some(up_count - down_count)
}

pub fn part_two(input: &str) -> Option<u32> {
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
                Some(idx as u32 + 1)
            } else {
                None
            }
        })
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(-1));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(3));
    }
}
