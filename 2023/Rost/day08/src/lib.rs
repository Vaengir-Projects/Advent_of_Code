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

pub fn process_part2(_input: &str) -> usize {
    todo!();
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

    #[test]
    fn part1_works() {
        assert_eq!(process_part1(INPUT0), 2);
        assert_eq!(process_part1(INPUT1), 6);
    }

    #[test]
    fn part2_works() {
        todo!();
    }
}
