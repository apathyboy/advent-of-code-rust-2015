use std::collections::HashMap;

use glam::IVec2;

advent_of_code::solution!(18);

#[derive(Debug, Clone)]
struct Light {
    position: IVec2,
    on: bool,
    neighbours: Vec<IVec2>,
}

impl Light {
    fn new(position: IVec2, on: bool) -> Self {
        Self {
            position,
            on,
            neighbours: Vec::new(),
        }
    }

    fn step(&mut self, grid: &HashMap<IVec2, Light>) {
        let mut on_neighbours = 0;

        for neighbour in &self.neighbours {
            if let Some(light) = grid.get(neighbour) {
                if light.on {
                    on_neighbours += 1;
                }
            }
        }

        if self.on {
            if on_neighbours != 2 && on_neighbours != 3 {
                self.on = false;
            }
        } else if on_neighbours == 3 {
            self.on = true;
        }
    }
}

fn parse_grid(input: &str) -> HashMap<IVec2, Light> {
    let mut grid = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let position = IVec2::new(x as i32, y as i32);
            let light = Light::new(position, c == '#');

            grid.insert(position, light);
        }
    }

    grid
}

fn add_neighbors(light: &mut Light, grid: &HashMap<IVec2, Light>) {
    let mut neighbours = Vec::new();

    for x in -1..=1 {
        for y in -1..=1 {
            if x == 0 && y == 0 {
                continue;
            }

            let position = light.position + IVec2::new(x, y);
            if let Some(neighbour) = grid.get(&position) {
                neighbours.push(neighbour.position);
            }
        }
    }

    light.neighbours = neighbours;
}

fn walk_step_lights_part1(grid: &mut HashMap<IVec2, Light>, steps: u32) -> Option<u32> {
    let new_grid = grid.clone();

    for light in grid.values_mut() {
        add_neighbors(light, &new_grid);
    }

    (0..steps).for_each(|_| {
        let mut new_grid = grid.clone();

        for light in new_grid.values_mut() {
            light.step(grid);
        }

        grid.iter_mut().for_each(|(position, light)| {
            *light = new_grid.remove(position).unwrap();
        });
    });

    let mut on = 0;

    for light in grid.values() {
        if light.on {
            on += 1;
        }
    }

    Some(on)
}

fn walk_step_lights_part2(
    grid: &mut HashMap<IVec2, Light>,
    steps: u32,
    width: i32,
    height: i32,
) -> Option<u32> {
    let new_grid = grid.clone();

    for light in grid.values_mut() {
        add_neighbors(light, &new_grid);
    }

    (0..steps).for_each(|_| {
        let mut new_grid = grid.clone();

        for light in new_grid.values_mut() {
            if light.position == IVec2::new(0, 0)
                || light.position == IVec2::new(width, 0)
                || light.position == IVec2::new(0, height)
                || light.position == IVec2::new(width, height)
            {
                continue;
            }

            light.step(grid);
        }

        grid.iter_mut().for_each(|(position, light)| {
            *light = new_grid.remove(position).unwrap();
        });
    });

    let mut on = 0;

    for light in grid.values() {
        if light.on {
            on += 1;
        }
    }

    Some(on)
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid = parse_grid(input);

    walk_step_lights_part1(&mut grid, 100)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid = parse_grid(input);

    let width = grid
        .keys()
        .map(|position| position.x)
        .max()
        .unwrap_or_default() as i32;
    let height = grid
        .keys()
        .map(|position| position.y)
        .max()
        .unwrap_or_default() as i32;

    grid.get_mut(&IVec2::new(0, 0))
        .map(|light| light.on = true)
        .unwrap_or_default();
    grid.get_mut(&IVec2::new(width, 0))
        .map(|light| light.on = true)
        .unwrap_or_default();
    grid.get_mut(&IVec2::new(0, height))
        .map(|light| light.on = true)
        .unwrap_or_default();
    grid.get_mut(&IVec2::new(width, height))
        .map(|light| light.on = true)
        .unwrap_or_default();

    walk_step_lights_part2(&mut grid, 100, width, height)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let mut grid = parse_grid(&advent_of_code::template::read_file("examples", DAY));

        let result = walk_step_lights_part1(&mut grid, 4);
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two() {
        let mut grid = parse_grid(&advent_of_code::template::read_file("examples", DAY));

        let width = grid
            .keys()
            .map(|position| position.x)
            .max()
            .unwrap_or_default() as i32;
        let height = grid
            .keys()
            .map(|position| position.y)
            .max()
            .unwrap_or_default() as i32;

        grid.get_mut(&IVec2::new(0, 0))
            .map(|light| light.on = true)
            .unwrap_or_default();
        grid.get_mut(&IVec2::new(width, 0))
            .map(|light| light.on = true)
            .unwrap_or_default();
        grid.get_mut(&IVec2::new(0, height))
            .map(|light| light.on = true)
            .unwrap_or_default();
        grid.get_mut(&IVec2::new(width, height))
            .map(|light| light.on = true)
            .unwrap_or_default();

        let result = walk_step_lights_part2(&mut grid, 5, width, height);
        assert_eq!(result, Some(17));
    }
}
