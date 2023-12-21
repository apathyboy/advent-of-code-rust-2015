use itertools::Itertools;

advent_of_code::solution!(10);

fn look_and_say(input: &str) -> String {
    input
        .chars()
        .group_by(|&char| char)
        .into_iter()
        .map(|(key, group)| {
            let count = group.count();
            let count_str: String = count.to_string();
            let key_str: String = key.to_string();
            count_str + &key_str
        })
        .collect()
}

fn run_sequence(input: &str, times: u32) -> String {
    (0..times).fold(input.to_string(), |acc, _| look_and_say(&acc))
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(run_sequence(input.trim(), 40).len())
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(run_sequence(input.trim(), 50).len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_making_sequence() {
        assert_eq!(look_and_say("1"), "11".to_string());
        assert_eq!(look_and_say("11"), "21".to_string());
        assert_eq!(look_and_say("21"), "1211".to_string());
        assert_eq!(look_and_say("1211"), "111221".to_string());
        assert_eq!(look_and_say("111221"), "312211".to_string());
        assert_eq!(run_sequence("1", 5), "312211".to_string())
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(part_one(&input), Some(82350));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(part_two(&input), Some(1166642));
    }
}
