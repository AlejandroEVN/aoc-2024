use std::{fs, i32};

pub fn run_part_1() -> u32 {
    let lines =
        fs::read_to_string("src/day2/input.txt").expect("Should have been able to read the file");

    let mut safe_reports: u32 = 0;

    for line in lines.trim().split("\n") {
        let reports: Vec<i32> = line
            .split(|c: char| !c.is_numeric())
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();

        let mut is_decreasing: bool = false;
        let mut is_increasing: bool = false;
        let mut is_safe: bool = true;

        for (i, report) in reports.iter().enumerate() {
            if i + 1 == reports.len() {
                break;
            }

            let diff: u32 = report.abs_diff(reports[i + 1]);

            if diff == 0 || diff > 3 {
                is_safe = false;
            }

            if *report < reports[i + 1] {
                is_increasing = true;
            } else {
                is_decreasing = true;
            }
        }

        if is_decreasing && is_increasing {
            is_safe = false;
        }

        if is_safe {
            safe_reports += 1;
        }
    }

    safe_reports
}

pub fn run_part_2() {
    todo!()
}
