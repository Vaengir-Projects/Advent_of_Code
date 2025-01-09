pub fn process_part1(input: &str) -> usize {
    let mut sum: usize = 0;
    let matches: Vec<(usize, &str)> = input.match_indices("mul(").collect();
    for (u, _s) in matches {
        let subs = &input[u + 4..];
        let nums = subs.split(')').collect::<Vec<_>>()[0];
        let nums: Vec<Result<usize, _>> = nums.split(',').map(|s| s.parse::<usize>()).collect();
        if nums.len() < 2 {
            continue;
        }
        match (nums[0].clone(), nums[1].clone()) {
            (Ok(v1), Ok(v2)) => sum += v1 * v2,
            _ => continue,
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn part1_works() {
        assert_eq!(process_part1(INPUT1), 161);
    }
}
