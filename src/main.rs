use aoc::day1;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.get(1) {
        Some(arg) if arg == "day1" => {
            println!("Running day1...");
            day1::run();
        }
        _ => {
            println!("No day mod file found!");
        }
    }
}
