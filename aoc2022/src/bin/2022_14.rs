use itertools::Itertools;
use std::collections::HashSet;

fn parse(inp: &str) -> (HashSet<(i64, i64)>, i64) {
    let mut map = HashSet::<(i64, i64)>::new();
    let mut bottom = i64::MIN;
    for line in inp.lines() {
        for ((mut col1, mut row1), (mut col2, mut row2)) in line
            .split(" -> ")
            .map(|s| {
                let (a, b) = s.split_once(',').unwrap();
                (a.parse().unwrap(), b.parse().unwrap())
            })
            .tuple_windows()
        {
            if col1 > col2 {
                std::mem::swap(&mut col1, &mut col2);
            }
            if row1 > row2 {
                std::mem::swap(&mut row1, &mut row2);
            }
            for row in row1..=row2 {
                bottom = bottom.max(row);
                for col in col1..=col2 {
                    map.insert((row, col));
                }
            }
        }
    }
    (map, bottom)
}

fn part1(inp: &str) -> i64 {
    let (mut map, bottom) = parse(inp);
    let mut round = 0;
    loop {
        let (mut row, mut col) = (0, 500);
        while row < bottom {
            // eprintln!("{row} {col}");
            if !map.contains(&(row + 1, col)) {
                row += 1;
            } else if !map.contains(&(row + 1, col - 1)) {
                row += 1;
                col -= 1;
            } else if !map.contains(&(row + 1, col + 1)) {
                row += 1;
                col += 1;
            } else {
                map.insert((row, col));
                break;
            }
        }
        if row >= bottom {
            break;
        }
        round += 1;
    }
    round
}

fn part2(inp: &str) -> i64 {
    let (mut map, bottom) = parse(inp);
    let mut round = 0;
    loop {
        let (mut row, mut col) = (0, 500);
        loop {
            if row == bottom + 1 {
                map.insert((row, col));
                break;
            }
            if !map.contains(&(row + 1, col)) {
                row += 1;
            } else if !map.contains(&(row + 1, col - 1)) {
                row += 1;
                col -= 1;
            } else if !map.contains(&(row + 1, col + 1)) {
                row += 1;
                col += 1;
            } else {
                map.insert((row, col));
                break;
            }
        }
        round += 1;
        if (row, col) == (0, 500) {
            break;
        }
    }
    round
}

xaoc::xaoc!(
    sample = r#"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"#
);
