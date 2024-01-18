pub fn process_part1(input: &str) -> usize {
    let sections: Vec<&str> = input.split("\n\n").collect();
    let seeds: Vec<usize> = sections[0]
        .split_whitespace()
        .flat_map(|x| x.parse::<usize>())
        .collect();
    let mut results: Vec<usize> = Vec::new();
    for mut seed in seeds {
        let maps: Vec<Vec<Vec<usize>>> = sections[1..]
            .iter()
            .map(|map| {
                map.lines()
                    .map(|line| {
                        line.split_whitespace()
                            .flat_map(|x| x.parse::<usize>())
                            .collect()
                    })
                    .collect()
            })
            .collect();
        for mut map in maps {
            map.retain(|x| !x.is_empty());
            for numbers in map {
                if seed >= numbers[1] && seed <= numbers[1] + numbers[2] {
                    seed = seed + numbers[0] - numbers[1];
                    break;
                }
            }
        }
        results.push(seed);
    }
    *results.iter().min().unwrap()
}

pub fn process_part2(input: &str) -> usize {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn part1_works() {
        assert_eq!(process_part1(INPUT1), 35);
    }

    #[test]
    fn part2_works() {
        todo!();
    }
}
