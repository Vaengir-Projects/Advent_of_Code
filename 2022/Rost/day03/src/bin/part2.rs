use day03::process_part2;
use std::fs;

fn main() {
  let file = fs::read_to_string(std::env::args().nth(1).expect("No file provided")).unwrap();
  process_part2(&file);
}
