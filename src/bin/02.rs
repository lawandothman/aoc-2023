advent_of_code::solution!(2);

struct CubeSet {
    red: u32,
    green: u32,
    blue: u32,
}

struct Game {
    id: u32,
    sets: Vec<CubeSet>,
}

fn game_is_possible(game: &Game, max_cubes: &CubeSet) -> bool {
    game.sets.iter().all(|set| {
        set.red <= max_cubes.red && set.green <= max_cubes.green && set.blue <= max_cubes.blue
    })
}

fn parse_input(input: &str) -> Vec<Game> {
    input
        .lines()
        .filter_map(|line| {
            if let Some((id_part, sets_part)) = line.split_once(": ") {
                let id = id_part.trim_start_matches("Game ").parse::<u32>().ok()?;

                let sets = sets_part
                    .split(';')
                    .filter_map(|set| {
                        let mut red = 0;
                        let mut green = 0;
                        let mut blue = 0;

                        for cube in set.trim().split(',') {
                            let parts = cube.split_whitespace().collect::<Vec<&str>>();
                            if parts.len() != 2 {
                                return None;
                            }
                            let coount = parts[0].parse::<u32>().ok()?;
                            match parts[1].chars().next()? {
                                'r' | 'R' => red += coount,
                                'g' | 'G' => green += coount,
                                'b' | 'B' => blue += coount,
                                _ => return None,
                            }
                        }
                        Some(CubeSet { red, green, blue })
                    })
                    .collect();
                Some(Game { id, sets })
            } else {
                None
            }
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let games = parse_input(input);
    let max_cubes = CubeSet {
        red: 12,
        green: 13,
        blue: 14,
    };
    Some(
        games
            .iter()
            .filter(|game| game_is_possible(game, &max_cubes))
            .map(|game| game.id)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let games = parse_input(input);
    Some(
        games
            .iter()
            .map(|game| {
                let mut min_red = 0;
                let mut min_green = 0;
                let mut min_blue = 0;

                for set in &game.sets {
                    min_red = std::cmp::max(min_red, set.red);
                    min_green = std::cmp::max(min_green, set.green);
                    min_blue = std::cmp::max(min_blue, set.blue);
                }

                min_red * min_green * min_blue
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
