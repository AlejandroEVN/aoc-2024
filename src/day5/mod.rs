use std::collections::HashSet;

fn are_pages_correct(pages: &Vec<i32>, rules_hash: &HashSet<(i32, i32)>) -> bool {
    let mut incorrect_positions = 0;

    for (index, first_page) in pages.iter().rev().enumerate() {
        if incorrect_positions != 0 {
            break;
        }
        if index == pages.len() - 1 {
            break;
        }

        for second_page in pages.iter().rev().skip(index + 1) {
            let page_tuple = (*first_page, *second_page);

            match rules_hash.contains(&page_tuple) {
                true => {
                    incorrect_positions += 1;
                    continue;
                }
                false => {}
            }
        }
    }

    if incorrect_positions > 0 {
        return false;
    }

    true
}

pub fn part_1() -> i32 {
    let data = include_str!("input.txt");

    let mut is_rule = true;
    let mut rules_hash: HashSet<(i32, i32)> = HashSet::new();
    let mut middle_pages: Vec<i32> = Vec::new();

    for line in data.lines() {
        if line.is_empty() {
            is_rule = false;
        }
        if is_rule {
            let rule: Vec<i32> = line
                .split("|")
                .filter_map(|c| c.parse::<i32>().ok())
                .collect();
            rules_hash.insert((rule[0], rule[1]));
        } else {
            let pages: Vec<i32> = line
                .split(",")
                .filter_map(|c| c.parse::<i32>().ok())
                .collect();

            if pages.is_empty() {
                continue;
            }

            if are_pages_correct(&pages, &rules_hash) {
                middle_pages.push(*pages.get(pages.len() / 2).unwrap());
            }
        }
    }

    middle_pages.into_iter().sum()
}
pub fn part_2() {}
