use std::collections::{HashMap, HashSet};

use itertools::{Itertools, MinMaxResult};
use sscanf::scanf;

fn parse(inp: &str) -> Vec<(i64, i64)> {
    inp.lines()
        .map(|line| {
            let (col, row) = scanf!(line, "{}, {}", i64, i64).unwrap();
            (row, col)
        })
        .collect()
}

fn dist((row1, col1): (i64, i64), (row2, col2): (i64, i64)) -> i64 {
    (row1 - row2).abs() + (col1 - col2).abs()
}

fn part1(inp: &str) -> usize {
    let inp = parse(inp);
    let MinMaxResult::MinMax(minr, maxr) = inp.iter().map(|(row, _)| row).copied().minmax() else { unreachable!() };
    let MinMaxResult::MinMax(minc, maxc) = inp.iter().map(|(_, col)| col).copied().minmax() else { unreachable!() };
    let mut areas = HashMap::<(i64, i64), usize>::new();
    let mut infinite = HashSet::<(i64, i64)>::new();
    for row in minr..=maxr {
        for col in minc..=maxc {
            let dists = inp
                .iter()
                .map(|&pos| (dist((row, col), pos), pos))
                .sorted()
                .collect_vec();
            let (dist, pos) = dists[0];
            if dist != dists[1].0 {
                *areas.entry(pos).or_default() += 1;
                if row == minr || row == maxr || col == minc || col == maxc {
                    infinite.insert(pos);
                }
            }
        }
    }
    areas
        .into_iter()
        .filter_map(|(pos, area)| {
            if infinite.contains(&pos) {
                None
            } else {
                Some(area)
            }
        })
        .max()
        .unwrap()
}

fn part2(inp: &str) -> i64 {
    let inp = parse(inp);
    let MinMaxResult::MinMax(mut minr, mut maxr) = inp.iter().map(|(row, _)| row).copied().minmax() else { unreachable!() };
    let MinMaxResult::MinMax(mut minc, mut maxc) = inp.iter().map(|(_, col)| col).copied().minmax() else { unreachable!() };
    let mut count;
    loop {
        count = 0;
        let mut boundary_leak = false;
        for row in minr..=maxr {
            for col in minc..=maxc {
                let dist = inp.iter().map(|&pos| dist((row, col), pos)).sum::<i64>();
                if dist < 10000 {
                    count += 1;
                }
                if (row == minr || row == maxr || col == minc || col == maxc) && dist < 10000 {
                    boundary_leak = true;
                }
            }
        }
        if !boundary_leak {
            break;
        }
        let height = maxr - minr;
        minr -= height;
        maxr += height;
        let width = maxc - minc;
        minc -= width;
        maxc += width;
    }
    count
}

xaoc::xaoc!();
