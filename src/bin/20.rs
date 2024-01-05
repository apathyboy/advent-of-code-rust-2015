advent_of_code::solution!(20);

pub fn part_one(input: &str) -> Option<u32> {
    let target = input.trim().parse::<u32>().ok()?;

    let mut house = target / 44;
    let mut presents = 0;
    while presents < target {
        house += 1;
        presents = (1..=house)
            .filter(|elf| house % elf == 0)
            .map(|elf| elf * 10)
            .sum();
    }

    Some(house)
}

pub fn part_two(input: &str) -> Option<u32> {
    let target = input.trim().parse::<u32>().ok()?;

    let mut house = target / 45;
    let mut presents = 0;
    while presents < target {
        house += 1;
        presents = (1..=house)
            .filter(|elf| house % elf == 0)
            .filter(|elf| house / elf <= 50)
            .map(|elf| elf * 11)
            .sum();
    }

    Some(house)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
