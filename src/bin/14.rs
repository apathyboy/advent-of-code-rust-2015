fn parse_line(line: &str) -> Option<(u32, u32, u32)> {
    let parts = line.split(" ").collect::<Vec<&str>>();

    if parts.len() != 15 {
        return None;
    }

    let speed = parts[3].parse::<u32>().ok()?;
    let duration = parts[6].parse::<u32>().ok()?;
    let rest = parts[13].parse::<u32>().ok()?;

    Some((speed, duration, rest))
}

fn calculate_distance(speed: u32, duration: u32, rest: u32, time: u32) -> u32 {
    let cycle = duration + rest;
    let cycles = time / cycle;
    let remainder = time % cycle;

    let mut distance = speed * duration * cycles;

    if remainder > duration {
        distance += speed * duration;
    } else {
        distance += speed * remainder;
    }

    distance
}

pub fn part_one(_input: &str) -> Option<u32> {
    let max_distance = _input
        .lines()
        .map(|line| {
            let Some((speed, duration, rest)) = parse_line(line) else {
                return 0;
            };
            calculate_distance(speed, duration, rest, 2503)
        })
        .max();

    max_distance
}

pub fn part_two(_input: &str) -> Option<u32> {
    let reindeer = _input
        .lines()
        .map(|line| {
            let Some((speed, duration, rest)) = parse_line(line) else {
                return (0, 0, 0);
            };
            (speed, duration, rest)
        })
        .collect::<Vec<(u32, u32, u32)>>();
    let mut points = vec![0; reindeer.len()];
    let mut distances = vec![0; reindeer.len()];
    let mut time = 1;

    while time <= 2503 {
        for i in 0..reindeer.len() {
            let (speed, duration, rest) = reindeer[i];
            let distance = calculate_distance(speed, duration, rest, time);

            if distance > distances[i] {
                distances[i] = distance;
            }
        }

        let max_distance = distances.iter().max().unwrap();

        for i in 0..reindeer.len() {
            if distances[i] == *max_distance {
                points[i] += 1;
            }
        }

        time += 1;
    }

    points.iter().max().cloned()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_two(&input), None);
    }
}
