advent_of_code::solution!(9);

fn generate_difference_sequence(history: &[i32]) -> Vec<i32> {
    history.windows(2).map(|pair| pair[1] - pair[0]).collect()
}

fn all_zeroes(history: &[i32]) -> bool {
    history.iter().all(|&x| x == 0)
}

pub fn part_one(input: &str) -> Option<i32> {
    input
        .lines()
        .map(|line| {
            let mut history: Vec<Vec<i32>> = vec![line
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect()];

            while !all_zeroes(history.last().unwrap()) {
                let next_sequence = generate_difference_sequence(history.last().unwrap());
                history.push(next_sequence);
            }

            let mut next_value = 0;
            for i in (0..history.len()).rev() {
                next_value += history[i].last().unwrap_or(&0);
            }

            next_value
        })
        .sum::<i32>()
        .into()
}

pub fn part_two(input: &str) -> Option<i32> {
    input
        .lines()
        .map(|line| {
            let nums: Vec<i32> = line
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .rev()
                .collect();

            find_next_value(&nums)
        })
        .sum::<i32>()
        .into()
}

fn find_next_value(nums: &[i32]) -> i32 {
    let mut all_zeroes = true;
    let diffs = nums
        .windows(2)
        .map(|window| {
            let diff = window[1] - window[0];
            all_zeroes &= diff == 0;
            diff
        })
        .collect::<Vec<_>>();

    let end = *nums.last().unwrap();
    if all_zeroes {
        end
    } else {
        end + find_next_value(&diffs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
