use day05::process_part1;
use std::fs;
use day05::process_part1_better;

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    println!("{}", process_part1(&file));
    println!("{}", process_part1_better());
}
