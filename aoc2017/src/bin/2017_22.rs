use itertools::{Itertools, MinMaxResult};
use std::collections::{HashMap, HashSet};

static DIRS: &[(i64, i64)] = &[(-1, 0), (0, 1), (1, 0), (0, -1)];

#[allow(dead_code)]
fn print(map: &HashSet<(i64, i64)>, vrow: i64, vcol: i64) {
    let MinMaxResult::MinMax(minr, maxr) = map.iter().map(|(row, _)| row).copied().minmax() else { unreachable!() };
    let MinMaxResult::MinMax(minc, maxc) = map.iter().map(|(_, col)| col).copied().minmax() else { unreachable!() };
    let minr = minr.min(vrow);
    let maxr = maxr.max(vrow);
    let minc = minc.min(vcol);
    let maxc = maxc.max(vcol);
    for row in minr..=maxr {
        for col in minc..=maxc {
            print!("{}", if (row, col) == (vrow, vcol) { '[' } else { ' ' });
            print!("{}", if map.contains(&(row, col)) { '#' } else { '.' });
            print!("{}", if (row, col) == (vrow, vcol) { ']' } else { ' ' });
        }
        println!();
    }
}

fn part1(inp: &str) -> i64 {
    let mut map = HashSet::new();
    let mut width = 0;
    let mut height = 0;
    for (row, line) in inp.lines().enumerate() {
        let row = row as i64;
        height = height.max(row);
        for (col, c) in line.chars().enumerate() {
            let col = col as i64;
            width = width.max(col);
            if c == '#' {
                map.insert((row, col));
            }
        }
    }
    height += 1;
    width += 1;
    let mut count = 0;
    let (mut row, mut col) = (height / 2, width / 2);
    let mut dir: i64 = 0;
    for _ in 0..10000 {
        if map.contains(&(row, col)) {
            dir = (dir + 1).rem_euclid(DIRS.len() as i64);
            map.remove(&(row, col));
        } else {
            dir = (dir - 1).rem_euclid(DIRS.len() as i64);
            map.insert((row, col));
            count += 1;
        }
        let (dr, dc) = DIRS[dir as usize];
        row += dr;
        col += dc;
    }
    count
}

enum State {
    Weakened,
    Infected,
    Flagged,
}

fn part2(inp: &str) -> i64 {
    let mut map = HashMap::new();
    let mut width = 0;
    let mut height = 0;
    for (row, line) in inp.lines().enumerate() {
        let row = row as i64;
        height = height.max(row);
        for (col, c) in line.chars().enumerate() {
            let col = col as i64;
            width = width.max(col);
            if c == '#' {
                map.insert((row, col), State::Infected);
            }
        }
    }
    height += 1;
    width += 1;
    let mut count = 0;
    let (mut row, mut col) = (height / 2, width / 2);
    let mut dir: i64 = 0;
    for _ in 0..10000000 {
        match map.get(&(row, col)) {
            Some(State::Weakened) => {
                map.insert((row, col), State::Infected);
                count += 1;
            }
            Some(State::Infected) => {
                map.insert((row, col), State::Flagged);
                dir = (dir + 1).rem_euclid(DIRS.len() as i64);
            }
            Some(State::Flagged) => {
                map.remove(&(row, col));
                dir = (dir + 2).rem_euclid(DIRS.len() as i64);
            }
            None => {
                map.insert((row, col), State::Weakened);
                dir = (dir - 1).rem_euclid(DIRS.len() as i64);
            }
        }
        let (dr, dc) = DIRS[dir as usize];
        row += dr;
        col += dc;
    }
    count
}

xaoc::xaoc!(sample_idx = 2);
