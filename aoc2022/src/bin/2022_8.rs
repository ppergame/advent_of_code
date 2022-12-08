use std::collections::HashMap;

fn part1(inp: &str) -> i64 {
    let mut grid = HashMap::new();
    let mut width = 0;
    let mut height = 0;
    for (row, line) in inp.lines().enumerate() {
        height = height.max(row);
        for (col, c) in line.chars().enumerate() {
            width = width.max(col);
            grid.insert((row, col), c.to_digit(10).unwrap() as u8);
        }
    }
    width += 1;
    height += 1;
    let mut count = 0;
    for (&(row, col), &d) in &grid {
        if (0..row).map(|r| grid[&(r, col)]).all(|nd| nd < d)
            || (row + 1..height).map(|r| grid[&(r, col)]).all(|nd| nd < d)
            || (0..col).map(|c| grid[&(row, c)]).all(|nd| nd < d)
            || (col + 1..width).map(|c| grid[&(row, c)]).all(|nd| nd < d)
        {
            count += 1;
        }
    }
    count
}

fn part2(inp: &str) -> i64 {
    let mut grid = HashMap::new();
    let mut width = 0;
    let mut height = 0;
    for (row, line) in inp.lines().enumerate() {
        height = height.max(row);
        for (col, c) in line.chars().enumerate() {
            width = width.max(col);
            grid.insert((row, col), c.to_digit(10).unwrap() as u8);
        }
    }
    width += 1;
    height += 1;
    let mut max_score = 0;
    for (&(row, col), &d) in &grid {
        let mut score = 1;
        let mut count = 0;
        for r in (0..row).rev() {
            count += 1;
            if grid[&(r, col)] >= d {
                break;
            }
        }
        score *= count;
        let mut count = 0;
        for r in row + 1..height {
            count += 1;
            if grid[&(r, col)] >= d {
                break;
            }
        }
        score *= count;
        let mut count = 0;
        for c in (0..col).rev() {
            count += 1;
            if grid[&(row, c)] >= d {
                break;
            }
        }
        score *= count;
        let mut count = 0;
        for c in col + 1..width {
            count += 1;
            if grid[&(row, c)] >= d {
                break;
            }
        }
        score *= count;
        max_score = max_score.max(score);
    }
    max_score
}

xaoc::xaoc!();
