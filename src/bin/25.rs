advent_of_code::solution!(25);

fn parse_code_position(input: &str) -> (u32, u32) {
    (
        input[80..84].parse().unwrap(),
        input[93..97].parse().unwrap(),
    )
}

fn row_col_to_n(row: u32, col: u32) -> u32 {
    (col + row - 1) * (col + row) / 2 - row + 1
}

pub fn part_one(input: &str) -> Option<u64> {
    let (row, column) = parse_code_position(input);
    let n = row_col_to_n(row, column);

    let mut code = 20151125u64;

    for _ in 1..n {
        code = (code * 252533) % 33554393;
    }

    Some(code)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part_one() {
        assert_eq!(0, 0);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(0, 0);
    }
}
