pub fn part_1() -> u32 {
    let lines = include_str!("input.txt");

    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();

    for line in lines.lines() {
        let parts: Vec<&str> = line.split("   ").collect();
        let numbers: Result<Vec<u32>, _> = parts.into_iter().map(|s| s.parse::<u32>()).collect();

        match numbers {
            Ok(nums) => match nums.as_slice() {
                [x, y] => {
                    left.push(*x);
                    right.push(*y);
                }
                _ => {
                    panic!("Wrong data. Not a parsable number.");
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

pub fn part_2() -> u32 {
    let lines = include_str!("input.txt");

    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();

    for line in lines.lines() {
        let parts: Vec<&str> = line.split("   ").collect();
        let numbers: Result<Vec<u32>, _> = parts.into_iter().map(|s| s.parse()).collect();

        match numbers {
            Ok(nums) => match nums.as_slice() {
                [x, y] => {
                    left.push(*x);
                    right.push(*y);
                }
                _ => {
                    panic!("Wrong data. Not a parsable number.");
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
