use day_01::process_part1;
use day_01::process_part1_better;
use std::fs;

fn main() {
    let file = fs::read_to_string(std::env::args().nth(1).expect("No file provided")).unwrap();
    println!("{}", process_part1(&file));
    println!("{}", process_part1_better(&file));
}
