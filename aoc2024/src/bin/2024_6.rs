use hashbrown::{HashMap, HashSet};
use rayon::prelude::*;

lazy_static::lazy_static! {
    static ref DIRS: &'static [(i64, i64)] = &[(0, -1), (-1, 0), (0, 1), (1, 0)];
    static ref DIR_CHAR: HashMap<char, usize> = {
        let mut m = HashMap::new();
        m.insert('<', 0);
        m.insert('^', 1);
        m.insert('>', 2);
        m.insert('v', 3);
        m
    };
}

#[derive(Debug, Clone)]
struct Map {
    map: HashSet<(i64, i64)>,
    max_col: i64,
    max_row: i64,
    guard: ((i64, i64), usize),
}

impl Map {
    fn parse(inp: &str) -> Self {
        let mut map = HashSet::new();
        let mut guard = None;
        let mut max_row = 0;
        let mut max_col = 0;
        for (row, line) in inp.lines().enumerate() {
            let row = row as i64;
            max_row = max_row.max(row);
            for (col, c) in line.chars().enumerate() {
                let col = col as i64;
                max_col = max_col.max(col);
                if c == '#' {
                    map.insert((row, col));
                } else if let Some(dir) = DIR_CHAR.get(&c) {
                    guard = Some(((row, col), *dir));
                } else if c == '.' {
                } else {
                    panic!("bad char {c}");
                }
            }
        }
        Self {
            map,
            max_col,
            max_row,
            guard: guard.unwrap(),
        }
    }

    fn next_pos(&self) -> (i64, i64) {
        let (pos, dir) = self.guard;
        let (drow, dcol) = DIRS[dir];
        (pos.0 + drow, pos.1 + dcol)
    }

    fn step(&mut self) {
        let (mut pos, mut dir) = self.guard;
        let next_pos = self.next_pos();
        if self.map.contains(&next_pos) {
            dir = (dir + 1) % DIRS.len();
        } else {
            pos = next_pos;
        }
        self.guard = (pos, dir);
    }

    fn in_bounds(&mut self) -> bool {
        let ((row, col), _) = self.guard;
        row >= 0 && row <= self.max_row && col >= 0 && col <= self.max_col
    }

    fn seen_outbound(&self) -> HashSet<(i64, i64)> {
        let mut map = self.clone();
        let mut seen = HashSet::new();
        while map.in_bounds() {
            seen.insert(map.guard.0);
            map.step();
        }
        seen
    }

    fn is_looped(&self, (row, col): (i64, i64)) -> bool {
        let mut map = self.clone();
        map.map.insert((row, col));
        let mut seen = HashSet::new();
        seen.insert(self.guard);
        while map.in_bounds() {
            map.step();
            if seen.contains(&map.guard) {
                return true;
            }
            seen.insert(map.guard);
        }
        false
    }

    #[allow(dead_code)]
    fn print(&self) {
        let (pos, dir) = self.guard;
        for row in 0..self.max_row {
            for col in 0..self.max_col {
                if pos == (col, row) {
                    print!(
                        "{}",
                        match dir {
                            0 => '<',
                            1 => '^',
                            2 => '>',
                            3 => 'v',
                            _ => panic!("bad dir"),
                        }
                    );
                } else if self.map.contains(&(col, row)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

fn part1(inp: &str) -> usize {
    let map = Map::parse(inp);
    map.seen_outbound().len()
}

fn part2(inp: &str) -> usize {
    let map = Map::parse(inp);
    map.seen_outbound()
        .par_iter()
        .filter(|(row, col)| map.is_looped((*row, *col)))
        .count()
}

xaoc::xaoc!(
    sample = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."
);
