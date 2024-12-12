use std::fs;

pub fn run_part_1() -> u32 {
    let contents =
        fs::read_to_string("src/day1/input.txt").expect("Should have been able to read the file");

    let data = contents.trim().split("\n");

    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();

    for d in data {
        let parts: Vec<&str> = d.split("   ").collect();
        let numbers: Result<Vec<u32>, _> = parts.into_iter().map(|s| s.parse::<u32>()).collect();

        match numbers {
            Ok(nums) => match nums.as_slice() {
                [x, y] => {
                    left.push(*x);
                    right.push(*y);
                }
                _ => {
                    panic!("WTF");
                }
            },
            Err(_) => {
                println!("Wrong data! All data must be parsable to a number");
            }
        }
    }

    left.sort();
    right.sort();

    let mut result: u32 = 0;

    for (i, num) in left.iter().enumerate() {
        result += num.abs_diff(right[i]);
    }

    result
}

pub fn run_part_2() -> u32 {
    let contents =
        fs::read_to_string("src/day1/input.txt").expect("Should have been able to read the file");

    let data = contents.trim().split("\n");

    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();

    for d in data {
        let parts: Vec<&str> = d.split("   ").collect();
        let numbers: Result<Vec<u32>, _> = parts.into_iter().map(|s| s.parse::<u32>()).collect();

        match numbers {
            Ok(nums) => match nums.as_slice() {
                [x, y] => {
                    left.push(*x);
                    right.push(*y);
                }
                _ => {
                    panic!("WTF");
                }
            },
            Err(_) => {
                println!("Wrong data! All data must be parsable to a number");
            }
        }
    }

    left.sort();
    right.sort();

    let mut result: u32 = 0;

    for num in &left {
        let mut count: u32 = 0;

        for right_num in &right {
            if num == right_num {
                count += 1;
            }
        }
        result += num * count;
    }

    result
}
