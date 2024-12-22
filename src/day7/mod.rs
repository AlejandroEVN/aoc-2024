use regex::Regex;

pub fn part_1() -> i64 {
    let data = include_str!("input.txt");

    let regex = Regex::new(r"[^\d]+").unwrap();

    let mut total_calibration = 0;
    for line in data.lines() {
        let equation: Vec<i64> = regex
            .split(line)
            .filter_map(|str| str.parse().ok())
            .collect();

        match equation.first() {
            None => continue,
            Some(target) => {
                let is_solvable = solve(*target, &equation[1..], 0, false);
                if is_solvable {
                    total_calibration += target;
                }
            }
        }
    }

    total_calibration
}

fn solve(target: i64, nums: &[i64], acc: i64, allow_concat: bool) -> bool {
    match nums.first() {
        None => {
            if target == acc {
                return true;
            }
            return false;
        }
        Some(num) => {
            let concated_nums: i64 = (acc.to_string() + &num.to_string()).parse().unwrap();
            return solve(target, &nums[1..], acc + num, allow_concat)
                || solve(target, &nums[1..], acc * num, allow_concat)
                || allow_concat && solve(target, &nums[1..], concated_nums, allow_concat);
        }
    }
}

pub fn part_2() -> i64 {
    let data = include_str!("input.txt");

    let regex = Regex::new(r"[^\d]+").unwrap();

    let mut total_calibration = 0;
    for line in data.lines() {
        let equation: Vec<i64> = regex
            .split(line)
            .filter_map(|str| str.parse().ok())
            .collect();

        match equation.first() {
            None => continue,
            Some(target) => {
                let is_solvable = solve(*target, &equation[1..], 0, true);
                if is_solvable {
                    total_calibration += target;
                }
            }
        }
    }

    total_calibration
}
