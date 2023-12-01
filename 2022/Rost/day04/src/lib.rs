pub fn process_part1(input: &str) -> usize {
  let mut result: usize = 0;
  let area: Vec<&str> = input.lines().collect();
  let mut area_left: Vec<&str> = Vec::new();
  let mut area_right: Vec<&str> = Vec::new();
  for a in area {
    let mut iter = a.split(',');
    area_left = iter.next().unwrap().split("-").map(|x| x).collect();
    area_right = iter.next().unwrap().split("-").map(|x| x).collect();
    let left: Vec<usize> = area_left.into_iter().map(|x| x.parse().unwrap()).collect();
    let right: Vec<usize> = area_right.into_iter().map(|x| x.parse().unwrap()).collect();
    if (left.first() >= right.first()) && (left.last() <= right.last()) {
      result += 1;
    } else if (right.first() >= left.first()) && (right.last() <= left.last()) {
      result += 1;
    }
  }
  result
}

pub fn process_part2(input: &str) -> usize {
  let mut result: usize = 0;
  let area: Vec<&str> = input.lines().collect();
  let mut area_left: Vec<&str> = Vec::new();
  let mut area_right: Vec<&str> = Vec::new();
  for a in area {
    let mut iter = a.split(',');
    area_left = iter.next().unwrap().split("-").map(|x| x).collect();
    area_right = iter.next().unwrap().split("-").map(|x| x).collect();
    let left: Vec<usize> = area_left.into_iter().map(|x| x.parse().unwrap()).collect();
    let right: Vec<usize> = area_right.into_iter().map(|x| x.parse().unwrap()).collect();
    if left.first() >= right.first() && left.first() <= right.last() {
      result += 1;
    } else if left.last() >= right.first() && left.last() <= right.last() {
      result += 1;
    } else if left.first() >= right.first() && left.first() <= right.last() {
      result += 1;
    } else if right.last() >= left.first() && right.last() <= left.last() {
      result += 1;
    }
  }
  result
}

#[cfg(test)]
mod tests {
  use super::*;

  const INPUT1: &str = "\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

  #[test]
  fn part1_works() {
    assert_eq!(process_part1(INPUT1), 2);
  }

  #[test]
  fn part2_works() {
    assert_eq!(process_part2(INPUT1), 4);
  }
}
