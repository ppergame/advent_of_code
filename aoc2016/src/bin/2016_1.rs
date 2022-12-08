use std::collections::HashSet;

use itertools::Itertools;

static DIRS: &[(i64, i64)] = &[(0, -1), (1, 0), (0, 1), (-1, 0)];

fn part1(inp: &str) -> i64 {
    let (mut x, mut y) = (0, 0);
    let mut dir = 0;
    for s in inp.split(',').map(|s| s.trim()) {
        let mut it = s.chars();
        let t = it.next().unwrap();
        let steps: i64 = it.join("").parse().unwrap();
        match t {
            'L' => {
                if dir == 0 {
                    dir = DIRS.len() - 1
                } else {
                    dir = (dir - 1) % DIRS.len()
                }
            }
            'R' => dir = (dir + 1) % DIRS.len(),
            _ => unreachable!(),
        }
        let (dx, dy) = DIRS[dir];
        x += dx * steps;
        y += dy * steps;
    }
    x.abs() + y.abs()
}

fn part2(inp: &str) -> i64 {
    let mut visited = HashSet::new();
    let (mut x, mut y) = (0, 0);
    let mut dir = 0;
    for s in inp.split(',').map(|s| s.trim()) {
        let mut it = s.chars();
        let t = it.next().unwrap();
        let steps: i64 = it.join("").parse().unwrap();
        match t {
            'L' => {
                if dir == 0 {
                    dir = DIRS.len() - 1
                } else {
                    dir = (dir - 1) % DIRS.len()
                }
            }
            'R' => dir = (dir + 1) % DIRS.len(),
            _ => unreachable!(),
        }
        let (dx, dy) = DIRS[dir];
        for _ in 0..steps {
            x += dx;
            y += dy;
            if visited.contains(&(x, y)) {
                return x.abs() + y.abs();
            }
            visited.insert((x, y));
        }
    }
    unreachable!();
}

xaoc::xaoc!();
