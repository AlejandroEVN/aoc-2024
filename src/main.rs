use aoc;
use aoc::measure_time;
use std::env;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.get(1) {
        Some(arg) if arg == "day1" => {
            println!("Running day1...");
            measure_time!({ aoc::day1::part_1() }, 1, 1);
            measure_time!({ aoc::day1::part_2() }, 1, 2);
        }
        Some(arg) if arg == "day2" => {
            println!("Running day2...");
            measure_time!({ aoc::day2::part_1() }, 2, 1);
            measure_time!({ aoc::day2::part_2() }, 2, 2);
        }
        Some(arg) if arg == "day3" => {
            println!("Running day3...");
            measure_time!({ aoc::day3::part_1() }, 3, 1);
            measure_time!({ aoc::day3::part_2() }, 3, 2);
        }
        Some(arg) if arg == "day4" => {
            println!("Running day4...");
            measure_time!({ aoc::day4::part_1() }, 4, 1);
            measure_time!({ aoc::day4::part_2() }, 4, 2);
        }
        _ => {
            println!("No day mod file found!");
        }
    }
}
