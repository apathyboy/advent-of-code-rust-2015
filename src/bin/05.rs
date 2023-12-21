advent_of_code::solution!(5);

pub fn contains_vowels(s: &str) -> bool {
    s.chars().filter(|c| "aeiou".contains(*c)).count() >= 3
}

pub fn contains_double_letter(s: &str) -> bool {
    s.chars().zip(s.chars().skip(1)).any(|(a, b)| a == b)
}

pub fn contains_naughty_strings(s: &str) -> bool {
    s.contains("ab") || s.contains("cd") || s.contains("pq") || s.contains("xy")
}

pub fn contains_repeated_pair(s: &str) -> bool {
    s.chars()
        .zip(s.chars().skip(1))
        .enumerate()
        .any(|(i, (a, b))| s[i + 2..].contains(&format!("{}{}", a, b)))
}

pub fn contains_repeated_letter(s: &str) -> bool {
    s.chars().zip(s.chars().skip(2)).any(|(a, b)| a == b)
}

pub fn part_one(input: &str) -> Option<u32> {
    input
        .lines()
        .filter(|s| contains_vowels(s) && contains_double_letter(s) && !contains_naughty_strings(s))
        .count()
        .try_into()
        .ok()
}

pub fn part_two(input: &str) -> Option<u32> {
    input
        .lines()
        .filter(|s| contains_repeated_pair(s) && contains_repeated_letter(s))
        .count()
        .try_into()
        .ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(part_two(&input), Some(2));
    }
}
