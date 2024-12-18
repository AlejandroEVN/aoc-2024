use core::panic;
use std::{collections::HashSet, isize, usize};

type Grid = Vec<Vec<char>>;
type Delta = (isize, isize);
type Position = (usize, usize);

trait AddDelta {
    fn add_delta(&self, delta: Delta) -> Position;
}

impl AddDelta for Position {
    fn add_delta(&self, delta: Delta) -> Position {
        let (pos_row, pos_col) = *self;
        let (d_row, d_col) = delta;
        let new_row = pos_row as isize + d_row;
        let new_col = pos_col as isize + d_col;

        (new_row as usize, new_col as usize)
    }
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
enum LookDir {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl LookDir {
    fn delta(&self) -> Delta {
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
    starting_pos: Position,
    direction: LookDir,
    pos: Position,
    path: HashSet<(LookDir, usize, usize)>,
    unique_positions: HashSet<Position>,
}

impl Guard {
    fn new(pos: Position, look_dir: LookDir) -> Guard {
        Guard {
            starting_pos: (pos.0, pos.1),
            pos: (pos.0, pos.1),
            direction: look_dir,
            path: HashSet::new(),
            unique_positions: HashSet::new(),
        }
    }

    fn next_position(&self) -> Position {
        let (new_row, new_col) = self.pos.add_delta(self.direction.delta());

        (new_row as usize, new_col as usize)
    }

    fn walk(&mut self) {
        let next_pos = self.next_position();
        if next_pos != self.starting_pos {
            self.unique_positions.insert((next_pos.0, next_pos.1));
        }
        self.path
            .insert((self.direction.clone(), next_pos.0, next_pos.1));
        self.pos = (next_pos.0, next_pos.1)
    }

    fn turn(&mut self) {
        self.direction = turn_right_direction(&self.direction);
    }
}

fn turn_right_direction(curr_dir: &LookDir) -> LookDir {
    match curr_dir {
        LookDir::UP => LookDir::RIGHT,
        LookDir::RIGHT => LookDir::DOWN,
        LookDir::DOWN => LookDir::LEFT,
        LookDir::LEFT => LookDir::UP,
    }
}

pub fn part_1() -> usize {
    let data = include_str!("input.txt");

    let (grid, (looking_at, pos)) = parse_grid_and_guard_position(data);

    let mut guard = Guard::new(pos, looking_at.clone());

    loop {
        let next_position = guard.next_position();

        let exits_area =
            exits_the_mapped_area(&grid, (next_position.0 as isize, next_position.1 as isize));

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

    guard.unique_positions.iter().count()
}

pub fn part_2() -> usize {
    let data = include_str!("input.txt");

    let (grid, (looking_at, pos)) = parse_grid_and_guard_position(data);

    let mut guard = Guard::new(pos, looking_at.clone());

    let mut obstacles = 0;

    loop {
        let next_position = guard.next_position();

        let exits_area =
            exits_the_mapped_area(&grid, (next_position.0 as isize, next_position.1 as isize));

        if exits_area {
            guard.walk();
            break;
        }

        if obstacle_on_crossed_path(&grid, &guard) {
            obstacles += 1;
        }

        let should_turn = should_turn(&grid, next_position);

        if should_turn {
            guard.turn();
        }

        guard.walk();
    }

    obstacles
}

fn next_position(grid: &Grid, curr_position: Position, direction: &LookDir) -> Option<Position> {
    let (next_x, next_y) = curr_position.add_delta(direction.delta());
    let out_of_bounds = exits_the_mapped_area(grid, (next_x as isize, next_y as isize));

    if out_of_bounds {
        return None;
    }

    Some((next_x as usize, next_y as usize))
}

fn obstacle_on_crossed_path(grid: &Grid, guard: &Guard) -> bool {
    let turn_right_direction = turn_right_direction(&guard.direction);
    let mut position = guard.pos;

    while let Some(next_pos) = next_position(grid, position, &turn_right_direction) {
        if guard
            .path
            .contains(&(turn_right_direction.clone(), next_pos.0, next_pos.1))
        {
            return true;
        }
        position = next_pos;
    }

    false
}

fn exits_the_mapped_area(grid: &Grid, next_position: Delta) -> bool {
    next_position.0 >= grid.len() as isize
        || next_position.1 >= grid.first().unwrap().len() as isize
        || next_position.0 < 0
        || next_position.1 < 0
}

fn should_turn(grid: &Grid, next_position: Position) -> bool {
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

fn parse_grid_and_guard_position(data: &str) -> (Grid, (LookDir, Position)) {
    let grid: Grid = data
        .lines()
        .into_iter()
        .map(|line| line.chars().collect())
        .collect();

    let guard_position: Vec<(LookDir, Position)> = grid
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
