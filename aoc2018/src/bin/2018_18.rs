use std::{collections::HashMap, fmt::Display};

use array2d::Array2D;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Acre {
    Open,
    Tree,
    Yard,
}

impl Display for Acre {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Acre::Open => '.',
            Acre::Tree => '|',
            Acre::Yard => '#',
        };
        write!(f, "{}", c)
    }
}

fn count_adj(v: &Array2D<Acre>, row: usize, col: usize, acre: Acre) -> usize {
    let mut count = 0;
    for srow in row.saturating_sub(1)..=row + 1 {
        for scol in col.saturating_sub(1)..=col + 1 {
            if (srow, scol) == (row, col) {
                continue;
            }
            if let Some(&item) = v.get(srow, scol) {
                if item == acre {
                    count += 1;
                }
            }
        }
    }
    count
}

#[allow(dead_code)]
fn print(v: &Array2D<Acre>) {
    for row in 0..v.num_rows() {
        for col in 0..v.num_columns() {
            print!("{}", v[(row, col)]);
        }
        println!();
    }
}

fn value(v: &Array2D<Acre>) -> usize {
    v.elements_row_major_iter()
        .filter(|&&item| item == Acre::Tree)
        .count()
        * v.elements_row_major_iter()
            .filter(|&&item| item == Acre::Yard)
            .count()
}

fn parse(inp: &str) -> Array2D<Acre> {
    let mut v = vec![];
    for line in inp.lines() {
        let mut acc = vec![];
        for c in line.chars() {
            let acre = match c {
                '.' => Acre::Open,
                '|' => Acre::Tree,
                '#' => Acre::Yard,
                _ => unreachable!(),
            };
            acc.push(acre);
        }
        v.push(acc);
    }
    Array2D::from_rows(&v).unwrap()
}

fn step(v: &Array2D<Acre>) -> Array2D<Acre> {
    let mut next = v.clone();
    for row in 0..v.num_rows() {
        for col in 0..v.num_columns() {
            match v[(row, col)] {
                Acre::Open => {
                    if count_adj(v, row, col, Acre::Tree) >= 3 {
                        next[(row, col)] = Acre::Tree;
                    }
                }
                Acre::Tree => {
                    if count_adj(v, row, col, Acre::Yard) >= 3 {
                        next[(row, col)] = Acre::Yard;
                    }
                }
                Acre::Yard => {
                    if count_adj(v, row, col, Acre::Yard) == 0
                        || count_adj(v, row, col, Acre::Tree) == 0
                    {
                        next[(row, col)] = Acre::Open;
                    }
                }
            };
        }
    }
    next
}

fn part1(inp: &str) -> usize {
    let mut v = parse(inp);
    for _minute in 0..10 {
        v = step(&v);
    }
    value(&v)
}

fn part2(inp: &str) -> usize {
    let mut v = parse(inp);
    let mut memo = HashMap::new();
    let mut minute = 0;
    while minute < 1000000000 {
        if let Some(prev) = memo.get(&v) {
            let delta = minute - prev;
            let remaining = 1000000000 - minute;
            let bump = remaining / delta * delta;
            if bump > 0 {
                minute += bump;
                memo.clear();
                continue;
            }
        }
        memo.insert(v.clone(), minute);
        v = step(&v);
        minute += 1;
    }
    value(&v)
}

xaoc::xaoc!(
    sample = r#".#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|."#
);
