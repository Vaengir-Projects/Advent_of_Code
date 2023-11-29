pub fn process_part1(input: &str) -> u32 {
  let y = input
    .split("\n")
    .map(|rucksack| rucksack.split_at(rucksack.len() / 2));

  let mut result: u32 = 0;

  for (comp1, comp2) in y {
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

pub fn process_part2(input: &str) -> i32 {
  0
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn part1_works() {
    let input = "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
    assert_eq!(process_part1(input), 157);
  }
}
