advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let result = input
        .lines()
        .map(|line| {
            let numbers_part = line.split_once(": ").unwrap().1;
            let parts: Vec<&str> = numbers_part.split(" | ").collect();
            let winning_numbers: Vec<u32> = parts[0]
                .split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect();
            let player_numbers: Vec<u32> = parts[1]
                .split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect();

            player_numbers
                .iter()
                .filter(|&n| winning_numbers.contains(n))
                .fold(0, |acc, _| if acc == 0 { 1 } else { acc * 2 })
        })
        .sum::<u32>();
    Some(result)
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
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
