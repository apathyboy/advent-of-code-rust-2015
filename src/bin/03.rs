use nalgebra::Vector2;
use std::collections::HashSet;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<usize> {
    let houses = input
        .chars()
        .fold(
            (Vector2::new(0, 0), HashSet::new()),
            |(mut pos, mut visited), c| {
                match c {
                    '<' => pos.x -= 1,
                    '>' => pos.x += 1,
                    '^' => pos.y += 1,
                    'v' => pos.y -= 1,
                    _ => {}
                }
                visited.insert(pos);
                (pos, visited)
            },
        )
        .1
        .len();

    Some(houses)
}

pub fn part_two(input: &str) -> Option<usize> {
    let initial_state = (Vector2::new(0, 0), Vector2::new(0, 0), HashSet::new());

    let (_, _, visited) = input.chars().enumerate().fold(
        initial_state,
        |(mut santa_pos, mut robo_pos, mut visited), (idx, ch)| {
            let current_pos = if idx % 2 == 0 {
                &mut santa_pos
            } else {
                &mut robo_pos
            };

            match ch {
                '^' => current_pos.y += 1,
                'v' => current_pos.y -= 1,
                '>' => current_pos.x += 1,
                '<' => current_pos.x -= 1,
                _ => {}
            }

            visited.insert(*current_pos);

            (santa_pos, robo_pos, visited)
        },
    );

    Some(visited.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(part_one(&input), Some(4));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(part_two(&input), Some(3));
    }
}
