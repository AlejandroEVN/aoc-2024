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
        Some(arg) if arg == "day5" => {
            println!("Running day5...");
            measure_time!({ aoc::day5::part_1() }, 5, 1);
            measure_time!({ aoc::day5::part_2() }, 5, 2);
        }
        Some(arg) if arg == "day6" => {
            println!("Running day6...");
            measure_time!({ aoc::day6::part_1() }, 6, 1);
            measure_time!({ aoc::day6::part_2() }, 6, 2);
        }
        Some(arg) if arg == "day7" => {
            println!("Running day7...");
            measure_time!({ aoc::day7::part_1() }, 7, 1);
            measure_time!({ aoc::day7::part_2() }, 7, 2);
        }
        Some(arg) if arg == "day8" => {
            println!("Running day8...");
            measure_time!({ aoc::day8::part_1() }, 8, 1);
            measure_time!({ aoc::day8::part_2() }, 8, 2);
        }
        _ => {
            println!("No day mod file found!");
        }
    }
}
