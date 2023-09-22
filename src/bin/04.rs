use md5;

pub fn make_secret(input: &str, suffix: u32) -> String {
    format!("{}{}", input.trim(), suffix)
}

pub fn make_hash(input: &str) -> String {
    let digest = md5::compute(input);
    format!("{:x}", digest)
}

pub fn is_valid_hash(input: &str, precision: usize) -> bool {
    input.starts_with(&"0".repeat(precision))
}

pub fn find_suffix(input: &str, precision: usize) -> Option<u32> {
    (1..).find(|&suffix| {
        let secret = make_secret(input, suffix);
        let hash = make_hash(&secret);
        is_valid_hash(&hash, precision)
    })
}

pub fn part_one(input: &str) -> Option<u32> {
    find_suffix(input, 5)
}

pub fn part_two(input: &str) -> Option<u32> {
    find_suffix(input, 6)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secret_maker() {
        let secret = make_secret("abcdef", 1234);
        assert_eq!(secret, "abcdef1234");
    }

    #[test]
    fn test_hash_maker() {
        let hash = make_hash("abcdef1234");
        assert_eq!(hash, "9bb793c73de0193293096d68f93d2e75");
    }

    #[test]
    fn test_hash_validator() {
        assert_eq!(is_valid_hash("000003c73de0193293096d68f93d2e75", 5), true);
        assert_eq!(is_valid_hash("9bb793c73de0193293096d68f93d2e75", 5), false);
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(609043));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(6742839));
    }
}
