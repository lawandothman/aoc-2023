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
    let cards: Vec<(Vec<u32>, Vec<u32>)> = input
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
            (winning_numbers, player_numbers)
        })
        .collect();

    let mut result = 0;
    let mut to_process: Vec<(usize, usize)> = (0..cards.len()).map(|i| (i, 1)).collect();

    while let Some((card_idx, copies)) = to_process.pop() {
        result += copies;
        let (ref winning_numbers, ref player_numbers) = cards[card_idx];
        let matches = player_numbers
            .iter()
            .filter(|&n| winning_numbers.contains(n))
            .count();

        if card_idx + matches < cards.len() {
            for next_card in card_idx + 1..=card_idx + matches {
                to_process.push((next_card, copies));
            }
        }
    }
    Some(result as u32)
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
        assert_eq!(result, Some(30));
    }
}
