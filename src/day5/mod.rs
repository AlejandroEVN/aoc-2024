use std::collections::HashSet;

fn are_pages_correct(
    pages: &Vec<i32>,
    rules_hash: &HashSet<(i32, i32)>,
) -> (bool, Vec<(i32, i32)>) {
    let mut incorrect_positions: Vec<(i32, i32)> = Vec::new();

    for (index, first_page) in pages.iter().rev().enumerate() {
        for second_page in pages.iter().rev().skip(index + 1) {
            let page_tuple = (*first_page, *second_page);

            match rules_hash.contains(&page_tuple) {
                true => {
                    let first_page_index =
                        pages.iter().position(|&x| x == *first_page).unwrap() as i32;
                    let second_page_index =
                        pages.iter().position(|&x| x == *second_page).unwrap() as i32;
                    incorrect_positions.push((second_page_index, first_page_index));
                }
                false => {}
            }
        }
    }

    if incorrect_positions.len() > 0 {
        return (false, incorrect_positions);
    }

    (true, incorrect_positions)
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

            let (correct, _) = are_pages_correct(&pages, &rules_hash);
            if correct {
                middle_pages.push(*pages.get(pages.len() / 2).unwrap());
            }
        }
    }

    middle_pages.into_iter().sum()
}

pub fn part_2() -> i32 {
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

            let (correct, incorrect_positions) = are_pages_correct(&pages, &rules_hash);

            if correct {
                continue;
            }

            let mut new_pages_order = pages.clone();
            let mut swap_position = *incorrect_positions.get(0).unwrap();

            loop {
                let (correct, position) =
                    check_and_swap(&mut new_pages_order, swap_position, &rules_hash);

                if correct {
                    break;
                }

                swap_position = position.unwrap();
            }

            let middle_page = new_pages_order.get(new_pages_order.len() / 2).unwrap();
            middle_pages.push(*middle_page);
        }
    }

    middle_pages.into_iter().sum()
}

fn check_and_swap(
    pages: &mut Vec<i32>,
    (i, j): (i32, i32),
    rules_hash: &HashSet<(i32, i32)>,
) -> (bool, Option<(i32, i32)>) {
    pages.swap(i as usize, j as usize);
    let (correct, incorrect_positions) = are_pages_correct(&pages, &rules_hash);

    if correct {
        return (true, None);
    }

    (false, Some(*incorrect_positions.get(0).unwrap()))
}
