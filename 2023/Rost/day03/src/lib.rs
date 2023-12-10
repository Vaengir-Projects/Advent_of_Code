use std::collections::HashSet;

pub fn get_number(grid: &Vec<Vec<char>>, i: usize, j: usize, n: char) -> (u32, usize) {
  let mut number: String = String::from(n);
  if grid[i][j + 1].is_digit(10) {
    number += &grid[i][j + 1].to_string();
  } else {
    return (number.parse().unwrap(), number.len());
  }
  if grid[i][j + 2].is_digit(10) {
    number += &grid[i][j + 2].to_string();
  }
  (number.parse().unwrap(), number.len())
}

pub fn has_symbol(grid: &Vec<Vec<char>>, length: usize, i: usize, j: usize) -> bool {
  let mut spaces_to_check: Vec<(i32, i32)> = Vec::new();
  for x in 0..length + 2 {
    spaces_to_check.push((i as i32 - 1, j as i32 + x as i32 - 1));
    spaces_to_check.push((i as i32, j as i32 - 1));
    spaces_to_check.push((i as i32, j as i32 + length as i32));
    spaces_to_check.push((i as i32 + 1, j as i32 + x as i32 - 1));
  }
  let spaces_to_check: Vec<(i32, i32)> = spaces_to_check
    .into_iter()
    .filter(|&(x, y)| x >= 0 && x < grid.len() as i32 && y >= 0 && y < grid[i].len() as i32)
    .collect();
  let symbol: Vec<(i32, i32)> = spaces_to_check
    .into_iter()
    .filter(|&(x, y)| {
      grid[x as usize][y as usize] != '.' && !grid[x as usize][y as usize].is_digit(10)
    })
    .collect();
  symbol.len() > 0
}

pub fn process_part1(input: &str) -> usize {
  let mut result: usize = 0;
  let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
  let mut number: u32 = 0;
  let mut length: usize = 0;
  for i in 0..grid.len() {
    let mut j = 0;
    loop {
      if grid[i][j].is_digit(10) {
        (number, length) = get_number(&grid, i, j, grid[i][j].clone());

        if has_symbol(&grid, length, i, j) {
          println!("{}", number);
          result += number as usize;
        }

        j += length;
      } else {
        j += 1;
      }

      if j >= grid[i].len() {
        break;
      }
    }
  }
  result
}

#[derive(Debug, Clone)]
pub struct Gear {
  gear: bool,
  first: Option<usize>,
  second: Option<usize>,
}

impl Gear {
  pub fn build(gear: bool, first: Option<usize>, second: Option<usize>) -> Gear {
    Gear {
      gear,
      first,
      second,
    }
  }
}

pub fn create_number(grid: &Vec<Vec<char>>, i: usize, j: usize) -> usize {
  let mut result = String::from(grid[i][j]);
  if grid[i][j - 1].is_digit(10) {
    result = format!("{}{}", grid[i][j - 1], result);
    if grid[i][j - 2].is_digit(10) {
      result = format!("{}{}", grid[i][j - 2], result);
    }
  }
  if grid[i][j + 1].is_digit(10) && j + 1 < grid[i].len() {
    result = format!("{}{}", result, grid[i][j + 1]);
    if grid[i][j + 2].is_digit(10) && j + 2 < grid[i].len() {
      result = format!("{}{}", result, grid[i][j + 2]);
    }
  }

  result.parse::<usize>().unwrap()
}

pub fn check_gears(grid: &Vec<Vec<char>>, i: usize, j: usize) -> Gear {
  let area_to_check: Vec<(i32, i32)> = vec![
    (i as i32 - 1, j as i32 - 1),
    (i as i32 - 1, j as i32),
    (i as i32 - 1, j as i32 + 1),
    (i as i32, j as i32 - 1),
    (i as i32, j as i32),
    (i as i32, j as i32 + 1),
    (i as i32 + 1, j as i32 - 1),
    (i as i32 + 1, j as i32),
    (i as i32 + 1, j as i32 + 1),
  ];
  let area_to_check: Vec<(i32, i32)> = area_to_check
    .into_iter()
    .filter(|&(x, y)| x >= 0 && x < grid.len() as i32 && y >= 0 && y < grid[i].len() as i32)
    .collect();

  let mut numbers: HashSet<usize> = HashSet::new();

  for area in area_to_check {
    if grid[area.0 as usize][area.1 as usize].is_digit(10) {
      numbers.insert(create_number(grid, area.0 as usize, area.1 as usize));
    }
  }
  if numbers.len() == 2 {
    let gear = true;
    return Gear::build(
      gear,
      Some(numbers.iter().next().unwrap().clone()),
      Some(numbers.iter().nth(1).unwrap().clone()),
    );
  } else {
    let gear = false;
    Gear::build(gear, None, None)
  }
}

pub fn process_part2(input: &str) -> usize {
  let mut result: usize = 0;
  let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
  for i in 0..grid.len() {
    let mut j = 0;
    loop {
      if grid[i][j] == '*' {
        let gear: Gear = check_gears(&grid, i, j);
        if gear.gear {
          result += gear.first.unwrap() * gear.second.unwrap();
        }
      }
      j += 1;

      if j >= grid[i].len() {
        break;
      }
    }
  }
  result
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

  const INPUT3: &str = "\
........810........................*.......805.......100..*...........999.....743......169............477...961......973............$.......
.574...........6*262........398....204......*.....%.......525...........*.........................412..*.........376...*.@43....=......=....
....*836....................................25...619............658.....172.......................*...408...........*........%...776..802...";

  #[test]
  fn part1_works() {
    assert_eq!(process_part1(INPUT1), 4361);
  }

  #[test]
  fn part1_works_with_single_digits() {
    assert_eq!(process_part1(INPUT3), 9294);
  }

  #[test]
  fn part2_works() {
    assert_eq!(process_part2(INPUT1), 467835);
  }
}
