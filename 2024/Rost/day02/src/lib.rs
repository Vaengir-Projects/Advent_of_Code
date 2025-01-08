pub fn process_part1(input: &str) -> usize {
    let mut sum: usize = 0;
    let lines = input.lines();
    for line in lines {
        let levels: Vec<usize> = line
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        let mut up_or_down: Option<bool> = None;
        let mut runs: usize = 1;
        for (i, _) in levels.iter().enumerate().take(levels.len() - 1) {
            if (levels[i].abs_diff(levels[i + 1]) > 3) || (levels[i] == levels[i + 1]) {
                break;
            }
            match up_or_down {
                Some(true) => {
                    if levels[i] <= levels[i + 1] {
                        break;
                    }
                }
                Some(false) => {
                    if levels[i] >= levels[i + 1] {
                        break;
                    }
                }
                None => {
                    up_or_down = Some(levels[i] > levels[i + 1]);
                }
            }
            runs += 1;
            if runs == levels.len() {
                sum += 1;
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn part1_works() {
        assert_eq!(process_part1(INPUT1), 2);
    }
}
