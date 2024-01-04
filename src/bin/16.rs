use itertools::Itertools;

advent_of_code::solution!(16);

#[derive(Debug)]
struct Sue {
    number: u32,
    children: Option<u32>,
    cats: Option<u32>,
    samoyeds: Option<u32>,
    pomeranians: Option<u32>,
    akitas: Option<u32>,
    vizslas: Option<u32>,
    goldfish: Option<u32>,
    trees: Option<u32>,
    cars: Option<u32>,
    perfumes: Option<u32>,
}

fn parse_sue(line: &str) -> Option<Sue> {
    line.splitn(2, ": ").collect_tuple().map(|(number, rest)| {
        let number = number
            .strip_prefix("Sue ")
            .and_then(|number| number.parse().ok())
            .unwrap();

        let mut children = None;
        let mut cats = None;
        let mut samoyeds = None;
        let mut pomeranians = None;
        let mut akitas = None;
        let mut vizslas = None;
        let mut goldfish = None;
        let mut trees = None;
        let mut cars = None;
        let mut perfumes = None;

        for property in rest.split(", ") {
            let (name, value) = property.splitn(2, ": ").collect_tuple().unwrap();
            let value = value.parse().unwrap();

            match name {
                "children" => children = Some(value),
                "cats" => cats = Some(value),
                "samoyeds" => samoyeds = Some(value),
                "pomeranians" => pomeranians = Some(value),
                "akitas" => akitas = Some(value),
                "vizslas" => vizslas = Some(value),
                "goldfish" => goldfish = Some(value),
                "trees" => trees = Some(value),
                "cars" => cars = Some(value),
                "perfumes" => perfumes = Some(value),
                _ => unreachable!(),
            }
        }

        Sue {
            number,
            children,
            cats,
            samoyeds,
            pomeranians,
            akitas,
            vizslas,
            goldfish,
            trees,
            cars,
            perfumes,
        }
    })
}

pub fn part_one(input: &str) -> Option<u32> {
    let sue = input.lines().filter_map(parse_sue).find(|sue| {
        let children_match = matches!(sue.children, None | Some(3));
        let cats_match = matches!(sue.cats, None | Some(7));
        let samoyeds_match = matches!(sue.samoyeds, None | Some(2));
        let pomeranians_match = matches!(sue.pomeranians, None | Some(3));
        let akitas_match = matches!(sue.akitas, None | Some(0));
        let vizslas_match = matches!(sue.vizslas, None | Some(0));
        let goldfish_match = matches!(sue.goldfish, None | Some(5));
        let trees_match = matches!(sue.trees, None | Some(3));
        let cars_match = matches!(sue.cars, None | Some(3));
        let perfumes_match = matches!(sue.perfumes, None | Some(1));

        children_match
            && cats_match
            && samoyeds_match
            && pomeranians_match
            && akitas_match
            && vizslas_match
            && goldfish_match
            && trees_match
            && cars_match
            && perfumes_match
    })?;

    Some(sue.number)
}

pub fn part_two(input: &str) -> Option<u32> {
    let sue = input.lines().filter_map(parse_sue).find(|sue| {
        let children_match = matches!(sue.children, None | Some(3));

        let cats_match = match sue.cats {
            None => true,
            Some(val) => val > 7,
        };

        let samoyeds_match = matches!(sue.samoyeds, None | Some(2));

        let pomeranians_match = match sue.pomeranians {
            None => true,
            Some(val) => val < 3,
        };

        let akitas_match = matches!(sue.akitas, None | Some(0));
        let vizslas_match = matches!(sue.vizslas, None | Some(0));

        let goldfish_match = match sue.goldfish {
            None => true,
            Some(val) => val < 5,
        };

        let trees_match = match sue.trees {
            None => true,
            Some(val) => val > 3,
        };

        let cars_match = matches!(sue.cars, None | Some(3));
        let perfumes_match = matches!(sue.perfumes, None | Some(1));

        children_match
            && cats_match
            && samoyeds_match
            && pomeranians_match
            && akitas_match
            && vizslas_match
            && goldfish_match
            && trees_match
            && cars_match
            && perfumes_match
    })?;

    Some(sue.number)
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
