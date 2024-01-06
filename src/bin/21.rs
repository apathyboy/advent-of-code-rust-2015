use itertools::Itertools;

advent_of_code::solution!(21);

fn parse_boss(input: &str) -> Option<(i32, i32, i32)> {
    let (boss_hit_points, boss_damage, boss_armor) = input.lines().collect_tuple()?;
    let boss_hit_points = boss_hit_points
        .split_whitespace()
        .nth(2)
        .unwrap()
        .parse::<i32>()
        .unwrap();
    let boss_damage = boss_damage
        .split_whitespace()
        .nth(1)
        .unwrap()
        .parse::<i32>()
        .unwrap();
    let boss_armor = boss_armor
        .split_whitespace()
        .nth(1)
        .unwrap()
        .parse::<i32>()
        .unwrap();

    Some((boss_hit_points, boss_damage, boss_armor))
}

fn create_store() -> (
    [(u32, i32, i32); 5],
    [(u32, i32, i32); 6],
    [(u32, i32, i32); 8],
) {
    let weapons_store = [(8, 4, 0), (10, 5, 0), (25, 6, 0), (40, 7, 0), (74, 8, 0)];
    let armor_store = [
        (0, 0, 0),
        (13, 0, 1),
        (31, 0, 2),
        (53, 0, 3),
        (75, 0, 4),
        (102, 0, 5),
    ];
    let rings_store = [
        (0, 0, 0),
        (0, 0, 0),
        (25, 1, 0),
        (50, 2, 0),
        (100, 3, 0),
        (20, 0, 1),
        (40, 0, 2),
        (80, 0, 3),
    ];

    (weapons_store, armor_store, rings_store)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (boss_hit_points, boss_damage, boss_armor) = parse_boss(input)?;
    let (weapons_store, armor_store, rings_store) = create_store();

    let mut min_cost = u32::MAX;

    for weapon in weapons_store {
        for armor in armor_store.iter().combinations(1) {
            for rings in rings_store.iter().combinations(2) {
                let cost = weapon.0 + armor[0].0 + rings[0].0 + rings[1].0;
                let damage: i32 = weapon.1 + armor[0].1 + rings[0].1 + rings[1].1;
                let armor: i32 = weapon.2 + armor[0].2 + rings[0].2 + rings[1].2;

                let mut boss_hp: i32 = boss_hit_points;
                let mut player_hp: i32 = 100;

                loop {
                    boss_hp = boss_hp.saturating_sub(damage.saturating_sub(boss_armor));
                    if boss_hp <= 0 {
                        break;
                    }
                    player_hp = player_hp.saturating_sub(boss_damage.saturating_sub(armor));
                    if player_hp <= 0 {
                        break;
                    }
                }

                if boss_hp <= 0 && cost < min_cost {
                    min_cost = cost;
                }
            }
        }
    }

    Some(min_cost)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (boss_hit_points, boss_damage, boss_armor) = parse_boss(input)?;
    let (weapons_store, armor_store, rings_store) = create_store();

    let mut max_cost = 0;

    for weapon in weapons_store {
        for armor in armor_store.iter().combinations(1) {
            for rings in rings_store.iter().combinations(2) {
                let cost = weapon.0 + armor[0].0 + rings[0].0 + rings[1].0;
                let damage: i32 = weapon.1 + armor[0].1 + rings[0].1 + rings[1].1;
                let armor: i32 = weapon.2 + armor[0].2 + rings[0].2 + rings[1].2;

                let mut boss_hp: i32 = boss_hit_points;
                let mut player_hp: i32 = 100;

                loop {
                    boss_hp = boss_hp.saturating_sub(damage.saturating_sub(boss_armor));
                    if boss_hp <= 0 {
                        break;
                    }
                    player_hp = player_hp.saturating_sub(boss_damage.saturating_sub(armor));
                    if player_hp <= 0 {
                        break;
                    }
                }

                if player_hp <= 0 && cost > max_cost {
                    max_cost = cost;
                }
            }
        }
    }

    Some(max_cost)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_parse_boss() {
        let result = parse_boss(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some((12, 7, 2)));
    }
}
