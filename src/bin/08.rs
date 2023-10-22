fn count_chars(input: &str) -> (usize, usize) {
    let mut chars = 0;
    let mut mem = 0;
    let mut iter = input.chars();
    while let Some(c) = iter.next() {
        chars += 1;
        match c {
            '\\' => match iter.next() {
                Some('\\') => {
                    mem += 1;
                    chars += 1;
                }
                Some('\"') => {
                    mem += 1;
                    chars += 1;
                }
                Some('x') => {
                    iter.next();
                    iter.next();
                    mem += 1;
                    chars += 3;
                }
                _ => {}
            },
            '\"' => {}
            _ => {
                mem += 1;
            }
        }
    }
    (chars, mem)
}

fn encode_chars(input: &str) -> (usize, usize) {
    let mut chars = 0;
    let mut mem = 2;
    let mut iter = input.chars();
    while let Some(c) = iter.next() {
        chars += 1;
        match c {
            '\\' => match iter.next() {
                Some('\\') => {
                    mem += 4;
                    chars += 1;
                }
                Some('\"') => {
                    mem += 4;
                    chars += 1;
                }
                Some('x') => {
                    iter.next();
                    iter.next();
                    mem += 5;
                    chars += 3;
                }
                _ => {}
            },
            '\"' => {
                mem += 2;
            }
            _ => {
                mem += 1;
            }
        }
    }
    (chars, mem)
}

pub fn part_one(_input: &str) -> Option<usize> {
    _input
        .lines()
        .map(count_chars)
        .fold(Some(0), |acc, (chars, mem)| {
            acc.and_then(|acc| Some(acc + chars - mem))
        })
}

pub fn part_two(_input: &str) -> Option<usize> {
    _input
        .lines()
        .map(encode_chars)
        .fold(Some(0), |acc, (chars, mem)| {
            acc.and_then(|acc| Some(acc + mem - chars))
        })
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), None);
    }
}
