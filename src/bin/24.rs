use itertools::Itertools;

advent_of_code::solution!(24);

fn parse(input: &str) -> Vec<u32> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    let weights = parse(input);

    let total_weight: usize = weights.iter().map(|&a| a as usize).sum();
    let target_weight = total_weight / 3;

    let mut min_qe = usize::MAX;
    let mut min_length = usize::MAX;

    for length in 1..weights.len() {
        for combination in weights.iter().combinations(length) {
            if combination.iter().map(|&&a| a as usize).sum::<usize>() == target_weight {
                let qe = combination.iter().map(|&&a| a as usize).product::<usize>();
                if length < min_length || length == min_length && qe < min_qe {
                    min_qe = qe;
                    min_length = length;
                }
            }
        }
        if min_qe != usize::MAX {
            break;
        }
    }

    Some(min_qe)
}

pub fn part_two(input: &str) -> Option<usize> {
    let weights = parse(input);

    let total_weight: usize = weights.iter().map(|&a| a as usize).sum();
    let target_weight = total_weight / 4;

    let mut min_qe = usize::MAX;
    let mut min_length = usize::MAX;

    for length in 1..weights.len() {
        for combination in weights.iter().combinations(length) {
            if combination.iter().map(|&&a| a as usize).sum::<usize>() == target_weight {
                let qe = combination.iter().map(|&&a| a as usize).product::<usize>();
                if length < min_length || length == min_length && qe < min_qe {
                    min_qe = qe;
                    min_length = length;
                }
            }
        }
        if min_qe != usize::MAX {
            break;
        }
    }

    Some(min_qe)
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
