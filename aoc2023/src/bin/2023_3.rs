use std::collections::{HashMap, HashSet};
use std::iter::once;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Number {
    // row, col of first digit
    id: (usize, usize),
    val: usize,
}

struct Map {
    map: HashMap<(usize, usize), char>,
    max_row: usize,
    max_col: usize,
    adj: HashMap<(usize, usize), HashSet<Number>>,
}

impl Map {
    fn new(inp: &str) -> Self {
        let mut map = HashMap::new();
        // coordinate -> all adjacent numbers
        let mut adj = HashMap::<(usize, usize), HashSet<Number>>::new();
        let mut max_row = 0;
        let mut max_col = 0;
        for (row, line) in inp.lines().enumerate() {
            max_row = max_row.max(row);
            let mut acc = vec![];
            // col of first digit in current number
            let mut start = None;
            for (col, c) in line.chars().chain(once('.')).enumerate() {
                max_col = max_col.max(col);
                map.insert((row, col), c);
                match (c.is_ascii_digit(), start) {
                    (true, None) => {
                        start = Some(col);
                        acc = vec![c];
                    }
                    (true, Some(_)) => {
                        acc.push(c);
                    }
                    (false, None) => {}
                    (false, Some(start_col)) => {
                        let val = acc.iter().collect::<String>().parse::<usize>().unwrap();
                        let num = Number {
                            id: (row, start_col),
                            val,
                        };
                        for ccol in start_col..col {
                            for (arow, acol) in adj_cells(row, ccol) {
                                adj.entry((arow, acol)).or_default().insert(num.clone());
                            }
                        }
                        acc.clear();
                        start = None;
                    }
                }
            }
        }
        Self {
            map,
            max_row,
            max_col,
            adj,
        }
    }
}

fn adj_cells(row: usize, col: usize) -> Vec<(usize, usize)> {
    let mut ret = vec![];
    if row > 0 {
        ret.push((row - 1, col));
        ret.push((row - 1, col + 1));
    }
    if col > 0 {
        ret.push((row, col - 1));
        ret.push((row + 1, col - 1));
    }
    if row > 0 && col > 0 {
        ret.push((row - 1, col - 1));
    }
    ret.push((row + 1, col));
    ret.push((row, col + 1));
    ret.push((row + 1, col + 1));
    ret
}

fn part1(inp: &str) -> usize {
    let map = Map::new(inp);
    let mut seen = HashSet::new();
    let mut res = 0;
    for row in 0..=map.max_row {
        for col in 0..=map.max_col {
            let c = map.map[&(row, col)];
            if !c.is_ascii_digit() && c != '.' {
                for opt in map.adj.get(&(row, col)).iter() {
                    for num in opt.iter() {
                        if seen.contains(&num.id) {
                            continue;
                        }
                        res += num.val;
                        seen.insert(num.id);
                    }
                }
            }
        }
    }
    res
}

fn part2(inp: &str) -> usize {
    let map = Map::new(inp);
    let mut res = 0;
    for row in 0..=map.max_row {
        for col in 0..=map.max_col {
            let c = map.map[&(row, col)];
            if c == '*' {
                for opt in map.adj.get(&(row, col)).iter() {
                    if opt.len() == 2 {
                        res += opt.iter().map(|n| n.val).product::<usize>();
                    }
                }
            }
        }
    }
    res
}

xaoc::xaoc!(
    sample = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
);
