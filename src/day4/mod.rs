use std::{collections::HashSet, i32, usize};

fn compare_chars(target: [&str; 2], curr_chars: Vec<&str>) -> bool {
    if target.len() != curr_chars.len() {
        return false;
    }

    let set1: HashSet<&str> = target.iter().cloned().collect();
    let set2: HashSet<&str> = curr_chars.iter().cloned().collect();

    set1 == set2
}

const DIRS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

const DIAG_DIRS: [((i32, i32), (i32, i32)); 2] = [((-1, -1), (1, 1)), ((1, -1), (-1, 1))];
const TARGET_CHARS: [&str; 2] = ["M", "S"];

fn get_new_cords(
    grid: &Vec<Vec<&str>>,
    row: i32,
    col: i32,
    (x, y): (i32, i32),
) -> Option<(i32, i32)> {
    let new_row = row as i32 + x;
    let new_col = col as i32 + y;

    if new_row < 0
        || new_row == grid.len() as i32
        || new_col < 0
        || new_col == grid[new_row as usize].len() as i32
    {
        return None;
    }

    Some((new_row, new_col))
}

fn has_cross_word(
    target_chars: [&str; 2],
    grid: &Vec<Vec<&str>>,
    curr_cord: (i32, i32),
    diagonal_cords: Vec<(i32, i32)>,
) -> bool {
    let mut chars: Vec<&str> = Vec::new();

    for (x, y) in &diagonal_cords {
        let new_cords;
        match get_new_cords(grid, curr_cord.0, curr_cord.1, (*x, *y)) {
            Some(cords) => {
                new_cords = cords;
            }
            None => {
                return false;
            }
        }
        let character = grid[new_cords.0 as usize][new_cords.1 as usize];
        chars.push(character);
    }
    let equal = compare_chars(target_chars, chars);

    if !equal {
        return false;
    }

    true
}

fn has_word(
    target_word: String,
    curr_word: String,
    grid: &Vec<Vec<&str>>,
    row: i32,
    col: i32,
    (x, y): (i32, i32),
    char_index: usize,
) -> bool {
    if curr_word == target_word {
        return true;
    }

    let new_cords;

    match get_new_cords(grid, row, col, (x, y)) {
        Some(cords) => {
            new_cords = cords;
        }
        None => {
            return false;
        }
    }

    let curr_char = grid[new_cords.0 as usize][new_cords.1 as usize];
    let compare_char = target_word.get(char_index..char_index + 1).unwrap();

    if curr_char != compare_char {
        return false;
    }

    let mut new_word = curr_word.to_string();
    new_word.push_str(curr_char);

    has_word(
        target_word,
        new_word,
        grid,
        new_cords.0,
        new_cords.1,
        (x, y),
        char_index + 1,
    )
}

pub fn part_1() -> i32 {
    let lines: Vec<&str> = include_str!("input.txt").trim().split('\n').collect();
    let grid: Vec<Vec<&str>> = lines
        .into_iter()
        .map(|line| line.split("").filter(|s| !s.is_empty()).collect())
        .collect();

    let mut count = 0;
    const TARGET_WORD: &str = "XMAS";

    for (row, line) in grid.clone().into_iter().enumerate() {
        for (col, c) in line.into_iter().enumerate() {
            if c != "X" {
                continue;
            }

            for dir in DIRS {
                let word_found = has_word(
                    TARGET_WORD.to_string(),
                    "X".to_string(),
                    &grid,
                    row as i32,
                    col as i32,
                    dir,
                    1,
                );
                if word_found {
                    count += 1;
                }
            }
        }
    }

    count
}

pub fn part_2() -> i32 {
    let lines: Vec<&str> = include_str!("input.txt").trim().split('\n').collect();
    let grid: Vec<Vec<&str>> = lines
        .into_iter()
        .map(|line| line.split("").filter(|s| !s.is_empty()).collect())
        .collect();

    let mut count = 0;

    for (row, line) in grid.clone().into_iter().enumerate() {
        for (col, c) in line.into_iter().enumerate() {
            if c != "A" {
                continue;
            }

            let mut diags_count = 0;

            for dir in DIAG_DIRS {
                let diag_dirs: Vec<(i32, i32)> = vec![(dir.0), (dir.1)];
                let word_found =
                    has_cross_word(TARGET_CHARS, &grid, (row as i32, col as i32), diag_dirs);

                if word_found {
                    diags_count += 1;
                }
            }

            if diags_count == 2 {
                count += 1;
            }
        }
    }

    count
}
