pub fn process_part1(input: &str) -> usize {
    let (left, right) = parse_input1(input);
    let mut sum: usize = 0;
    for (i, _) in left.iter().enumerate() {
        sum += calculate_distance(left[i], right[i]);
    }
    sum
}

fn parse_input1(input: &str) -> (Vec<usize>, Vec<usize>) {
    let mut left_nums: Vec<usize> = Vec::new();
    let mut right_nums: Vec<usize> = Vec::new();
    let lines = input.lines();
    for line in lines {
        let mut nums = line.split_whitespace();
        left_nums.push(nums.next().unwrap().parse::<usize>().unwrap());
        right_nums.push(nums.next().unwrap().parse::<usize>().unwrap());
    }
    left_nums.sort();
    right_nums.sort();

    (left_nums, right_nums)
}

fn calculate_distance(l: usize, r: usize) -> usize {
    l.abs_diff(r)
}

pub fn process_part2(input: &str) -> usize {
    let (left, right) = parse_input1(input);
    let mut sum: usize = 0;
    for l in left {
        sum += l * right.iter().filter(|r| **r == l).count();
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn part1_works() {
        assert_eq!(process_part1(INPUT1), 11);
    }

    #[test]
    fn part2_works() {
        assert_eq!(process_part2(INPUT1), 31);
    }
}
