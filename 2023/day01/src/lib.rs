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


pub fn process_part2(input: &str) -> usize {
  todo!();
}

#[cfg(test)]
mod tests {
  use super::*;

  const INPUT1: &str = "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

  #[test]
  fn part1_works() {
    assert_eq!(process_part1(INPUT1), 142);
  }

  #[test]
  fn part2_works() {
    todo!();
  }
}
