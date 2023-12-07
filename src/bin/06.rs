advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines();

    let times: Vec<u32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().expect("Invalid time"))
        .collect();

    let distances: Vec<u32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().expect("Invalid distance"))
        .collect();

    let races: Vec<(u32, u32)> = times.into_iter().zip(distances).collect();

    Some(
        races
            .iter()
            .map(|&(t, d)| {
                (1..t)
                    .filter(|&hold_time| (t - hold_time) * hold_time > d)
                    .count() as u32
            })
            .product::<u32>(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines();

    let time: u64 = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .collect::<String>()
        .parse()
        .unwrap();
    println!("time: {}", time);
    let distance: u64 = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .collect::<String>()
        .parse()
        .unwrap();

    let mut count = 0;
    for hold_time in 1..time {
        if hold_time * (time - hold_time) > distance {
            count += 1;
        }
    }

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
