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
    let mut result: usize = 1;
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
    for race in races.iter() {
        let mut wins: usize = 0;
        for i in 1..race.time {
            if i * (race.time - i) > race.distance {
                wins += 1;
            }
        }
        result *= wins;
    }
    result
}

pub fn process_part2(input: &str) -> usize {
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
    let time = times
        .iter()
        .map(|&num| num.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse::<usize>()
        .unwrap();
    let distance = distances
        .iter()
        .map(|&num| num.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse::<usize>()
        .unwrap();
    let mut min: usize = 0;
    let mut max: usize = 0;
    for i in 1..time {
        if i * (time - i) > distance {
            min = i;
            break;
        }
    }
    for i in (1..time).rev() {
        if i * (time - i) > distance {
            max = i;
            break;
        }
    }
    max - min + 1
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
        assert_eq!(process_part2(INPUT1), 71503);
    }
}
