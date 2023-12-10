use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Tile {
    adj1: (i64, i64),
    adj2: (i64, i64),
    c: char,
}

impl Tile {
    fn next(&self, prev: (i64, i64)) -> (i64, i64) {
        if prev == self.adj1 {
            self.adj2
        } else {
            assert_eq!(prev, self.adj2);
            self.adj1
        }
    }

    fn connects_to(&self, adj: (i64, i64)) -> bool {
        self.adj1 == adj || self.adj2 == adj
    }
}

#[derive(Debug)]
struct Map {
    tiles: HashMap<(i64, i64), Tile>,
    start: (i64, i64),
    max_row: i64,
    max_col: i64,
}

fn adj((row, col): (i64, i64), c: char) -> ((i64, i64), (i64, i64)) {
    match c {
        '|' => ((row - 1, col), (row + 1, col)),
        '-' => ((row, col - 1), (row, col + 1)),
        'L' => ((row - 1, col), (row, col + 1)),
        'J' => ((row - 1, col), (row, col - 1)),
        '7' => ((row, col - 1), (row + 1, col)),
        'F' => ((row, col + 1), (row + 1, col)),
        _ => unreachable!(),
    }
}

impl Map {
    fn parse(inp: &str) -> Self {
        let mut tiles = HashMap::new();
        let mut start = None;
        let mut max_row = 0;
        let mut max_col = 0;
        for (row, line) in inp.lines().enumerate() {
            let row = row as i64;
            max_row = max_row.max(row);
            for (col, c) in line.chars().enumerate() {
                let col = col as i64;
                max_col = max_col.max(col);
                let adj1;
                let adj2;
                match c {
                    '|' | '-' | 'L' | 'J' | '7' | 'F' => (adj1, adj2) = adj((row, col), c),
                    '.' => {
                        continue;
                    }
                    'S' => {
                        start = Some((row, col));
                        continue;
                    }
                    _ => unreachable!(),
                }
                tiles.insert((row, col), Tile { adj1, adj2, c });
            }
        }
        let mut ret = Self {
            tiles,
            start: start.unwrap(),
            max_row,
            max_col,
        };
        ret.amend_start();
        ret
    }

    fn amend_start(&mut self) {
        let start_c = *PIPES
            .iter()
            .find(|c| {
                let (adj1, adj2) = adj(self.start, **c);
                self.tiles
                    .get(&adj1)
                    .map(|t| t.connects_to(self.start))
                    .unwrap_or(false)
                    && self
                        .tiles
                        .get(&adj2)
                        .map(|t| t.connects_to(self.start))
                        .unwrap_or(false)
            })
            .unwrap();
        let (adj1, adj2) = adj(self.start, start_c);
        self.tiles.insert(
            self.start,
            Tile {
                adj1,
                adj2,
                c: start_c,
            },
        );
    }

    fn fill(&self, start: (i64, i64), path: &[(i64, i64)]) -> Option<usize> {
        let path = HashSet::<(i64, i64)>::from_iter(path.iter().flat_map(|(row, col)| {
            let (row2, col2) = (2 * *row, 2 * *col);
            let (adj1, adj2) = adj((row2, col2), self.tiles[&(*row, *col)].c);
            [adj1, adj2, (row2, col2)]
        }));
        let mut filled = HashSet::new();
        let mut todo = vec![start];
        while let Some(pos) = todo.pop() {
            if filled.contains(&pos) {
                continue;
            }
            filled.insert(pos);
            // self.render(&filled, &todo.iter().copied().collect(), &path);
            let (row, col) = pos;
            if row < 0 || row > self.max_row * 2 || col < 0 || col > self.max_col * 2 {
                return None;
            }
            todo.extend(
                [
                    (row - 1, col),
                    (row + 1, col),
                    (row, col - 1),
                    (row, col + 1),
                ]
                .into_iter()
                .filter(|pos| !filled.contains(pos) && !path.contains(pos)),
            );
        }
        Some(
            filled
                .into_iter()
                .filter(|(row, col)| row % 2 == 0 && col % 2 == 0)
                .count(),
        )
    }

    #[allow(dead_code)]
    fn render(
        &self,
        filled: &HashSet<(i64, i64)>,
        todo: &HashSet<(i64, i64)>,
        path: &HashSet<(i64, i64)>,
    ) {
        for row in 0..=self.max_row * 2 {
            for col in 0..=self.max_col * 2 {
                let c = if filled.contains(&(row, col)) {
                    '#'
                } else if todo.contains(&(row, col)) {
                    '?'
                } else if path.contains(&(row, col)) {
                    'X'
                } else {
                    '.'
                };
                print!("{}", c);
            }
            println!();
        }
    }
}

const PIPES: &[char] = &['|', '-', 'L', 'J', '7', 'F'];

fn part1(inp: &str) -> i64 {
    let map = Map::parse(inp);
    let mut prev = map.start;
    let mut pos = map.tiles[&map.start].adj1;
    let mut count = 1;
    while pos != map.start {
        let next = map.tiles[&pos].next(prev);
        prev = pos;
        pos = next;
        count += 1;
    }
    count / 2
}

fn part2(inp: &str) -> usize {
    let map = Map::parse(inp);
    let mut path = vec![map.start];
    let mut prev = map.start;
    let mut pos = map.tiles[&map.start].adj1;
    while pos != map.start {
        path.push(pos);
        let next = map.tiles[&pos].next(prev);
        prev = pos;
        pos = next;
    }
    let (row, col) = *path
        .iter()
        .find(|(row, col)| map.tiles[&(*row, *col)].c == '|')
        .unwrap();
    map.fill((row * 2, col * 2 - 1), &path)
        .or_else(|| map.fill((row * 2, col * 2 + 1), &path))
        .unwrap()
}

xaoc::xaoc!(
    sample = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF",
    sample2 = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
..........."
);
