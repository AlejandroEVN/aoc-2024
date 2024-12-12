use std::fs;

pub fn run() {
    let contents =
        fs::read_to_string("src/day1/example.txt").expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}
