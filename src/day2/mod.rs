use std::i32;

fn is_safe(reports: &[i32]) -> bool {
    if reports.len() <= 1 {
        return true;
    }

    let order = reports[0].cmp(&reports[1]);

    for window in reports.windows(2) {
        let (curr, next) = (window[0], window[1]);

        if !(1..=3).contains(&curr.abs_diff(next)) {
            return false;
        }

        if curr.cmp(&next) != order {
            return false;
        }
    }

    true
}

fn is_safe_with_tolerance(reports: &Vec<i32>) -> bool {
    if is_safe(reports) {
        return true;
    }

    for i in 0..reports.len() {
        let mut mod_reports = reports.clone();
        mod_reports.remove(i);

        if is_safe(&mod_reports) {
            return true;
        }
    }

    false
}

pub fn part_1() -> usize {
    let lines = include_str!("input.txt");

    let mut safe_reports = 0;

    for line in lines.trim().split("\n") {
        let reports: Vec<i32> = line
            .split(' ')
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();

        if is_safe(&reports) {
            safe_reports += 1;
        }
    }

    safe_reports
}

pub fn part_2() -> usize {
    let lines = include_str!("input.txt");

    let mut safe_reports = 0;

    for line in lines.lines() {
        let reports: Vec<i32> = line.split(' ').map(|c| c.parse().unwrap()).collect();

        if is_safe_with_tolerance(&reports) {
            safe_reports += 1;
        }
    }

    safe_reports
}
