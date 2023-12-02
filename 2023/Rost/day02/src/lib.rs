pub fn process_part1(input: &str) -> usize {
  let mut result: usize = 0;
  let lines = input.lines();
  for line in lines {
    let mut iter = line.split(':');
    let id: Vec<u32> = iter
      .next()
      .unwrap()
      .chars()
      .filter_map(|c| c.to_digit(10))
      .collect();
    let id: String = id.iter().map(|&n| n.to_string()).collect();
    let id: u32 = id.parse().unwrap();
    let cubes = iter.next().unwrap().replace(";", ",");
    let cubes = cubes.split(',');
    let mut impossible = 0;
    for cube in cubes {
      let cube: Vec<&str> = cube.trim().split_whitespace().collect();
      let number: u32 = cube.first().unwrap().parse().unwrap();
      let color = cube.last().unwrap();
      if *color == "red" && number > 12 {
        impossible += 1;
      } else if *color == "green" && number > 13 {
        impossible += 1;
      } else if *color == "blue" && number > 14 {
        impossible += 1;
      }
    }
    if impossible == 0 {
      result += id as usize;
    }
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
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

  #[test]
  fn part1_works() {
    assert_eq!(process_part1(INPUT1), 8);
  }

  #[test]
  fn part2_works() {
    todo!();
  }
}
