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

pub fn process_part2(input: &str) -> usize {
    let mut sum: usize = 0;
    let lines = input.lines();
    for line in lines {
        let levels: Vec<usize> = line
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        let up_or_down: Option<bool> = None;
        if check_levels(&levels, up_or_down, false) {
            sum += 1;
        }
    }

    sum
}

fn check_levels(levels: &[usize], mut up_or_down: Option<bool>, bad: bool) -> bool {
    for (i, _) in levels.iter().enumerate().take(levels.len() - 1) {
        if (levels[i].abs_diff(levels[i + 1]) > 3) || (levels[i] == levels[i + 1]) {
            if bad {
                return false;
            }
            let mut temp_vec1;
            let mut temp_vec2;
            if i == 1 {
                up_or_down = None;
                temp_vec1 = levels.to_vec();
                let _ = temp_vec1.remove(i - 1);
                temp_vec2 = levels.to_vec();
                let _ = temp_vec2.remove(i);
            } else {
                temp_vec1 = levels.to_vec();
                let _ = temp_vec1.remove(i);
                temp_vec2 = levels.to_vec();
                let _ = temp_vec2.remove(i + 1);
            }
            return check_levels(&temp_vec1, up_or_down, true)
                || check_levels(&temp_vec2, up_or_down, true);
        }
        match up_or_down {
            Some(true) => {
                if levels[i] <= levels[i + 1] {
                    if bad {
                        return false;
                    }
                    let mut temp_vec1;
                    let mut temp_vec2;
                    if i == 1 {
                        up_or_down = None;
                        temp_vec1 = levels.to_vec();
                        let _ = temp_vec1.remove(i - 1);
                        temp_vec2 = levels.to_vec();
                        let _ = temp_vec2.remove(i);
                    } else {
                        temp_vec1 = levels.to_vec();
                        let _ = temp_vec1.remove(i);
                        temp_vec2 = levels.to_vec();
                        let _ = temp_vec2.remove(i + 1);
                    }
                    return check_levels(&temp_vec1, up_or_down, true)
                        || check_levels(&temp_vec2, up_or_down, true);
                }
            }
            Some(false) => {
                if levels[i] >= levels[i + 1] {
                    if bad {
                        return false;
                    }
                    let mut temp_vec1;
                    let mut temp_vec2;
                    if i == 1 {
                        up_or_down = None;
                        temp_vec1 = levels.to_vec();
                        let _ = temp_vec1.remove(i - 1);
                        temp_vec2 = levels.to_vec();
                        let _ = temp_vec2.remove(i);
                    } else {
                        temp_vec1 = levels.to_vec();
                        let _ = temp_vec1.remove(i);
                        temp_vec2 = levels.to_vec();
                        let _ = temp_vec2.remove(i + 1);
                    }
                    return check_levels(&temp_vec1, up_or_down, true)
                        || check_levels(&temp_vec2, up_or_down, true);
                }
            }
            None => {
                up_or_down = Some(levels[i] > levels[i + 1]);
            }
        }
    }

    true
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

    const INPUT24: &[usize] = &[24, 21, 22, 24, 25, 26, 27];

    const INPUT131: &[usize] = &[35, 34, 31, 34, 30];

    const INPUT148: &[usize] = &[80, 82, 81, 80, 77, 76, 73, 72];

    const INPUT287: &[usize] = &[5, 4, 8, 9, 12, 13];

    #[test]
    fn part1_works() {
        assert_eq!(process_part1(INPUT1), 2);
    }

    #[test]
    fn part2_works() {
        assert_eq!(process_part2(INPUT1), 4);
    }

    #[test]
    fn line_24_works() {
        assert!(check_levels(INPUT24, None, false));
    }

    #[test]
    fn line_131_works() {
        assert!(check_levels(INPUT131, None, false));
    }

    #[test]
    fn line_148_works() {
        assert!(check_levels(INPUT148, None, false));
    }

    #[test]
    fn line_287_works() {
        assert!(check_levels(INPUT287, None, false));
    }
}
