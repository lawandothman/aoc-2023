use std::collections::HashMap;

advent_of_code::solution!(8);
#[derive(Debug)]
struct Map {
    instructions: Vec<Direction>,
    nodes: HashMap<String, Node>,
}

impl Map {
    fn parse(input: &str) -> Self {
        let mut sections = input.split("\n\n");
        let instructions = sections.next().unwrap();
        let nodes = sections.next().unwrap();

        let instructions = instructions.chars().map(Direction::from).collect();

        let nodes = nodes
            .lines()
            .map(Node::parse)
            .map(|n| (n.name.clone(), n))
            .collect();

        Self {
            instructions,
            nodes,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}
impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!("Invalid direction: {}", c),
        }
    }
}
#[derive(Debug)]
struct Node {
    name: String,
    left: String,
    right: String,
}

impl Node {
    fn parse(line: &str) -> Self {
        let mut parts = line.split(" = ");
        let name = parts.next().unwrap().to_string();
        let (left, right) = parts
            .next()
            .unwrap()
            .trim_matches(|p| p == '(' || p == ')')
            .split_once(", ")
            .unwrap();
        Self {
            name,
            left: left.to_string(),
            right: right.to_string(),
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = Map::parse(input);
    let mut count = 0;
    let mut current = "AAA".to_string();
    let mut iter = map.instructions.iter().cycle();

    while current != "ZZZ" {
        let instruction = iter.next().unwrap();
        let node = map.nodes.get(&current).unwrap();
        current = match instruction {
            Direction::Left => &node.left,
            Direction::Right => &node.right,
        }
        .to_string();
        count += 1;
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<usize> {
    let map = Map::parse(input);

    let starts = map
        .nodes
        .iter()
        .filter(|(_, n)| n.name.ends_with('A'))
        .map(|(_, n)| n.name.clone())
        .collect::<Vec<_>>();

    let mut counts = HashMap::<String, usize>::new();

    for start in starts.iter() {
        let mut current = start.clone();
        let mut count = 0;
        let mut iter = map.instructions.iter().cycle();

        while !current.ends_with('Z') {
            let instruction = iter.next().unwrap();
            let current_node = map.nodes.get(&current).unwrap();

            let next = match instruction {
                Direction::Left => current_node.left.clone(),
                Direction::Right => current_node.right.clone(),
            };

            current = next;
            count += 1;
        }

        counts.insert(start.clone(), count);
    }

    let counts = counts.values().cloned().collect::<Vec<_>>();

    Some(least_common_multiple(&counts))
}

fn least_common_multiple(nums: &[usize]) -> usize {
    let mut result = 1;
    for &num in nums {
        result = num * result / gcd(num, result);
    }
    result
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }

    gcd(b, a % b)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(
            "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
",
        );
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(
            "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)",
        );
        assert_eq!(result, Some(6));
    }
}
