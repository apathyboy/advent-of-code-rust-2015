use itertools::Itertools;

pub fn make_sequence(input: Vec<char>) -> Vec<char> {
    input
        .into_iter()
        .group_by(|elt| *elt)
        .into_iter()
        .map(|(_, group)| group.collect::<Vec<char>>())
        .map(|v| {
            let first_char = v.first().unwrap();
            let count = v.len().to_string();
            format!("{}{}", count, first_char).chars().collect_vec()
        })
        .flatten()
        .collect()
}

fn run_sequence(input: &str, times: u32) -> Vec<char> {
    let mut sequence = input.chars().collect::<Vec<char>>();
    for _ in 0..times {
        sequence = make_sequence(sequence);
    }
    sequence
}

pub fn part_one(_input: &str) -> Option<usize> {
    Some(run_sequence(_input.trim(), 40).len())
}

pub fn part_two(_input: &str) -> Option<usize> {
    Some(run_sequence(_input.trim(), 50).len())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_making_sequence() {
        assert_eq!(
            make_sequence("1".chars().collect()),
            "11".chars().collect::<Vec<char>>()
        );
        assert_eq!(
            make_sequence("11".chars().collect()),
            "21".chars().collect::<Vec<char>>()
        );
        assert_eq!(
            make_sequence("21".chars().collect()),
            "1211".chars().collect::<Vec<char>>()
        );
        assert_eq!(
            make_sequence("1211".chars().collect()),
            "111221".chars().collect::<Vec<char>>()
        );
        assert_eq!(
            make_sequence("111221".chars().collect()),
            "312211".chars().collect::<Vec<char>>()
        );
        assert_eq!(
            run_sequence("1", 5),
            "312211".chars().collect::<Vec<char>>()
        )
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), None);
    }
}
