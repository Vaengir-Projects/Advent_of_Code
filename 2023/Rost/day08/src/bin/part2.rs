// use day08::process_part2;
// use day08::process_part2_better;
use day08::process_part2_try2;
use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    // println!("{}", process_part2(&file));
    // println!("{}", process_part2_better(&file));
    println!("{}", process_part2_try2(&file));
}
