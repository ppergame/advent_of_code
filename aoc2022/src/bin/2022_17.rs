use itertools::{Itertools, MinMaxResult};
use pathfinding::prelude::*;
use std::collections::{BTreeSet, HashMap};

fn move_down(block: &[(i64, i64)]) -> Vec<(i64, i64)> {
    block.iter().map(|&(row, col)| (row + 1, col)).collect()
}

fn move_left(block: &[(i64, i64)]) -> Vec<(i64, i64)> {
    let mut ret = vec![];
    for &(row, col) in block {
        if col == 0 {
            return block.to_vec();
        }
        ret.push((row, col - 1));
    }
    ret
}

fn move_right(block: &[(i64, i64)]) -> Vec<(i64, i64)> {
    let mut ret = vec![];
    for &(row, col) in block {
        if col == 6 {
            return block.to_vec();
        }
        ret.push((row, col + 1));
    }
    ret
}

fn get_pattern(pattern: usize, tower: &BTreeSet<(i64, i64)>) -> Vec<(i64, i64)> {
    let minr = tower.iter().map(|(row, _)| row).copied().min().unwrap();
    match pattern {
        0 => vec![(minr - 4, 2), (minr - 4, 3), (minr - 4, 4), (minr - 4, 5)],
        1 => vec![
            (minr - 4, 3),
            (minr - 5, 2),
            (minr - 5, 3),
            (minr - 5, 4),
            (minr - 6, 3),
        ],
        2 => vec![
            (minr - 4, 2),
            (minr - 4, 3),
            (minr - 4, 4),
            (minr - 5, 4),
            (minr - 6, 4),
        ],
        3 => vec![(minr - 4, 2), (minr - 5, 2), (minr - 6, 2), (minr - 7, 2)],
        4 => vec![(minr - 4, 2), (minr - 4, 3), (minr - 5, 2), (minr - 5, 3)],
        _ => unreachable!(),
    }
}

fn intersects(tower: &BTreeSet<(i64, i64)>, block: &[(i64, i64)]) -> bool {
    block.iter().any(|cc| tower.contains(cc))
}

#[allow(dead_code)]
fn print(tower: &BTreeSet<(i64, i64)>) {
    let MinMaxResult::MinMax(minr, maxr) = tower.iter().map(|(row, _)| row).copied().minmax() else { unreachable!() };
    let MinMaxResult::MinMax(minc, maxc) = tower.iter().map(|(_, col)| col).copied().minmax() else { unreachable!() };
    for row in minr..=maxr {
        for col in minc..=maxc {
            if tower.contains(&(row, col)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

#[allow(dead_code)]
fn print_block(tower: &BTreeSet<(i64, i64)>, block: &[(i64, i64)]) {
    let mut map =
        HashMap::<(i64, i64), char>::from_iter(tower.iter().map(|&(row, col)| ((row, col), '#')));
    for &(row, col) in block {
        map.insert((row, col), '@');
    }
    let MinMaxResult::MinMax(minr, maxr) = map.keys().map(|(row, _)| row).copied().minmax() else { unreachable!() };
    let MinMaxResult::MinMax(minc, maxc) = map.keys().map(|(_, col)| col).copied().minmax() else { unreachable!() };
    for row in minr..=maxr {
        for col in minc..=maxc {
            if let Some(c) = map.get(&(row, col)) {
                print!("{}", c);
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn trim(tower: BTreeSet<(i64, i64)>) -> (BTreeSet<(i64, i64)>, i64) {
    let MinMaxResult::MinMax(minr, maxr) = tower.iter().map(|(row, _)| row).copied().minmax() else { unreachable!() };
    let bottom = dfs_reach((minr - 1, 0i64), |&(row, col)| {
        let mut ret = vec![];
        if col > 0 {
            ret.push((row, col - 1));
        }
        if col < 6 {
            ret.push((row, col + 1));
        }
        if row >= minr {
            ret.push((row - 1, col));
        }
        ret.push((row + 1, col));
        ret.retain(|&(row, col)| !tower.contains(&(row, col)));
        ret
    })
    .map(|(row, _)| row)
    .max()
    .unwrap()
        + 1;
    let shift = maxr - bottom;
    let mut ret = BTreeSet::new();
    for (row, col) in tower {
        if row > bottom {
            continue;
        }
        ret.insert((row + shift, col));
    }
    (ret, shift)
}

fn part1(inp: &str) -> i64 {
    let mut seq = inp.chars().enumerate().cycle().peekable();
    let mut tower = BTreeSet::new();
    for col in 0..7 {
        // floor
        tower.insert((0, col));
    }
    let mut bottom = 0;
    let mut pattern = 0;
    for _ in 0..2022 {
        let mut block = get_pattern(pattern, &tower);
        pattern = (pattern + 1) % 5;
        loop {
            let side = match seq.next().unwrap().1 {
                '<' => move_left(&block),
                '>' => move_right(&block),
                _ => unreachable!(),
            };
            if !intersects(&tower, &side) {
                block = side;
            }
            let down = move_down(&block);
            if intersects(&tower, &down) {
                tower.extend(block);
                break;
            }
            block = down;
        }
        let (new_tower, shift) = trim(tower);
        tower = new_tower;
        bottom -= shift;
    }
    -tower.iter().map(|(row, _)| row).copied().min().unwrap() - bottom
}

fn part2(inp: &str) -> i64 {
    let mut seq = inp.chars().enumerate().cycle().peekable();
    let mut tower = BTreeSet::new();
    for col in 0..7 {
        // floor
        tower.insert((0, col));
    }
    let mut bottom = 0;
    let mut pattern = 0;
    let mut memo = HashMap::new();
    let mut idx = 0;
    let mut hit = false;
    const ITER: usize = 1000000000000;
    loop {
        if idx >= ITER {
            break;
        }
        let &(c_idx, _) = seq.peek().unwrap();
        if !hit {
            match memo.entry((pattern, c_idx, tower.clone())) {
                std::collections::hash_map::Entry::Occupied(o) => {
                    let &(prev_idx, prev_bottom) = o.get();
                    hit = true;
                    let diff = idx - prev_idx;
                    let mult = (ITER - idx) / diff;
                    idx += mult * diff;
                    bottom += (bottom - prev_bottom) * mult as i64;
                }
                std::collections::hash_map::Entry::Vacant(v) => {
                    v.insert((idx, bottom));
                }
            }
        }
        let mut block = get_pattern(pattern, &tower);
        pattern = (pattern + 1) % 5;
        loop {
            let side = match seq.next().unwrap().1 {
                '<' => move_left(&block),
                '>' => move_right(&block),
                _ => unreachable!(),
            };
            if !intersects(&tower, &side) {
                block = side;
            }
            let down = move_down(&block);
            if intersects(&tower, &down) {
                tower.extend(block);
                break;
            }
            block = down;
        }
        let (new_tower, shift) = trim(tower);
        tower = new_tower;
        bottom -= shift;
        idx += 1;
    }
    -tower.iter().map(|(row, _)| row).copied().min().unwrap() - bottom
}

xaoc::xaoc!(sample = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>");
