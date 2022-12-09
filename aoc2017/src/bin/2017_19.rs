use colored::Colorize;
use itertools::{Itertools, MinMaxResult};
use std::collections::HashMap;

static DIRS: &[(i64, i64)] = &[(0, -1), (1, 0), (0, 1), (-1, 0)];

#[allow(dead_code)]
fn print(crow: i64, ccol: i64, board: &HashMap<(i64, i64), char>) {
    let MinMaxResult::MinMax(min_row, max_row) = board.keys().map(|(row, _)| *row).minmax() else { unreachable!() };
    let MinMaxResult::MinMax(min_col, max_col) = board.keys().map(|(_,col)| *col).minmax() else { unreachable!() };
    for row in min_row..=max_row {
        for col in min_col..=max_col {
            let mut c = board
                .get(&(row, col))
                .copied()
                .unwrap_or(' ')
                .to_string()
                .green();
            if row == crow && col == ccol {
                c = c.red().bold();
            }
            print!("{}", c);
        }
        println!();
    }
}

fn part1(inp: &str) -> String {
    let mut board = HashMap::<(i64, i64), char>::new();
    for (row, line) in inp.lines().enumerate() {
        let row = row as i64;
        for (col, c) in line.chars().enumerate() {
            let col = col as i64;
            if c != ' ' {
                board.insert((row, col), c);
            }
        }
    }
    let mut acc = String::new();
    let mut dir: i64 = 1;
    let mut row = 0;
    let mut col = board
        .iter()
        .find_map(|(&(row, col), &d)| {
            if row == 0 && d == '|' {
                Some(col)
            } else {
                None
            }
        })
        .unwrap();
    // print(row, col, &board);
    loop {
        let (dr, dc) = DIRS[dir as usize];
        let (nr, nc) = (row + dr, col + dc);
        if let c @ 'A'..='Z' = board.get(&(row, col)).unwrap() {
            acc.push(*c)
        }
        if !board.contains_key(&(nr, nc)) {
            let mut found = false;
            for ndir in [dir - 1, dir + 1] {
                let ndir = ndir.rem_euclid(DIRS.len() as i64);
                let (dr, dc) = DIRS[ndir as usize];
                let (nr, nc) = (row + dr, col + dc);
                if board.contains_key(&(nr, nc)) {
                    assert_eq!(board.get(&(row, col)), Some(&'+'));
                    found = true;
                    dir = ndir;
                    break;
                }
            }
            if !found {
                // eprintln!("no way forward, no turns @ row={row} col={col} dir={dir}");
                return acc;
            }
            continue;
        }
        (row, col) = (nr, nc);
    }
}

fn part2(inp: &str) -> i64 {
    let mut board = HashMap::<(i64, i64), char>::new();
    for (row, line) in inp.lines().enumerate() {
        let row = row as i64;
        for (col, c) in line.chars().enumerate() {
            let col = col as i64;
            if c != ' ' {
                board.insert((row, col), c);
            }
        }
    }
    let mut steps = 1;
    let mut dir: i64 = 1;
    let mut row = 0;
    let mut col = board
        .iter()
        .find_map(|(&(row, col), &d)| {
            if row == 0 && d == '|' {
                Some(col)
            } else {
                None
            }
        })
        .unwrap();
    // print(row, col, &board);
    loop {
        let (dr, dc) = DIRS[dir as usize];
        let (nr, nc) = (row + dr, col + dc);
        if !board.contains_key(&(nr, nc)) {
            let mut found = false;
            for ndir in [dir - 1, dir + 1] {
                let ndir = ndir.rem_euclid(DIRS.len() as i64);
                let (dr, dc) = DIRS[ndir as usize];
                let (nr, nc) = (row + dr, col + dc);
                if board.contains_key(&(nr, nc)) {
                    assert_eq!(board.get(&(row, col)), Some(&'+'));
                    found = true;
                    dir = ndir;
                    break;
                }
            }
            if !found {
                // eprintln!("no way forward, no turns @ row={row} col={col} dir={dir}");
                return steps;
            }
            continue;
        }
        steps += 1;
        (row, col) = (nr, nc);
    }
}

xaoc::xaoc!(sample_idx = 3);
