use std::fs;

pub fn run_part_1() -> String {
    let contents =
        fs::read_to_string("src/day1/input.txt").expect("Should have been able to read the file");

    contents
}
