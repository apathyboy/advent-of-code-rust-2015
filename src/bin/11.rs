advent_of_code::solution!(11);

fn increment_password(password: &str) -> String {
    let mut password = password.to_string();
    let mut i = password.len() - 1;
    loop {
        let mut c = password.chars().nth(i).unwrap();
        if c == 'z' {
            c = 'a';
            password.replace_range(i..=i, &c.to_string());

            if i == 0 {
                password.insert(0, 'a');
                break;
            }

            i -= 1;
        } else {
            c = (c as u8 + 1) as char;
            password.replace_range(i..=i, &c.to_string());
            break;
        }
    }
    password
}

fn password_is_valid(password: &str) -> bool {
    let mut has_straight = false;
    let mut has_two_pairs = false;
    let mut pairs = 0;
    let mut last_char = ' ';
    let mut last_last_char = ' ';
    for c in password.chars() {
        if c == 'i' || c == 'o' || c == 'l' {
            return false;
        }

        if c == last_char && c != last_last_char {
            pairs += 1;
            if pairs == 2 {
                has_two_pairs = true;
            }
        }

        if c as u8 == last_char as u8 + 1 && last_char as u8 == last_last_char as u8 + 1 {
            has_straight = true;
        }

        last_last_char = last_char;
        last_char = c;
    }

    has_straight && has_two_pairs
}

fn find_next_password(password: &str) -> String {
    let mut password = password.to_string();
    loop {
        password = increment_password(&password);
        if password_is_valid(&password) {
            break;
        }
    }
    password
}

pub fn part_one(input: &str) -> Option<String> {
    Some(find_next_password(input.trim()))
}

pub fn part_two(_input: &str) -> Option<String> {
    Some(find_next_password(&find_next_password(_input.trim())))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_is_valid() {
        assert_eq!(password_is_valid("hijklmmn"), false);
        assert_eq!(password_is_valid("abbceffg"), false);
        assert_eq!(password_is_valid("abbcegjk"), false);
        assert_eq!(password_is_valid("abcdffaa"), true);
        assert_eq!(password_is_valid("ghjaabcc"), true);
    }

    #[test]
    fn test_increment_password() {
        assert_eq!(increment_password("a"), "b");
        assert_eq!(increment_password("z"), "aa");
        assert_eq!(increment_password("aa"), "ab");
        assert_eq!(increment_password("az"), "ba");
        assert_eq!(increment_password("zz"), "aaa");
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(part_one(&input), Some("ghjaabcc".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(part_two(&input), Some("ghjbbcdd".to_string()));
    }
}
