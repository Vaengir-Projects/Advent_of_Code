use std::cmp;

pub fn process_part1(input: &str) -> u32 {
    let mut result: u32 = 0;
    let lines = input.lines();
    for line in lines {
        let values: Vec<u32> = line
            .chars()
            .filter_map(|c| {
                let num: u32 = c.to_digit(36).unwrap();
                if num < 10 {
                    Some(num)
                } else {
                    None
                }
            })
            .collect();
        let value = values.first().unwrap() * 10 + values.last().unwrap();
        result += value;
    }
    result
}

#[derive(Debug, Clone, Copy)]
pub struct Number {
    value: usize,
    index: usize,
}

impl Number {
    fn build_first(line: &str, number_string: &str, number_usize: usize) -> Option<Number> {
        let s = line.find(number_string);
        let n = line.find(&number_usize.to_string());
        let index = match (s, n) {
            (Some(s), Some(n)) => Some(cmp::min(s, n)),
            (Some(s), None) => Some(s),
            (None, Some(n)) => Some(n),
            (None, None) => None,
        };
        if index.is_none() {
            None
        } else {
            Some(Number {
                value: number_usize,
                index: index.unwrap(),
            })
        }
    }

    fn build_last(line: &str, number_string: &str, number_usize: usize) -> Option<Number> {
        let s = line.rfind(number_string);
        let n = line.rfind(&number_usize.to_string());
        let index = match (s, n) {
            (Some(s), Some(n)) => Some(cmp::max(s, n)),
            (Some(s), None) => Some(s),
            (None, Some(n)) => Some(n),
            (None, None) => None,
        };
        if index.is_none() {
            None
        } else {
            Some(Number {
                value: number_usize,
                index: index.unwrap(),
            })
        }
    }
}

pub fn process_part2(input: &str) -> usize {
    let mut result: usize = 0;
    let lines = input.lines();
    for line in lines {
        let numbers: Vec<(String, usize)> = vec![
            ("one".to_string(), 1),
            ("two".to_string(), 2),
            ("three".to_string(), 3),
            ("four".to_string(), 4),
            ("five".to_string(), 5),
            ("six".to_string(), 6),
            ("seven".to_string(), 7),
            ("eight".to_string(), 8),
            ("nine".to_string(), 9),
        ];
        let mut indexes: Vec<Option<Number>> = Vec::new();
        for (number_string, number_usize) in numbers {
            indexes.push(Number::build_first(line, &number_string, number_usize));
            indexes.push(Number::build_last(line, &number_string, number_usize));
        }
        let indexes: Vec<Number> = indexes.into_iter().filter_map(|x| x).collect();
        let mut first: Number = Number {
            value: 0,
            index: 1000,
        };
        let mut last: Number = Number { value: 0, index: 0 };
        for index in indexes {
            if index.index < first.index {
                first = index.clone();
            }
            if index.index >= last.index {
                last = index.clone();
            }
        }
        let value = first.value * 10 + last.value;
        result += value;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    const INPUT2: &str = "\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    const INPUT3: &str = "\
4ffsfsvkxslvp";

    #[test]
    fn part1_works() {
        assert_eq!(process_part1(INPUT1), 142);
    }

    #[test]
    fn part2_works() {
        assert_eq!(process_part2(INPUT2), 281);
    }

    #[test]
    fn handle_0() {
        assert_eq!(process_part2(INPUT3), 44);
    }
}
