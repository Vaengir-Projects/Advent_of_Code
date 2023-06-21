use day02::process_part1;
use std::fs;

fn main() {
  let file = fs::read_to_string(std::env::args().nth(1).expect("No file provided")).unwrap();
  println!("{}", process_part1(&file));
}
