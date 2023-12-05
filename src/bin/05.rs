advent_of_code::solution!(5);

#[derive(Debug)]
struct ConversionMap {
    destination_start: i64,
    source_start: i64,
    length: i64,
}

fn parse_input(input: &str) -> (Vec<i64>, Vec<Vec<ConversionMap>>) {
    let mut maps = Vec::new();
    let mut current_map = Vec::new();
    let mut seeds = Vec::new();
    let mut is_seed = true;

    for line in input.lines() {
        if line.ends_with("map:") {
            is_seed = false;
            if !current_map.is_empty() {
                maps.push(current_map);
                current_map = Vec::new();
            }
        } else if !line.is_empty() {
            let parts: Vec<i64> = line
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();
            if is_seed {
                seeds = parts;
            } else if parts.len() == 3 {
                current_map.push(ConversionMap {
                    destination_start: parts[0],
                    source_start: parts[1],
                    length: parts[2],
                });
            }
        }
    }

    if !current_map.is_empty() {
        maps.push(current_map);
    }

    (seeds, maps)
}

fn convert_number(num: i64, map: &[ConversionMap]) -> i64 {
    map.iter()
        .find(|conversion| {
            num >= conversion.source_start && num < conversion.source_start + conversion.length
        })
        .map_or(num, |conversion| {
            conversion.destination_start + (num - conversion.source_start)
        })
}

fn convert_seed_to_location(seed: i64, maps: &[Vec<ConversionMap>]) -> i64 {
    maps.iter().fold(seed, |acc, map| convert_number(acc, map))
}

pub fn part_one(input: &str) -> Option<i64> {
    let (seeds, maps) = parse_input(input);

    let lowest_location = seeds
        .iter()
        .map(|&seed| Some(convert_seed_to_location(seed, &maps)))
        .min()
        .unwrap();

    lowest_location
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
