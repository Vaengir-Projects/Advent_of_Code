pub fn recursive_num_check(grid: Vec<Vec<char>>) -> usize {
  todo!();
}

pub fn process_part1(input: &str) -> usize {
  let mut result: usize = 0;
  let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
  for i in 0..grid.len() {
    for j in 0..grid[i].len() {
      if grid[i][j].is_digit(10) {
        dbg!(grid[i][j], i, j);
      }
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
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

  #[test]
  fn part1_works() {
    assert_eq!(process_part1(INPUT1), 4361);
  }

  #[test]
  fn part2_works() {
    todo!();
  }
}
