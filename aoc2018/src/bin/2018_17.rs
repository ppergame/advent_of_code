use itertools::{Itertools, MinMaxResult};
use sscanf::scanf;
use std::collections::{HashMap, HashSet};

struct Run {
    map: HashSet<(i64, i64)>,
    reach: HashMap<(i64, i64), bool>,
    top: i64,
    bottom: i64,
}

impl Run {
    fn new(inp: &str) -> Self {
        let mut map = HashSet::new();
        let mut top = i64::MAX;
        let mut bottom = i64::MIN;
        for line in inp.lines() {
            if let Ok((col, row1, row2)) = scanf!(line, "x={}, y={}..{}", i64, i64, i64) {
                for row in row1..=row2 {
                    top = top.min(row);
                    bottom = bottom.max(row);
                    map.insert((row, col));
                }
            } else if let Ok((row, col1, col2)) = scanf!(line, "y={}, x={}..{}", i64, i64, i64) {
                top = top.min(row);
                bottom = bottom.max(row);
                for col in col1..=col2 {
                    map.insert((row, col));
                }
            } else {
                unreachable!();
            }
        }
        // coord -> is_standing_water
        let mut reach = HashMap::new();
        let mut stack = vec![];
        stack.push((0, 500));
        while let Some((row, col)) = stack.pop() {
            if row > bottom {
                continue;
            }
            match (map.contains(&(row + 1, col)), reach.get(&(row + 1, col))) {
                (false, None) => {
                    reach.insert((row, col), false);
                    stack.push((row + 1, col))
                }
                (true, _) | (_, Some(true)) => {
                    let mut left = col;
                    let left_closed = loop {
                        if map.contains(&(row, left - 1)) {
                            break true;
                        }
                        if !map.contains(&(row + 1, left - 1))
                            && reach.get(&(row + 1, left - 1)) != Some(&true)
                        {
                            break false;
                        }
                        left -= 1;
                    };
                    let mut right = col;
                    let right_closed = loop {
                        if map.contains(&(row, right + 1)) {
                            break true;
                        }
                        if !map.contains(&(row + 1, right + 1))
                            && reach.get(&(row + 1, right + 1)) != Some(&true)
                        {
                            break false;
                        }
                        right += 1;
                    };
                    for col in left..=right {
                        reach.insert((row, col), left_closed && right_closed);
                    }
                    if !left_closed {
                        stack.push((row, left - 1));
                    }
                    if !right_closed {
                        stack.push((row, right + 1));
                    }
                    if left_closed && right_closed {
                        stack.push((row - 1, col));
                    }
                }
                (false, Some(false)) => {
                    reach.insert((row, col), false);
                }
            }
        }
        Self {
            map,
            reach,
            top,
            bottom,
        }
    }

    #[allow(dead_code)]
    fn print(&self) {
        println!();
        let MinMaxResult::MinMax(minr, maxr) = self.map.iter().map(|(row, _)| row).copied().minmax() else { unreachable!() };
        let MinMaxResult::MinMax(minc, maxc) = self.map.iter().map(|(_, col)| col).copied().minmax() else { unreachable!() };
        for row in minr - 5..=maxr + 5 {
            for col in minc - 5..=maxc + 5 {
                let mut c = if self.map.contains(&(row, col)) {
                    '#'
                } else {
                    ' '
                };
                match self.reach.get(&(row, col)) {
                    None => (),
                    Some(true) => c = '~',
                    Some(false) => c = '|',
                }
                if (row, col) == (0, 500) {
                    c = '+';
                }
                print!("{c}");
            }
            println!();
        }
    }
}

fn part1(inp: &str) -> usize {
    let run = Run::new(inp);
    run.reach
        .into_keys()
        .filter(|&(row, _)| row >= run.top && row <= run.bottom)
        .count()
}

fn part2(inp: &str) -> usize {
    let run = Run::new(inp);
    // run.print();
    run.reach
        .into_iter()
        .filter(|&((row, _), fill)| fill && row >= run.top && row <= run.bottom)
        .count()
}

xaoc::xaoc!(sample_idx = 3);
