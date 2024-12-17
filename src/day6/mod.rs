use std::{collections::HashSet, usize};

#[derive(Debug, PartialEq)]
enum LookDir {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Debug)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Guard<'a> {
    look_dir: &'a LookDir,
    pos: Position,
    path: HashSet<(usize, usize)>,
}

impl<'a> Guard<'a> {
    fn new(pos: (usize, usize), look_dir: &'a LookDir) -> Guard<'a> {
        Guard {
            pos: Position { x: pos.0, y: pos.1 },
            look_dir,
            path: HashSet::new(),
        }
    }

    fn walk_to(&mut self, pos: (usize, usize)) {
        self.path.insert(pos);
        self.pos = Position { x: pos.0, y: pos.1 };
    }

    fn turn(&mut self, look_dir: &'a LookDir) {
        self.look_dir = &look_dir;
    }
}

const CHAR_TO_DIR: [(LookDir, (isize, isize)); 4] = [
    (LookDir::UP, (-1, 0)),
    (LookDir::DOWN, (1, 0)),
    (LookDir::RIGHT, (0, 1)),
    (LookDir::LEFT, (0, -1)),
];

pub fn part_1() -> i32 {
    let data = include_str!("example.txt");

    let grid: Vec<Vec<&str>> = data
        .lines()
        .into_iter()
        .map(|line| line.split("").filter(|c| !c.is_empty()).collect())
        .collect();

    for (i, row) in grid.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            print!("({},{}){:?} | ", i, j, col);
        }
        println!();
    }

    let guard_position: Vec<(LookDir, (usize, usize))> = grid
        .iter()
        .enumerate()
        .flat_map(|(i, line)| {
            line.into_iter().enumerate().filter_map(move |(j, c)| {
                let looking_dir = c.chars().next().unwrap();

                match looking_dir {
                    '^' => Some((LookDir::UP, (i, j))),
                    'v' => Some((LookDir::DOWN, (i, j))),
                    '<' => Some((LookDir::LEFT, (i, j))),
                    '>' => Some((LookDir::RIGHT, (i, j))),
                    _ => None,
                }
            })
        })
        .collect();

    let (looking_at, pos) = guard_position.first().unwrap();
    let mut guard = Guard::new(*pos, looking_at);

    loop {
        let (look_to, walk_dir) = CHAR_TO_DIR
            .iter()
            .find(|(looking_dir, dir)| looking_dir == guard.look_dir)
            .unwrap();

        println!("Guard in {:?}", guard.pos);

        let new_x = guard.pos.x as isize + walk_dir.0;
        let new_y = guard.pos.y as isize + walk_dir.1;

        println!("Walk dir {:?}", walk_dir);
        guard.walk_to((new_x as usize, new_y as usize));
        println!("Guard in {:?}", guard.pos);
        break;
    }

    0
}

fn walk<'a>(
    grid: &'a Vec<Vec<&str>>,
    current_position: (usize, usize),
    path: &'a &Vec<(usize, usize)>,
    dir: (isize, isize),
) -> &'a Vec<(usize, usize)> {
    let new_x = current_position.0 as isize + dir.0;
    let new_y = current_position.1 as isize + dir.1;

    path
}

pub fn part_2() -> i32 {
    let data = include_str!("example.txt");
    0
}
