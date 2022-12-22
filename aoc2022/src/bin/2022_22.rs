use std::collections::{HashMap, VecDeque};

#[derive(Debug, Copy, Clone)]
enum Dir {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

impl Dir {
    fn right(&self) -> Self {
        match self {
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
            Dir::Up => Dir::Right,
        }
    }

    fn left(&self) -> Self {
        match self {
            Dir::Right => Dir::Up,
            Dir::Down => Dir::Right,
            Dir::Left => Dir::Down,
            Dir::Up => Dir::Left,
        }
    }
}

static DIRS: [(i64, i64); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
static DIR_CHARS: [char; 4] = ['>', 'v', '<', '^'];

#[derive(Debug)]
enum Step {
    TurnLeft,
    TurnRight,
    Move(i64),
}

impl Step {
    fn parse(s: &str) -> VecDeque<Self> {
        let mut acc = String::new();
        let mut ret = VecDeque::new();
        for c in s.chars() {
            match c {
                'L' => {
                    ret.push_back(Step::Move(std::mem::take(&mut acc).parse().unwrap()));
                    ret.push_back(Self::TurnLeft);
                }
                'R' => {
                    ret.push_back(Step::Move(std::mem::take(&mut acc).parse().unwrap()));
                    ret.push_back(Self::TurnRight);
                }
                '0'..='9' => acc.push(c),
                _ => unreachable!(),
            }
        }
        if !acc.is_empty() {
            ret.push_back(Step::Move(acc.parse().unwrap()));
        }
        ret
    }
}

struct Map {
    // true if passable, false if wall, absent if blank space in wrap around land
    map: HashMap<(i64, i64), bool>,
    row: i64,
    col: i64,
    max_row: i64,
    max_col: i64,
    dir: Dir,
    steps: VecDeque<Step>,
    trace: HashMap<(i64, i64), char>,
    part1: bool,
}

impl Map {
    fn parse(inp: &str, part1: bool) -> Self {
        let mut max_row = 0;
        let mut max_col = 0;
        let mut map = HashMap::new();
        let (grid, path) = inp.split_once("\n\n").unwrap();
        for (row, line) in grid.lines().enumerate() {
            let row = row as i64;
            max_row = max_row.max(row);
            for (col, c) in line.chars().enumerate() {
                let col = col as i64;
                max_col = max_col.max(col);
                match c {
                    '#' => {
                        map.insert((row, col), false);
                    }
                    '.' => {
                        map.insert((row, col), true);
                    }
                    ' ' => (),
                    _ => unreachable!(),
                };
            }
        }
        let col = (0..=max_col)
            .find(|&col| map.get(&(0, col)) == Some(&true))
            .unwrap();
        Self {
            map,
            row: 0,
            col,
            max_row,
            max_col,
            dir: Dir::Right,
            steps: Step::parse(path),
            trace: HashMap::new(),
            part1,
        }
    }

    fn step(&mut self) -> bool {
        let Some(next) = self.steps.pop_front() else { return false };
        match next {
            Step::TurnLeft => self.dir = self.dir.left(),
            Step::TurnRight => self.dir = self.dir.right(),
            Step::Move(n) => {
                for _ in 0..n {
                    if self.part1 {
                        let (dr, dc) = DIRS[self.dir as usize];
                        let mut row = self.row;
                        let mut col = self.col;
                        loop {
                            row += dr;
                            col += dc;
                            if row < 0 {
                                row = self.max_row;
                            } else if row > self.max_row {
                                row = 0;
                            }
                            if col < 0 {
                                col = self.max_col;
                            } else if col > self.max_col {
                                col = 0;
                            }
                            if self.map.contains_key(&(row, col)) {
                                break;
                            }
                        }
                        if self.map[&(row, col)] {
                            (self.row, self.col) = (row, col);
                            self.trace
                                .insert((self.row, self.col), DIR_CHARS[self.dir as usize]);
                        }
                    } else {
                        self.cube_step();
                        self.trace
                            .insert((self.row, self.col), DIR_CHARS[self.dir as usize]);
                    }
                }
            }
        }
        self.trace
            .insert((self.row, self.col), DIR_CHARS[self.dir as usize]);
        true
    }

    #[allow(dead_code)]
    fn print(&self) {
        for row in 0..=self.max_row {
            for col in 0..=self.max_col {
                if let Some(c) = self.trace.get(&(row, col)) {
                    print!("{}", c);
                } else {
                    match self.map.get(&(row, col)) {
                        Some(true) => print!("."),
                        Some(false) => print!("#"),
                        None => print!(" "),
                    }
                }
            }
            println!();
        }
    }

    fn cube_step(&mut self) {
        let (dr, dc) = DIRS[self.dir as usize];
        let mut row = self.row + dr;
        let mut col = self.col + dc;
        if let Some(&passable) = self.map.get(&(row, col)) {
            if passable {
                (self.row, self.col) = (row, col);
            }
            return;
        }
        let dir;
        match (self.row, self.col, self.dir) {
            (0, _, Dir::Up) if (50..100).contains(&self.col) => {
                row = self.col + 100;
                col = 0;
                dir = Dir::Right;
            }
            (_, 0, Dir::Left) if (150..200).contains(&self.row) => {
                row = 0;
                col = self.row - 100;
                dir = Dir::Down;
            }
            (_, 0, Dir::Left) if (100..150).contains(&self.row) => {
                row = 49 - (self.row - 100);
                col = 50;
                dir = Dir::Right;
            }
            (_, 50, Dir::Left) if (0..50).contains(&self.row) => {
                row = (49 - self.row) + 100;
                col = 0;
                dir = Dir::Right;
            }
            (149, _, Dir::Down) if (50..100).contains(&self.col) => {
                row = self.col + 100;
                col = 49;
                dir = Dir::Left;
            }
            (_, 49, Dir::Right) if (150..200).contains(&self.row) => {
                row = 149;
                col = self.row - 100;
                dir = Dir::Up;
            }
            (_, 99, Dir::Right) if (100..150).contains(&self.row) => {
                row = 49 - (self.row - 100);
                col = 149;
                dir = Dir::Left;
            }
            (_, 149, Dir::Right) if (0..50).contains(&self.row) => {
                row = 49 - self.row + 100;
                col = 99;
                dir = Dir::Left;
            }
            (49, _, Dir::Down) if (100..150).contains(&self.col) => {
                row = self.col - 50;
                col = 99;
                dir = Dir::Left;
            }
            (_, 99, Dir::Right) if (50..100).contains(&self.row) => {
                row = 49;
                col = self.row + 50;
                dir = Dir::Up;
            }
            (_, 50, Dir::Left) if (50..100).contains(&self.row) => {
                row = 100;
                col = self.row - 50;
                dir = Dir::Down;
            }
            (100, _, Dir::Up) if (0..50).contains(&self.col) => {
                row = self.col + 50;
                col = 50;
                dir = Dir::Right;
            }
            (199, _, Dir::Down) if (0..50).contains(&self.col) => {
                row = 0;
                col += 100;
                dir = Dir::Down;
            }
            (0, _, Dir::Up) if (100..150).contains(&self.col) => {
                row = 199;
                col -= 100;
                dir = Dir::Up;
            }
            _ => unreachable!(
                "{},{} -> {},{} @{:?} {:?}",
                self.row, self.col, row, col, self.dir, DIRS[self.dir as usize]
            ),
        }
        if *self.map.get(&(row, col)).unwrap() {
            self.dir = dir;
            (self.row, self.col) = (row, col);
        }
    }
}

fn part1(inp: &str) -> i64 {
    let mut map = Map::parse(inp, true);
    while map.step() {}
    1000 * (map.row + 1) + 4 * (map.col + 1) + map.dir as i64
}

fn part2(inp: &str) -> i64 {
    let mut map = Map::parse(inp, false);
    if map.steps.len() < 20 {
        return 0;
    }
    while map.step() {}
    1000 * (map.row + 1) + 4 * (map.col + 1) + map.dir as i64
}

xaoc::xaoc!();
