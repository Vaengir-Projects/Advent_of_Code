pub fn process_part1(input: &str) -> usize {
    let mut sum: usize = 0;
    let mut rules: Vec<(usize, usize)> = Vec::new();
    let mut sections: Vec<Vec<usize>> = Vec::new();

    for line in input.lines() {
        let ordering: Vec<&str> = line.split('|').collect();
        if ordering.len() < 2 && !ordering.is_empty() && !ordering[0].is_empty() {
            sections.push(
                ordering[0]
                    .split(',')
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect(),
            );
        } else if ordering.len() == 2 {
            let x: Vec<usize> = ordering
                .iter()
                .map(|u| u.parse::<usize>().unwrap())
                .collect();
            rules.push((x[0], x[1]));
        }
    }
    for sec in sections {
        if check_valid1(&sec, rules.clone()) {
            sum += sec[sec.len() / 2];
        }
    }

    sum
}

fn check_valid1(sec: &[usize], rules: Vec<(usize, usize)>) -> bool {
    let mut valid = true;
    for update in sec {
        let relevant_rules: Vec<(usize, usize)> = rules
            .clone()
            .into_iter()
            .filter(|(x, y)| x == update || y == update)
            .collect();
        for relevant_rule in relevant_rules {
            let pos_0 = sec.iter().position(|&x| x == relevant_rule.0);
            let pos_1 = sec.iter().position(|&x| x == relevant_rule.1);
            if let (Some(p0), Some(p1)) = (pos_0, pos_1) {
                if p0 > p1 {
                    valid = false;
                }
            }
        }
    }

    valid
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn part1_works() {
        assert_eq!(process_part1(INPUT1), 143);
    }
}
