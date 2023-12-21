use ahash::AHashSet as HashSet;
use itertools::Itertools as _;
use ndarray::{Array2, ArrayView, Axis};
use std::collections::VecDeque;

struct Map {
    map: Array2<bool>,
    start: (i64, i64),
}

impl Map {
    fn parse(inp: &str) -> Map {
        let width = inp.lines().next().unwrap().chars().count();
        let mut map = Array2::default((0, width));
        let mut start = None;
        for (row, line) in inp.lines().enumerate() {
            let mut next_row = vec![];
            for (col, c) in line.chars().enumerate() {
                if c == 'S' {
                    start = Some((row as i64, col as i64));
                }
                next_row.push(match c {
                    '#' => true,
                    '.' | 'S' => false,
                    _ => unreachable!(),
                });
            }
            map.append(
                Axis(0),
                ArrayView::from(&next_row).into_shape((1, width)).unwrap(),
            )
            .unwrap();
        }
        Map {
            map,
            start: start.unwrap(),
        }
    }

    fn reach(&self, steps: usize) -> usize {
        let mut seen = HashSet::new();
        let mut todo = VecDeque::new();
        todo.push_back((self.start, 0));
        while let Some((pos, depth)) = todo.pop_front() {
            if seen.contains(&pos) {
                continue;
            }
            seen.insert(pos);
            if depth < steps {
                todo.extend(adj(pos).into_iter().filter_map(|p| {
                    if self.is_wall(p) {
                        None
                    } else {
                        Some((p, depth + 1))
                    }
                }));
            }
        }
        seen.into_iter()
            .filter(|(row, col)| (row + col).rem_euclid(2) == steps as i64 % 2)
            .count()
    }

    #[allow(dead_code)]
    fn print(&self, found: &HashSet<(i64, i64)>) {
        for row in 0..=self.map.nrows() {
            for col in 0..=self.map.ncols() {
                if found.contains(&(row as i64, col as i64)) {
                    eprint!("O");
                } else if self.map[(row, col)] {
                    eprint!("#");
                } else {
                    eprint!(".");
                }
            }
            eprintln!();
        }
    }

    fn is_wall(&self, (row, col): (i64, i64)) -> bool {
        self.map[(
            row.rem_euclid(self.map.nrows() as i64) as usize,
            col.rem_euclid(self.map.ncols() as i64) as usize,
        )]
    }
}

fn adj((row, col): (i64, i64)) -> [(i64, i64); 4] {
    [
        (row - 1, col),
        (row, col - 1),
        (row + 1, col),
        (row, col + 1),
    ]
}

fn part1(inp: &str) -> usize {
    let map = Map::parse(inp);
    let steps = if map.map.nrows() < 50 { 6 } else { 64 };
    map.reach(steps)
}

fn part2(inp: &str) -> usize {
    let map = Map::parse(inp);
    if map.map.nrows() < 50 {
        return map.reach(1000);
    }
    const GOAL: usize = 26501365;
    let iter = (GOAL % 262..)
        .step_by(262)
        .map(|steps| (steps, map.reach(steps)))
        .tuple_windows()
        .map(|((_, pcount), (steps, count))| (steps, count, count - pcount))
        .tuple_windows()
        .map(|((_, _, pinc), (steps, count, inc))| (steps, count, inc, inc - pinc))
        .tuple_windows()
        .map(|((_, _, _, pinc2), (steps, count, inc, inc2))| {
            (steps, count, inc, inc2, inc2 - pinc2)
        });
    for (mut steps, mut count, mut inc, inc2, inc3) in iter {
        if inc3 == 0 {
            while steps < GOAL {
                steps += 262;
                inc += inc2;
                count += inc;
            }
            return count;
        }
    }
    unreachable!();
}

xaoc::xaoc!(
    sample = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
..........."
);
