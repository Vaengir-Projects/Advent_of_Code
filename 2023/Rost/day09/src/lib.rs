#[derive(Debug, Clone)]
pub struct Node1 {
    value: i32,
    right: Option<Box<Node1>>,
}

impl Node1 {
    fn parse(input: Vec<i32>) -> Vec<Self> {
        input
            .iter()
            .map(|num| Node1 {
                value: *num,
                right: None,
            })
            .collect()
    }

    fn build_structure(input_nodes: Vec<Node1>) -> Vec<Self> {
        if input_nodes.iter().all(|x| x.value == 0) {
            input_nodes
        } else {
            let nodes = input_nodes[1..]
                .iter()
                .enumerate()
                .map(|(i, _)| Node1 {
                    value: input_nodes[i + 1].value - input_nodes[i].value,
                    right: Some(Box::new(input_nodes[i + 1].clone())),
                })
                .collect();
            Node1::build_structure(nodes)
        }
    }

    fn get_next(input_nodes: Vec<Node1>) -> i32 {
        let mut last = input_nodes.last().unwrap().clone();
        let mut next: Node1 = Node1 {
            value: 0,
            right: None,
        };
        last = *last.right.unwrap().clone();
        loop {
            if last.right.is_none() {
                return last.value + next.value;
            }
            next = Node1 {
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
            .map(|num| num.parse::<i32>().unwrap())
            .collect();
        let basic_nodes = Node1::parse(basic);
        let nodes = Node1::build_structure(basic_nodes);
        result += Node1::get_next(nodes);
    }
    result
}

#[derive(Debug, Clone)]
struct Node2 {
    value: i32,
    left: Option<Box<Node2>>,
}

impl Node2 {
    fn parse(input: Vec<i32>) -> Vec<Self> {
        input
            .iter()
            .map(|num| Node2 {
                value: *num,
                left: None,
            })
            .collect()
    }

    fn build_structure(input_nodes: Vec<Node2>) -> Vec<Self> {
        if input_nodes.iter().all(|x| x.value == 0) {
            input_nodes
        } else {
            let nodes = input_nodes[1..]
                .iter()
                .enumerate()
                .map(|(i, _)| Node2 {
                    value: input_nodes[i + 1].value - input_nodes[i].value,
                    left: Some(Box::new(input_nodes[i].clone())),
                })
                .collect();
            Node2::build_structure(nodes)
        }
    }

    fn get_previous(input_nodes: Vec<Node2>) -> i32 {
        let mut first = input_nodes.first().unwrap().clone();
        let mut prev: Node2 = Node2 {
            value: 0,
            left: None,
        };
        first = *first.left.unwrap().clone();
        loop {
            if first.left.is_none() {
                return first.value - prev.value;
            }
            prev = Node2 {
                value: first.value - prev.value,
                left: None,
            };
            first = *first.left.unwrap().clone();
        }
    }
}

pub fn process_part2(input: &str) -> i32 {
    let mut result: i32 = 0;
    let lines: Vec<&str> = input.lines().collect();
    for line in lines {
        let basic: Vec<i32> = line
            .split_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect();
        let basic_nodes = Node2::parse(basic);
        let nodes = Node2::build_structure(basic_nodes);
        result += Node2::get_previous(nodes);
    }
    result
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
        assert_eq!(process_part2(INPUT1), 2);
    }
}
