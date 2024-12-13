use regex::Regex;

pub fn part_1() -> i32 {
    let lines = include_str!("input.txt");

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)]").unwrap();

    let mut sum = 0;

    for line in lines.lines() {
        for cap in re.captures_iter(line) {
            let x: i32 = cap[1].parse().unwrap();
            let y: i32 = cap[2].parse().unwrap();

            sum += x * y;
        }
    }

    sum
}

pub fn part_2() -> i32 {
    let lines = include_str!("input.txt");

    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();

    let mut allow_add = true;
    let mut sum = 0;

    for line in lines.lines() {
        for cap in re.captures_iter(line) {
            let instruction = cap.get(0).unwrap().as_str();

            match instruction {
                "do()" => {
                    allow_add = true;
                }
                "don't()" => {
                    allow_add = false;
                }
                _ => {
                    if !allow_add {
                        continue;
                    }
                    let x: i32 = cap.get(1).unwrap().as_str().parse().unwrap();
                    let y: i32 = cap.get(2).unwrap().as_str().parse().unwrap();

                    sum += x * y;
                }
            }
        }
    }
    sum
}
