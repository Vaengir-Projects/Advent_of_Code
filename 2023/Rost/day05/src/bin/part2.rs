use day05::process_part2;
use std::fs;
use day05::process_part2_better;

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    println!("{}", process_part2(&file));
    println!("{}", process_part2_better());
}
