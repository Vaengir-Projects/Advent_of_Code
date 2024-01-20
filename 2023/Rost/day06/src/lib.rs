#[derive(Debug, Clone, Copy)]
struct Race {
    time: usize,
    distance: usize,
}

impl Race {
    fn new(time: usize, distance: usize) -> Race {
        Race { time, distance }
    }
}

pub fn process_part1(input: &str) -> usize {
    let result: usize = 0;
    let times: Vec<usize> = input
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .flat_map(|part| part.parse::<usize>())
        .collect();
    let distances: Vec<usize> = input
        .lines()
        .nth(1)
        .unwrap()
        .split_whitespace()
        .flat_map(|part| part.parse::<usize>())
        .collect();
    let mut races: Vec<Race> = Vec::new();
    for (i, _) in times.iter().enumerate() {
        races.push(Race::new(times[i], distances[i]));
    }
    dbg!(races);
    result
}

pub fn process_part2(_input: &str) -> usize {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "\
Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn part1_works() {
        assert_eq!(process_part1(INPUT1), 288);
    }

    #[test]
    fn part2_works() {
        todo!();
    }
}
