use core::panic;
use std::{collections::HashSet, usize};

type Grid = Vec<Vec<char>>;

#[derive(Debug, PartialEq, Clone)]
enum LookDir {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl LookDir {
    fn delta(&self) -> (isize, isize) {
        match self {
            LookDir::UP => (-1, 0),
            LookDir::DOWN => (1, 0),
            LookDir::RIGHT => (0, 1),
            LookDir::LEFT => (0, -1),
        }
    }
}

#[derive(Debug)]
struct Guard {
    starting_pos: (usize, usize),
    look_dir: LookDir,
    pos: (usize, usize),
    path: HashSet<(usize, usize)>,
}

impl Guard {
    fn new(pos: (usize, usize), look_dir: LookDir) -> Guard {
        Guard {
            starting_pos: (pos.0, pos.1),
            pos: (pos.0, pos.1),
            look_dir,
            path: HashSet::new(),
        }
    }

    fn next_position(&self) -> (usize, usize) {
        let (dx, dy) = self.look_dir.delta();
        let new_x = self.pos.0 as isize + dx;
        let new_y = self.pos.1 as isize + dy;

        (new_x as usize, new_y as usize)
    }

    fn walk(&mut self) {
        let next_pos = self.next_position();
        if next_pos != self.starting_pos {
            self.path.insert(next_pos);
        }
        self.pos = (next_pos.0, next_pos.1)
    }

    fn turn(&mut self) {
        match self.look_dir {
            LookDir::UP => self.look_dir = LookDir::RIGHT,
            LookDir::RIGHT => self.look_dir = LookDir::DOWN,
            LookDir::DOWN => self.look_dir = LookDir::LEFT,
            LookDir::LEFT => self.look_dir = LookDir::UP,
        }
    }
}

pub fn part_1() -> usize {
    let data = include_str!("input.txt");

    let (grid, (looking_at, pos)) = parse_grid_and_guard_position(data);

    let mut guard = Guard::new(pos, looking_at.clone());

    loop {
        let next_position = guard.next_position();

        let exits_area = exits_the_mapped_area(&grid, next_position);

        if exits_area {
            guard.walk();
            break;
        }

        let should_turn = should_turn(&grid, next_position);

        if should_turn {
            guard.turn();
        }

        guard.walk();
    }

    guard.path.iter().count()
}

pub fn part_2() -> usize {
    let data = include_str!("example.txt");

    let (grid, (looking_at, pos)) = parse_grid_and_guard_position(data);

    let mut guard = Guard::new(pos, looking_at.clone());

    guard.path.iter().count() + 1
}

fn exits_the_mapped_area(grid: &Grid, next_position: (usize, usize)) -> bool {
    next_position.0 >= grid.len() || next_position.1 >= grid.first().unwrap().len()
}

fn should_turn(grid: &Grid, next_position: (usize, usize)) -> bool {
    let row = grid.get(next_position.0);

    match row {
        Some(cols) => {
            let col = cols.get(next_position.1);
            match col {
                Some(cell) => {
                    if *cell == '#' {
                        true
                    } else {
                        false
                    }
                }
                None => panic!("Invalid position in grid"),
            }
        }
        None => panic!("Invalid position in grid"),
    }
}

fn parse_grid_and_guard_position(data: &str) -> (Grid, (LookDir, (usize, usize))) {
    let grid: Grid = data
        .lines()
        .into_iter()
        .map(|line| line.chars().collect())
        .collect();

    let guard_position: Vec<(LookDir, (usize, usize))> = grid
        .iter()
        .enumerate()
        .flat_map(|(i, line)| {
            line.into_iter()
                .enumerate()
                .filter_map(move |(j, c)| match c {
                    '^' => Some((LookDir::UP, (i, j))),
                    'v' => Some((LookDir::DOWN, (i, j))),
                    '<' => Some((LookDir::LEFT, (i, j))),
                    '>' => Some((LookDir::RIGHT, (i, j))),
                    _ => None,
                })
        })
        .collect();

    match guard_position.first() {
        Some((looking_dir, pos)) => (grid, (looking_dir.clone(), *pos)),
        None => panic!("Wrong data! No Guard in the grid"),
    }
}
