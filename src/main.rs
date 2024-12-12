use aoc::day1;
use aoc::day2;
use aoc::measure_time;
use std::env;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.get(1) {
        Some(arg) if arg == "day1" => {
            println!("Running day1...");
            measure_time!({ day1::run_part_1() }, 1, 1);
            measure_time!({ day1::run_part_2() }, 1, 2);
        }
        Some(arg) if arg == "day2" => {
            println!("Running day2...");
            measure_time!({ day2::run_part_1() }, 2, 1);
            measure_time!({ day2::run_part_2() }, 2, 2);
        }
        _ => {
            println!("No day mod file found!");
        }
    }
}
