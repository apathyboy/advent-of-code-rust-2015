use itertools::Itertools;

advent_of_code::solution!(15);

#[derive(Debug)]
struct Ingredient {
    capacity: isize,
    durability: isize,
    flavor: isize,
    texture: isize,
    calories: isize,
}

fn parse(line: &str) -> Option<Ingredient> {
    let (_, rest) = line.split_once(": ")?;
    let (capacity, rest) = rest.split_once(", ")?;
    let (durability, rest) = rest.split_once(", ")?;
    let (flavor, rest) = rest.split_once(", ")?;
    let (texture, calories) = rest.split_once(", ")?;

    Some(Ingredient {
        capacity: capacity[9..].parse().ok()?,
        durability: durability[11..].parse().ok()?,
        flavor: flavor[7..].parse().ok()?,
        texture: texture[8..].parse().ok()?,
        calories: calories[9..].parse().ok()?,
    })
}

pub fn part_one(input: &str) -> Option<isize> {
    let ingredients = input.lines().filter_map(parse).collect::<Vec<_>>();

    let mut max_score = 0;

    let target_sum: isize = 100;
    let num_numbers: usize = ingredients.len();
    let min_value: isize = 1;

    let permutations = (min_value..=target_sum - (num_numbers as isize - 1))
        .permutations(num_numbers)
        .filter(|combination| combination.iter().sum::<isize>() == target_sum);

    for combination in permutations {
        let mut capacity = 0;
        let mut durability = 0;
        let mut flavor = 0;
        let mut texture = 0;

        for (ingredient, amount) in ingredients.iter().zip(combination) {
            capacity += ingredient.capacity * amount;
            durability += ingredient.durability * amount;
            flavor += ingredient.flavor * amount;
            texture += ingredient.texture * amount;
        }

        if capacity < 0 || durability < 0 || flavor < 0 || texture < 0 {
            continue;
        }

        let score = capacity * durability * flavor * texture;

        if score > max_score {
            max_score = score;
        }
    }

    Some(max_score)
}

pub fn part_two(input: &str) -> Option<isize> {
    let ingredients = input.lines().filter_map(parse).collect::<Vec<_>>();

    let mut max_score = 0;

    let target_sum: isize = 100;
    let num_numbers: usize = ingredients.len();
    let min_value: isize = 1;

    let permutations = (min_value..=target_sum - (num_numbers as isize - 1))
        .permutations(num_numbers)
        .filter(|combination| combination.iter().sum::<isize>() == target_sum);

    for combination in permutations {
        let mut capacity = 0;
        let mut durability = 0;
        let mut flavor = 0;
        let mut texture = 0;
        let mut calories = 0;

        for (ingredient, amount) in ingredients.iter().zip(combination) {
            capacity += ingredient.capacity * amount;
            durability += ingredient.durability * amount;
            flavor += ingredient.flavor * amount;
            texture += ingredient.texture * amount;
            calories += ingredient.calories * amount;
        }

        if capacity < 0 || durability < 0 || flavor < 0 || texture < 0 || calories != 500 {
            continue;
        }

        let score = capacity * durability * flavor * texture;

        if score > max_score {
            max_score = score;
        }
    }

    Some(max_score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(part_one(&input), Some(62842880));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(part_two(&input), Some(57600000));
    }
}
