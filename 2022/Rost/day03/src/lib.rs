pub fn process_part1(input: &str) -> u32 {
  let rucksacks = input
    .lines()
    .map(|rucksack| rucksack.split_at(rucksack.len() / 2));

  let mut result: u32 = 0;

  for (comp1, comp2) in rucksacks {
    let mut chars: Vec<char> = comp1
      .chars()
      .map(|char| {
        if comp2.contains(char) {
          Some(char)
        } else {
          None
        }
      })
      .into_iter()
      .filter_map(|c| c)
      .collect();
    chars.dedup();
    for char in chars {
      let value: u32;
      if char.is_uppercase() {
        value = char.to_digit(36).unwrap() + 17;
      } else {
        value = char.to_digit(36).unwrap() - 9;
      }
      result += value;
    }
  }
  result
}

#[derive(Debug)]
pub struct Group {
  rucksack1: String,
  rucksack2: String,
  rucksack3: String,
}

impl Group {
  pub fn new(vector: Vec<&str>) -> Group {
    Group {
      rucksack1: vector[0].to_string(),
      rucksack2: vector[1].to_string(),
      rucksack3: vector[2].to_string(),
    }
  }
}

pub fn process_part2(input: &str) -> u32 {
  let mut groups: Vec<&str> = input.lines().into_iter().collect();
  let mut result: u32 = 0;
  while groups.len() >= 3 {
    let group: Vec<&str> = groups.drain(..3).into_iter().collect();
    let group = Group::new(group);
    let mut chars: Vec<char> = group
      .rucksack1
      .chars()
      .map(|char| {
        if group.rucksack2.contains(char) && group.rucksack3.contains(char) {
          Some(char)
        } else {
          None
        }
      })
      .filter_map(|c| c)
      .collect();
    chars.dedup();
    for char in chars {
      let value: u32;
      if char.is_uppercase() {
        value = char.to_digit(36).unwrap() + 17;
      } else {
        value = char.to_digit(36).unwrap() - 9;
      }
      result += value;
    }
  }
  result
}

#[cfg(test)]
mod tests {
  use super::*;
  const INPUT: &str = "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

  #[test]
  fn part1_works() {
    assert_eq!(process_part1(INPUT), 157);
  }

  #[test]
  fn part2_works() {
    assert_eq!(process_part2(INPUT), 70);
  }
}
