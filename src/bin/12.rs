use regex::Regex;
use std::str::FromStr;

fn parse_numbers(input: &str) -> i32 {
    let re = Regex::new(r"([+-]?\d+)").unwrap();
    re.captures_iter(input)
        .map(|cap| i32::from_str(&cap[1]).unwrap())
        .sum()
}

fn find_open_bracket_from_position(input: &str, position: usize) -> usize {
    let mut open_brackets = 0;
    let mut index = position;
    loop {
        if input.chars().nth(index).unwrap() == '}' {
            open_brackets += 1;
        } else if input.chars().nth(index).unwrap() == '{' {
            open_brackets -= 1;
            if open_brackets < 0 {
                break;
            }
        }
        index -= 1;
    }
    index
}

fn find_close_bracket_from_position(input: &str, position: usize) -> usize {
    let mut close_brackets = 0;
    let mut index = position;
    loop {
        if input.chars().nth(index).unwrap() == '{' {
            close_brackets += 1;
        } else if input.chars().nth(index).unwrap() == '}' {
            close_brackets -= 1;
            if close_brackets < 0 {
                break;
            }
        }
        index += 1;
    }
    index
}

fn remove_red_objects(input: &str) -> String {
    match input.find(":\"red\"") {
        Some(index) => {
            let begin_index = find_open_bracket_from_position(input, index);
            let end_index = find_close_bracket_from_position(input, index);
            let mut new_input = input.to_string();
            new_input.replace_range(begin_index..=end_index, "");
            remove_red_objects(&new_input)
        }
        _ => input.to_string(),
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    Some(parse_numbers(input))
}

pub fn part_two(_input: &str) -> Option<i32> {
    let input = remove_red_objects(_input);
    Some(parse_numbers(&input))
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
