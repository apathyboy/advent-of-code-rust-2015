use advent_of_code::helpers::parse_dimensions;

fn calculate_paper(length: i32, width: i32, height: i32) -> i32 {
    let side1 = length * width;
    let side2 = width * height;
    let side3 = height * length;
    let smallest_side = side1.min(side2).min(side3);
    2 * side1 + 2 * side2 + 2 * side3 + smallest_side
}

pub fn part_one(_input: &str) -> Option<i32> {
    let total_paper: i32 = _input
        .lines()
        .map(|line| match parse_dimensions(line) {
            Some((l, w, h)) => calculate_paper(l, w, h),
            None => panic!("Unexpected input"),
        })
        .sum();

    Some(total_paper)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
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
        let input = advent_of_code::read_file("examples", 2);

        assert_eq!(part_one(&input), Some(101));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), None);
    }
}
