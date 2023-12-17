advent_of_code::solution!(7);

#[derive(Copy, Clone, PartialEq, Eq)]
enum JTreatment {
    Jack,
    Joker,
}

struct Hand {
    strength: usize,
    bid: usize,
}

impl Hand {
    fn new<T>(hand: &str, bid: usize, j_treatment: JTreatment, symbol_to_rank: &T) -> Self
    where
        T: Fn(u8) -> Result<u8, String>,
    {
        const RANK_COUNT: usize = 13;
        const JOKER_RANK_INDEX: usize = 0;
        const PRIMARY_MULTIPLIER: usize = 13 * 13 * 13 * 13 * 13;

        let mut card_counts = [0; RANK_COUNT];
        let mut strength = 0;
        let mut joker_count = 0;

        for symbol in hand.bytes() {
            let rank_index = symbol_to_rank(symbol).expect("Invalid card symbol");
            strength *= RANK_COUNT;
            if j_treatment == JTreatment::Joker && rank_index == JOKER_RANK_INDEX as u8 {
                joker_count += 1;
            } else {
                strength += rank_index as usize;
                card_counts[rank_index as usize] += 1;
            }
        }

        let (most_of_a_rank, second_most_of_a_rank) =
            card_counts.iter().fold((0, 0), |(most, second), &count| {
                if count > most {
                    (count, most)
                } else if count > second {
                    (most, count)
                } else {
                    (most, second)
                }
            });

        let total_most = most_of_a_rank + joker_count;
        strength += PRIMARY_MULTIPLIER * (3 * total_most + second_most_of_a_rank);

        Self { strength, bid }
    }
}

fn symbol_to_rank_index_with_jack(symbol: u8) -> Result<u8, String> {
    match symbol {
        b'2'..=b'9' => Ok(symbol - b'2'),
        b'T' => Ok(8),
        b'J' => Ok(9),
        b'Q' => Ok(10),
        b'K' => Ok(11),
        b'A' => Ok(12),
        _ => Err(format!("Unrecognized card symbol '{}'.", symbol as char)),
    }
}

fn symbol_to_rank_index_with_joker(symbol: u8) -> Result<u8, String> {
    match symbol {
        b'J' => Ok(0),
        b'2'..=b'9' => Ok(symbol - b'1'),
        b'T' => Ok(9),
        b'Q' => Ok(10),
        b'K' => Ok(11),
        b'A' => Ok(12),
        _ => Err(format!("Unrecognized card symbol '{}'.", symbol as char)),
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    input
        .lines()
        .map(|l| {
            let (hand, bid) = l.split_once(' ')?;
            let bid: usize = bid.parse().ok()?;
            Some(Hand::new(
                hand,
                bid,
                JTreatment::Jack,
                &symbol_to_rank_index_with_jack,
            ))
        })
        .collect::<Option<Vec<_>>>()
        .map(|mut hands| {
            hands.sort_by_key(|hand| hand.strength);
            hands
                .iter()
                .enumerate()
                .map(|(i, hand)| hand.bid * (i + 1))
                .sum()
        })
}

pub fn part_two(input: &str) -> Option<usize> {
    input
        .lines()
        .map(|l| {
            let (hand, bid) = l.split_once(' ')?;
            let bid: usize = bid.parse().ok()?;
            Some(Hand::new(
                hand,
                bid,
                JTreatment::Joker,
                &symbol_to_rank_index_with_joker,
            ))
        })
        .collect::<Option<Vec<_>>>()
        .map(|mut hands| {
            hands.sort_by_key(|hand| hand.strength);
            hands
                .iter()
                .enumerate()
                .map(|(i, hand)| hand.bid * (i + 1))
                .sum()
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
