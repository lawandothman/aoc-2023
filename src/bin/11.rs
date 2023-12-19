advent_of_code::solution!(11);

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Tile {
    Galaxy,
    Empty,
}

struct Coordinate {
    row: usize,
    col: usize,
}

impl Coordinate {
    fn distance(&self, other: &Self) -> usize {
        self.row.abs_diff(other.row) + self.col.abs_diff(other.col)
    }
}

fn parse_input(input: &str) -> Vec<Vec<Tile>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Tile::Empty,
                    '#' => Tile::Galaxy,
                    c => panic!("Invalid input {}", c),
                })
                .collect()
        })
        .collect()
}

fn galaxy_coordinates(grid: &[Vec<Tile>], expansion: usize) -> Vec<Coordinate> {
    let empty_rows = empty_rows(grid);
    let empty_cols = empty_cols(grid);

    let mut galaxies = Vec::new();
    let mut current_row = 0;
    let mut current_col = 0;

    for (row_idx, row) in grid.iter().enumerate() {
        if empty_rows.contains(&row_idx) {
            current_row += expansion;
            continue;
        }
        for (col_idx, cell) in row.iter().enumerate() {
            if empty_cols.contains(&col_idx) {
                current_col += expansion;
                continue;
            }

            if *cell == Tile::Galaxy {
                galaxies.push(Coordinate {
                    row: current_row,
                    col: current_col,
                });
            }
            current_col += 1;
        }
        current_col = 0;
        current_row += 1;
    }

    galaxies
}

fn empty_rows(grid: &[Vec<Tile>]) -> Vec<usize> {
    grid.iter()
        .enumerate()
        .filter_map(|(idx, row)| {
            if !row.contains(&Tile::Galaxy) {
                Some(idx)
            } else {
                None
            }
        })
        .collect()
}

fn empty_cols(grid: &[Vec<Tile>]) -> Vec<usize> {
    let mut cols: Vec<Vec<Tile>> = vec![vec![Tile::Empty; grid.len()]; grid[0].len()];
    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, c) in row.iter().enumerate() {
            cols[col_idx][row_idx] = *c;
        }
    }

    empty_rows(&cols)
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid = parse_input(input);
    let galaxies = galaxy_coordinates(&grid, 2);
    let mut sum = 0;
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            sum += galaxies[i].distance(&galaxies[j]);
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid = parse_input(input);
    let galaxies = galaxy_coordinates(&grid, 1_000_000);
    let mut sum = 0;
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            sum += galaxies[i].distance(&galaxies[j]);
        }
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(82000210));
    }
}
