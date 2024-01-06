use std::collections::VecDeque;

use itertools::Itertools;

advent_of_code::solution!(22);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

const SPELLBOOK: [Spell; 5] = [
    Spell::MagicMissile,
    Spell::Drain,
    Spell::Shield,
    Spell::Poison,
    Spell::Recharge,
];

#[derive(Clone, Copy)]
struct Effect(Spell, u32);

impl Effect {
    fn new(spell: Spell) -> Option<Self> {
        match spell {
            Spell::Shield | Spell::Poison => Some(Self(spell, 6)),
            Spell::Recharge => Some(Self(spell, 5)),
            _ => None,
        }
    }

    fn run(&mut self, wizard: &mut Player, boss: &mut Player) {
        self.1 -= 1;
        match self.0 {
            Spell::Shield => {
                if self.1 == 0 {
                    wizard.armor -= 7;
                }
            }
            Spell::Poison => boss.hit_points = boss.hit_points.saturating_sub(3),
            Spell::Recharge => wizard.mana += 101,
            _ => unreachable!(),
        }
    }

    fn is_active(&self) -> bool {
        self.1 != 0
    }
}

#[derive(Debug, Clone, Copy)]
struct Player {
    hit_points: i32,
    damage: i32,
    armor: i32,
    mana: i32,
}

impl Player {
    fn attack(&self, other: &mut Self) {
        let damage = match self.damage.saturating_sub(other.armor) {
            0 => 1,
            n => n,
        };
        other.hit_points = other.hit_points.saturating_sub(damage)
    }

    fn is_alive(&self) -> bool {
        self.hit_points > 0
    }

    fn cast(&mut self, spell: Spell, boss: &mut Player) -> Result<(Option<Effect>, i32), ()> {
        let cost: i32;

        match spell {
            Spell::MagicMissile => {
                cost = 53;
                boss.hit_points = boss.hit_points.saturating_sub(4);
            }
            Spell::Drain => {
                cost = 73;
                self.hit_points += 2;
                boss.hit_points = boss.hit_points.saturating_sub(2);
            }
            Spell::Shield => {
                cost = 113;
                self.armor += 7;
            }
            Spell::Poison => cost = 173,
            Spell::Recharge => cost = 229,
        }

        if self.mana < cost {
            return Err(());
        }

        self.mana -= cost;
        Ok((Effect::new(spell), cost))
    }
}

fn do_battle(boss: Player, hard_mode: bool) -> i32 {
    let mut minimum_mana = std::i32::MAX;

    let mut queue = VecDeque::new();

    for i in 0..SPELLBOOK.len() {
        queue.push_back((
            Player {
                hit_points: 50,
                armor: 0,
                damage: 0,
                mana: 500,
            },
            boss,
            i,
            Vec::<Effect>::new(),
            0_i32,
        ))
    }

    while let Some(battle) = queue.pop_front() {
        let (mut player, mut boss, current_spell, mut effects, mut mana_expended) = battle;

        if hard_mode {
            player.hit_points -= 1;

            if !player.is_alive() {
                continue;
            }
        }

        for effect in &mut effects {
            effect.run(&mut player, &mut boss);
        }

        if !boss.is_alive() {
            minimum_mana = minimum_mana.min(mana_expended);
            continue;
        }

        effects.retain(|e| e.is_active());

        let spell = SPELLBOOK[current_spell];

        if effects.iter().any(|eff| eff.0 == spell) {
            continue;
        }

        mana_expended += match player.cast(spell, &mut boss) {
            Ok((Some(effect), cost)) => {
                effects.push(effect);
                cost
            }
            Ok((None, cost)) => cost,
            Err(()) => continue,
        };

        if mana_expended >= minimum_mana {
            continue;
        }

        if !boss.is_alive() {
            minimum_mana = minimum_mana.min(mana_expended);
            continue;
        }

        for effect in &mut effects {
            effect.run(&mut player, &mut boss);
        }

        if !boss.is_alive() {
            minimum_mana = minimum_mana.min(mana_expended);
            continue;
        }

        effects.retain(|e| e.is_active());

        boss.attack(&mut player);

        if !player.is_alive() {
            continue;
        }

        for i in 0..SPELLBOOK.len() {
            queue.push_back((player, boss, i, effects.clone(), mana_expended))
        }
    }

    minimum_mana
}

fn parse_boss(input: &str) -> Option<Player> {
    let (boss_hit_points, boss_damage) = input.lines().collect_tuple()?;
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

    Some(Player {
        hit_points: boss_hit_points,
        damage: boss_damage,
        armor: 0,
        mana: 0,
    })
}

pub fn part_one(input: &str) -> Option<i32> {
    let boss = parse_boss(input)?;

    Some(do_battle(boss, false))
}

pub fn part_two(input: &str) -> Option<i32> {
    let boss = parse_boss(input)?;

    Some(do_battle(boss, true))
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
