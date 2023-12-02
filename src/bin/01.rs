advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .filter_map(|line| {
                let first_digit = line.chars().find(|c| c.is_ascii_digit())?;
                let second_digit = line.chars().rev().find(|c| c.is_ascii_digit())?;
                format!("{}{}", first_digit, second_digit)
                    .parse::<u32>()
                    .ok()
            })
            .sum::<u32>(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    fn match_digit_prefix(line: &str, digit_str: &str, digit_value: u32) -> Option<u32> {
        if line.starts_with(digit_str) {
            Some(digit_value)
        } else {
            None
        }
    }

    fn extract_digits(line: &str) -> Vec<u32> {
        line.chars()
            .enumerate()
            .filter_map(|(idx, ch)| {
                ch.to_digit(10).or_else(|| match ch {
                    'o' => match_digit_prefix(&line[idx..], "one", 1),
                    't' => match_digit_prefix(&line[idx..], "two", 2)
                        .or_else(|| match_digit_prefix(&line[idx..], "three", 3)),
                    'f' => match_digit_prefix(&line[idx..], "four", 4)
                        .or_else(|| match_digit_prefix(&line[idx..], "five", 5)),
                    's' => match_digit_prefix(&line[idx..], "six", 6)
                        .or_else(|| match_digit_prefix(&line[idx..], "seven", 7)),
                    'e' => match_digit_prefix(&line[idx..], "eight", 8),
                    'n' => match_digit_prefix(&line[idx..], "nine", 9),
                    _ => None,
                })
            })
            .collect()
    }

    let mut sum = 0;
    for line in input.lines() {
        let digits = extract_digits(line);
        if digits.is_empty() {
            continue;
        }
        sum += digits[0] * 10 + digits.last().unwrap();
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        assert_eq!(part_one(input), Some(142));
    }
    #[test]
    fn test_part_two() {
        let input = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";
        assert_eq!(part_two(input), Some(281));
    }
}
