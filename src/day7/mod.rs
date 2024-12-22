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
                let is_solvable = solve(*target, &equation[1..], 0);
                if is_solvable {
                    total_calibration += target;
                }
            }
        }
    }

    total_calibration
}

fn solve(target: i64, nums: &[i64], acc: i64) -> bool {
    match nums.first() {
        None => {
            if target == acc {
                return true;
            }
            return false;
        }
        Some(num) => {
            return solve(target, &nums[1..], acc + num) || solve(target, &nums[1..], acc * num);
        }
    }
}

pub fn part_2() -> usize {
    0
}
