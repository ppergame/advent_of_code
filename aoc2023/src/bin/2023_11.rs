use std::collections::HashMap;

use itertools::Itertools as _;

fn part1(inp: &str) -> usize {
    solve(inp, 2)
}

fn part2(inp: &str) -> usize {
    solve(inp, 1000000)
}

fn solve(inp: &str, mult: usize) -> usize {
    let mut gals = HashMap::new();
    let mut row_offset = 0;
    let mut max_row = 0;
    let mut max_col = 0;
    for (row, line) in inp.lines().enumerate() {
        max_row = max_row.max(row);
        let mut found = false;
        for (col, c) in line.chars().enumerate() {
            max_col = max_col.max(col);
            if c == '#' {
                found = true;
                gals.insert((row, col), (row + row_offset, 0));
            }
        }
        if !found {
            row_offset += mult - 1;
        }
    }
    let mut col_offset = 0;
    for col in 0..=max_col {
        let mut found = false;
        for row in 0..=max_row {
            if let Some(pos) = gals.get_mut(&(row, col)) {
                found = true;
                *pos = (pos.0, col + col_offset);
            }
        }
        if !found {
            col_offset += mult - 1;
        }
    }
    let mut ret = 0;
    for ((row1, col1), (row2, col2)) in gals.values().tuple_combinations() {
        ret += row1.abs_diff(*row2) + col1.abs_diff(*col2);
    }
    ret
}

xaoc::xaoc!(
    sample = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."
);
