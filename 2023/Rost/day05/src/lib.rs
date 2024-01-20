use std::{f32::INFINITY, cell::RefCell, iter::from_fn};

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

pub fn process_part1_better() -> String {
    const SECTIONS: usize = 7;
    let input = include_bytes!("../input.txt");
    let mut lines = input.split(|b| b == &b'\n').skip(2);
    let maps: Vec<Vec<(std::ops::Range<u64>, u64)>> = (0..SECTIONS)
        .map(|_| {
            (&mut lines)
                .skip(1)
                .take_while(|line| !line.is_empty())
                .map(|line| {
                    let mut entry = line
                        .splitn(3, |b| b == &b' ')
                        .map(|n| atoi::atoi(n).unwrap());
                    let el: [_; 3] = std::array::from_fn(|_| entry.next().unwrap());
                    (el[1]..el[1] + el[2], el[0])
                })
                .collect()
        })
        .collect();

    input[SECTIONS..input.iter().position(|b| b == &b'\n').unwrap()]
        .split(|b| b == &b' ')
        .flat_map(atoi::atoi)
        .map(|seed| {
            maps.iter().fold(seed, |seed, map| {
                map.iter()
                    .find(|(range, _)| range.contains(&seed))
                    .map(|(range, to)| to + seed - range.start)
                    .unwrap_or(seed)
            })
        })
        .min()
        .unwrap()
        .to_string()
}

pub fn process_part2(input: &str) -> usize {
    let sections: Vec<&str> = input.split("\n\n").collect();
    let seed_range: Vec<usize> = sections[0]
        .split_whitespace()
        .flat_map(|x| x.parse::<usize>())
        .collect();
    let seed_range: Vec<(usize, usize)> = seed_range
        .chunks(2)
        .filter_map(|chunk| {
            if chunk.len() == 2 {
                Some((chunk[0], chunk[1]))
            } else {
                None
            }
        })
        .collect();
    let mut seeds: Vec<usize> = Vec::new();
    for (start, range) in seed_range {
        let mut current = start;
        while current < start + range {
            seeds.push(current);
            current += 1;
        }
    }
    let mut result: usize = INFINITY as usize;
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
        if seed < result {
            result = seed;
        }
        dbg!(seed, result);
    }
    result
}

pub fn process_part2_better() -> String {
    const SECTIONS: usize = 7;
    let input = include_bytes!("../input.txt");

    let mut seeds = input[SECTIONS..input.iter().position(|b| b == &b'\n').unwrap()]
        .split(|b| b == &b' ')
        .flat_map(atoi::atoi::<u64>);
    let mut lines = input.split(|b| b == &b'\n').skip(2);

    let maps: Vec<Vec<(std::ops::Range<u64>, u64)>> = (0..SECTIONS)
        .map(|_| {
            let mut map = (&mut lines)
                .skip(1)
                .take_while(|line| !line.is_empty())
                .map(|line| {
                    let mut entry = line
                        .splitn(3, |b| b == &b' ')
                        .map(|n| atoi::atoi(n).unwrap());
                    let el: [_; 3] = std::array::from_fn(|_| entry.next().unwrap());
                    (el[1]..el[1] + el[2], el[0])
                })
                .collect::<Vec<_>>();
            map.sort_by_key(|(range, _)| range.start);
            map
        })
        .collect();

    from_fn(|| seeds.next().zip(seeds.next()))
        .map(|(start, len)| start..start + len)
        .flat_map(|range| {
            maps.iter().fold(vec![range], |ranges, map| {
                ranges
                    .into_iter()
                    .flat_map(|base| {
                        let base_cell = RefCell::new(base);
                        map.iter()
                            .take_while(|_| !base_cell.borrow().is_empty())
                            .fold(Vec::with_capacity(6), |mut from, (to, n)| {
                                let mut base = base_cell.borrow_mut();
                                if base.start < to.start {
                                    from.push(base.start..(base.end.min(to.start)));
                                    base.start = to.start;
                                }

                                let len = base.end.min(to.end).saturating_sub(base.start);
                                if len > 0 {
                                    let to = *n + base.start - to.start;
                                    from.push(to..to + len);
                                    base.start += len;
                                }
                                from
                            })
                    })
                    .collect()
            })
        })
        .map(|range| range.start)
        .min()
        .unwrap()
        .to_string()
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
        assert_eq!(process_part2(INPUT1), 46);
    }
}
