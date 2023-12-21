use advent_of_code::parse_dimensions;

advent_of_code::solution!(2);

fn calculate_paper(length: i32, width: i32, height: i32) -> i32 {
    let side1 = length * width;
    let side2 = width * height;
    let side3 = height * length;
    let smallest_side = side1.min(side2).min(side3);
    2 * side1 + 2 * side2 + 2 * side3 + smallest_side
}

fn calculate_ribbon(length: i32, width: i32, height: i32) -> i32 {
    let mut dimensions = [length, width, height];
    dimensions.sort_unstable();
    let smallest_side = dimensions[0];
    let second_smallest_side = dimensions[1];
    let volume = length * width * height;
    2 * smallest_side + 2 * second_smallest_side + volume
}

pub fn part_one(input: &str) -> Option<i32> {
    let total_paper: i32 = input
        .lines()
        .map(|line| match parse_dimensions(line) {
            Some((l, w, h)) => calculate_paper(l, w, h),
            None => panic!("Unexpected input"),
        })
        .sum();

    Some(total_paper)
}

pub fn part_two(input: &str) -> Option<i32> {
    let total_ribbon: i32 = input
        .lines()
        .map(|line| match parse_dimensions(line) {
            Some((l, w, h)) => calculate_ribbon(l, w, h),
            None => panic!("Unexpected input"),
        })
        .sum();

    Some(total_ribbon)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_paper() {
        assert_eq!(calculate_paper(2, 3, 4), 58);
        assert_eq!(calculate_paper(1, 1, 10), 43);
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(part_one(&input), Some(101));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(part_two(&input), Some(48));
    }
}
