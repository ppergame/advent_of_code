use itertools::{Itertools, MinMaxResult};
use sscanf::scanf;
use std::collections::HashMap;

fn part1(inp: &str) -> usize {
    let mut map = HashMap::<(i64, i64), usize>::new();
    for line in inp.lines() {
        let Ok((_idx, start_col, start_row, cols, rows)) = scanf!(line, "#{} @ {},{}: {}x{}", usize, i64, i64, i64, i64) else { unreachable!() };
        for row in start_row..start_row + rows {
            for col in start_col..start_col + cols {
                *map.entry((row, col)).or_default() += 1;
            }
        }
    }
    map.into_values().filter(|&v| v >= 2).count()
}

#[allow(dead_code)]
fn print(map: &HashMap<(i64, i64), usize>) {
    let MinMaxResult::MinMax(minr, maxr) = map.keys().map(|(row, _)| row).copied().minmax() else { unreachable!() };
    let MinMaxResult::MinMax(minc, maxc) = map.keys().map(|(_, col)| col).copied().minmax() else { unreachable!() };
    for row in minr..=maxr {
        for col in minc..=maxc {
            print!("{}", map.get(&(row, col)).unwrap_or(&0));
        }
        println!();
    }
}

fn part2(inp: &str) -> usize {
    let mut map = HashMap::<(i64, i64), usize>::new();
    for line in inp.lines() {
        let Ok((_idx, start_col, start_row, cols, rows)) = scanf!(line, "#{} @ {},{}: {}x{}", usize, i64, i64, i64, i64) else { unreachable!() };
        for row in start_row..start_row + rows {
            for col in start_col..start_col + cols {
                *map.entry((row, col)).or_default() += 1;
            }
        }
    }
    for line in inp.lines() {
        let Ok((idx, start_col, start_row, cols, rows)) = scanf!(line, "#{} @ {},{}: {}x{}", usize, i64, i64, i64, i64) else { unreachable!() };
        let mut bad = false;
        for row in start_row..start_row + rows {
            for col in start_col..start_col + cols {
                if map[&(row, col)] >= 2 {
                    bad = true;
                    break;
                }
            }
        }
        if !bad {
            return idx;
        }
    }
    unreachable!();
}

xaoc::xaoc!(sample_idx = 10);
