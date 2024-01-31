#[derive(Debug, Clone)]
pub struct Node {
    value: i32,
    right: Option<Box<Node>>,
}

impl Node {
    fn parse(input: Vec<i32>) -> Vec<Self> {
        input
            .iter()
            .map(|num| Node {
                value: *num,
                right: None,
            })
            .collect()
    }

    fn build_structure(input_nodes: Vec<Node>) -> Vec<Self> {
        if input_nodes.iter().all(|x| x.value == 0) {
            return input_nodes;
        } else {
            let nodes = input_nodes[1..]
                .iter()
                .enumerate()
                .map(|(i, _)| Node {
                    value: input_nodes[i + 1].value - input_nodes[i].value,
                    right: Some(Box::new(input_nodes[i + 1].clone())),
                })
                .collect();
            Node::build_structure(nodes)
        }
    }

    fn get_next(input_nodes: Vec<Node>) -> i32 {
        let mut last = input_nodes.last().unwrap().clone();
        let mut next: Node = Node {
            value: 0,
            right: None,
        };
        last = *last.right.unwrap().clone();
        loop {
            if last.right.is_none() {
                return last.value + next.value;
            }
            next = Node {
                value: last.value + next.value,
                right: None,
            };
            last = *last.right.unwrap().clone();
        }
    }
}

pub fn process_part1(input: &str) -> i32 {
    let mut result: i32 = 0;
    let lines: Vec<&str> = input.lines().collect();
    for line in lines {
        let basic: Vec<i32> = line
            .split_whitespace()
            .into_iter()
            .map(|num| num.parse::<i32>().unwrap())
            .collect();
        let basic_nodes = Node::parse(basic);
        let nodes = Node::build_structure(basic_nodes);
        result += Node::get_next(nodes);
    }
    result
}

pub fn process_part2(_input: &str) -> usize {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "\
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn part1_works() {
        assert_eq!(process_part1(INPUT1), 114);
    }

    #[test]
    fn part2_works() {
        todo!();
    }
}
