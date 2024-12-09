use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

#[derive(Debug)]
struct Map {
    a: HashMap<char, Vec<(i64, i64)>>,
    max_row: i64,
    max_col: i64,
}

impl Map {
    fn parse(inp: &str) -> Self {
        let mut a = HashMap::new();
        let mut max_row = 0;
        let mut max_col = 0;
        for (row, line) in inp.lines().enumerate() {
            let row = row as i64;
            max_row = max_row.max(row);
            for (col, c) in line.chars().enumerate() {
                let col = col as i64;
                max_col = max_col.max(col);
                if c == '.' {
                    continue;
                }
                a.entry(c).or_insert(vec![]).push((row, col));
            }
        }
        Self {
            a,
            max_row,
            max_col,
        }
    }

    fn is_valid(&self, (row, col): (i64, i64)) -> bool {
        row >= 0 && row <= self.max_row && col >= 0 && col <= self.max_col
    }
}

fn part1(inp: &str) -> usize {
    let map = Map::parse(inp);
    let mut ret = HashSet::new();
    for v in map.a.values() {
        for ((r1, c1), (r2, c2)) in v.iter().tuple_combinations() {
            let dr = r2 - r1;
            let dc = c2 - c1;
            let p1 = (r1 - dr, c1 - dc);
            if map.is_valid(p1) {
                ret.insert(p1);
            }
            let p2 = (r2 + dr, c2 + dc);
            if map.is_valid(p2) {
                ret.insert(p2);
            }
        }
    }
    ret.len()
}

fn part2(inp: &str) -> usize {
    let map = Map::parse(inp);
    let mut ret = HashSet::new();
    for v in map.a.values() {
        for (&(r1, c1), &(r2, c2)) in v.iter().tuple_combinations() {
            let dr = r2 - r1;
            let dc = c2 - c1;
            let mut r = r1;
            let mut c = c1;
            while map.is_valid((r, c)) {
                ret.insert((r, c));
                r -= dr;
                c -= dc;
            }
            let mut r = r2;
            let mut c = c2;
            while map.is_valid((r, c)) {
                ret.insert((r, c));
                r += dr;
                c += dc;
            }
        }
    }
    ret.len()
}

xaoc::xaoc!();
