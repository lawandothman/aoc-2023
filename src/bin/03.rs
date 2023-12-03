use std::{collections::HashSet, ops::Range};

advent_of_code::solution!(3);

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Part {
    line: usize,
    range: Range<usize>,
    number: u64,
}

struct Symbol {
    parts: HashSet<Part>,
}

fn parse_parts(input: &str) -> Vec<Vec<Part>> {
    input
        .lines()
        .enumerate()
        .map(|(line_idx, line)| {
            let mut parts = Vec::new();
            let mut current_number = String::new();
            let mut start_index = 0;

            for (index, ch) in line.char_indices() {
                if ch.is_ascii_digit() {
                    if current_number.is_empty() {
                        start_index = index;
                    }
                    current_number.push(ch);
                } else if !current_number.is_empty() {
                    if let Ok(number) = current_number.parse::<u64>() {
                        parts.push(Part {
                            line: line_idx,
                            range: start_index..index,
                            number,
                        });
                    }
                    current_number.clear();
                }
            }

            // Handle any trailing number
            if !current_number.is_empty() {
                if let Ok(number) = current_number.parse::<u64>() {
                    parts.push(Part {
                        line: line_idx,
                        range: start_index..line.len(),
                        number,
                    });
                }
            }

            parts
        })
        .collect()
}

fn find_adjacent_parts(x: usize, y: usize, parts: &[Vec<Part>]) -> HashSet<Part> {
    const DIRECTIONS: &[(isize, isize); 8] = &[
        (-1, 1),
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
        (-1, 0),
    ];

    let mut adjacent_parts = HashSet::new();
    for &(dx, dy) in DIRECTIONS {
        let nx = x as isize + dx;
        let ny = y as isize + dy;

        if nx >= 0 && ny >= 0 && (ny as usize) < parts.len() {
            let line_parts = &parts[ny as usize];
            for part in line_parts {
                if part.range.contains(&(nx as usize)) {
                    adjacent_parts.insert(part.clone());
                }
            }
        }
    }

    adjacent_parts
}

fn find_symbols<'s>(input: &'s str, parts: &'s [Vec<Part>]) -> impl Iterator<Item = Symbol> + 's {
    input.lines().enumerate().flat_map(move |(line_idx, line)| {
        line.chars().enumerate().filter_map(move |(idx, c)| {
            if c.is_ascii_digit() || c == '.' {
                // Skip digits and dots
                return None;
            }
            let adjacent_parts = find_adjacent_parts(idx, line_idx, parts);
            Some(Symbol {
                parts: adjacent_parts,
            })
        })
    })
}
pub fn part_one(input: &str) -> Option<u64> {
    let parts = parse_parts(input);
    let relevant_parts = find_symbols(input, &parts)
        .flat_map(|symbol| symbol.parts.into_iter())
        .collect::<HashSet<_>>();

    Some(relevant_parts.iter().map(|part| part.number).sum::<u64>())
}

pub fn part_two(input: &str) -> Option<u64> {
    let parts = parse_parts(input);
    let gears = find_symbols(input, &parts)
        .filter(|symbol| symbol.parts.len() == 2) // Gears are adjacent to 2 parts
        .map(|symbol| symbol.parts.iter().map(|part| part.number).product::<u64>()) // Multiply the 2 numbers
        .sum();
    Some(gears)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
