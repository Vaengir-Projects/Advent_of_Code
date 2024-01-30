use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Node {
    key: String,
    elements: Vec<String>,
}

impl Default for Node {
    fn default() -> Self {
        Self::new()
    }
}

impl Node {
    pub fn new() -> Node {
        Node {
            key: String::new(),
            elements: Vec::new(),
        }
    }

    pub fn build(line: &str) -> Node {
        let key: String = line.split('=').nth(0).unwrap().trim().to_string();
        let maps: &str = line.split('=').nth(1).unwrap();
        let elements: Vec<String> = maps
            .trim()
            .trim_matches(|c| c == '(' || c == ')')
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();
        Node { key, elements }
    }
}

pub fn process_part1(input: &str) -> usize {
    let mut result: usize = 0;
    let navigation: Vec<char> = input.split("\n\n").next().unwrap().chars().collect();
    let mut nodes: Vec<Node> = Vec::new();
    for line in input.split("\n\n").nth(1).unwrap().lines() {
        nodes.push(Node::build(line));
    }
    let mut node = nodes.iter().find(|node| node.key == "AAA").unwrap();
    let mut navigation_cycle = navigation.iter().cycle();
    loop {
        if &node.key == "ZZZ" {
            break;
        }
        let nav = *navigation_cycle.next().unwrap();
        if nav == 'R' {
            node = nodes
                .iter()
                .find(|nod| nod.key == node.elements[1])
                .unwrap();
        } else {
            node = nodes
                .iter()
                .find(|nod| nod.key == node.elements[0])
                .unwrap();
        }
        result += 1;
    }
    result
}

pub fn process_part2(input: &str) -> usize {
    let mut result: usize = 0;
    let navigation: Vec<char> = input.split("\n\n").next().unwrap().chars().collect();
    let mut nodes: Vec<Node> = Vec::new();
    for line in input.split("\n\n").nth(1).unwrap().lines() {
        nodes.push(Node::build(line));
    }
    let mut starters: Vec<Node> = nodes
        .iter()
        .cloned()
        .filter(|nod| nod.key.ends_with('A'))
        .collect();
    let mut navigation_cycle = navigation.iter().cycle();
    loop {
        if starters.iter().all(|nod| nod.key.ends_with('Z')) {
            break;
        }
        let nav = *navigation_cycle.next().unwrap();
        let mut temp_starter: Vec<Node> = vec![Node::new(); starters.len()];
        for (i, node) in starters.iter().enumerate() {
            if nav == 'R' {
                temp_starter[i] = nodes
                    .iter()
                    .find(|nod| nod.key == node.elements[1])
                    .unwrap()
                    .clone()
            } else {
                temp_starter[i] = nodes
                    .iter()
                    .find(|nod| nod.key == node.elements[0])
                    .unwrap()
                    .clone()
            }
        }
        starters = temp_starter;
        result += 1;
    }
    result
}

#[derive(Debug)]
pub struct Map {
    instructions: Vec<Direction>,
    nodes: HashMap<String, NodeBetter>,
}

impl Map {
    fn parse(input: &str) -> Self {
        let mut split = input.split("\n\n");
        let instructions = split.next().unwrap();
        let nodes = split.next().unwrap();
        let instructions = instructions
            .chars()
            .map(|c| match c {
                'L' => Direction::Left,
                'R' => Direction::Right,
                _ => panic!("Invalid direction"),
            })
            .collect::<Vec<_>>();
        let nodes = nodes
            .lines()
            .map(NodeBetter::parse)
            .map(|n| (n.name.clone(), n))
            .collect::<HashMap<String, NodeBetter>>();
        Self {
            instructions,
            nodes,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
pub struct NodeBetter {
    name: String,
    left: String,
    right: String,
}

impl NodeBetter {
    fn parse(input: &str) -> Self {
        let mut name_and_connections = input.split(" = ");
        let name = name_and_connections.next().unwrap();
        let connections = name_and_connections.next().unwrap();
        let connections = connections
            .strip_prefix("(")
            .unwrap()
            .strip_suffix(")")
            .unwrap();
        let mut connections = connections.split(", ");
        let left = connections.next().unwrap();
        let right = connections.next().unwrap();
        Self {
            name: name.to_string(),
            left: left.to_string(),
            right: right.to_string(),
        }
    }
}

pub fn process_part2_better(input: &str) -> usize {
    let map = Map::parse(input);
    let starting_positions = map
        .nodes
        .iter()
        .filter(|(_, n)| n.name.ends_with('A'))
        .map(|(_, n)| n.name.clone())
        .collect::<Vec<_>>();
    let mut count = 0;
    let mut current_node_names = starting_positions;
    let mut intruction_iter = map.instructions.iter().cycle();
    while !current_node_names.iter().all(|n| n.ends_with('Z')) {
        let current_instruction = intruction_iter.next().unwrap();
        for current_node_name in current_node_names.iter_mut() {
            let current_node = map.nodes.get(current_node_name).unwrap();
            let next_node_name = match current_instruction {
                Direction::Left => current_node.left.clone(),
                Direction::Right => current_node.right.clone(),
            };
            *current_node_name = next_node_name;
        }
        count += 1;
    }
    count
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

pub fn process_part2_try2(input: &str) -> usize {
    let map = Map::parse(input);
    let starting_positions = map
        .nodes
        .iter()
        .filter(|(_, n)| n.name.ends_with('A'))
        .map(|(_, n)| n.name.clone())
        .collect::<Vec<_>>();
    let mut to_end_counts = HashMap::<String, usize>::new();
    for start in starting_positions.iter() {
        let mut current_node_name = start.clone();
        let mut count = 0;
        let mut instruction_iter = map.instructions.iter().cycle();
        while !current_node_name.ends_with('Z') {
            let current_instruction = instruction_iter.next().unwrap();
            let current_node = map.nodes.get(&current_node_name).unwrap();
            let next_node_name = match current_instruction {
                Direction::Left => current_node.left.clone(),
                Direction::Right => current_node.right.clone(),
            };
            current_node_name = next_node_name;
            count += 1;
        }
        to_end_counts.insert(start.clone(), count);
    }
    let counts = to_end_counts.values().cloned().collect::<Vec<_>>();
    let lcm = least_common_multiple(&counts);
    lcm
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT0: &str = "\
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    const INPUT1: &str = "\
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    const INPUT2: &str = "\
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn part1_works() {
        assert_eq!(process_part1(INPUT0), 2);
        assert_eq!(process_part1(INPUT1), 6);
    }

    #[test]
    fn part2_works() {
        assert_eq!(process_part2(INPUT2), 6);
    }

    #[test]
    fn part2better_works() {
        assert_eq!(process_part2_better(INPUT2), 6);
    }

    #[test]
    fn part2try2_works() {
        assert_eq!(process_part2_try2(INPUT2), 6);
    }
}
