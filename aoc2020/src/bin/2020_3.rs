use std::collections::HashSet;

type Coord = (i64, i64);

fn part1(inp: &str) -> i64 {
    let mut trees = HashSet::<Coord>::new();
    for (row, line) in inp.trim().lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == '#' {
                trees.insert((col as i64, row as i64));
            }
        }
    }
    let width = trees.iter().max_by_key(|(x, _)| x).unwrap().0 + 1;
    let height = trees.iter().max_by_key(|(_, y)| y).unwrap().1 + 1;
    let mut pos: Coord = (0, 0);
    let mut trees_hit = 0;
    while pos.1 < height {
        pos.0 += 3;
        pos.1 += 1;
        if trees.contains(&(pos.0.rem_euclid(width), pos.1)) {
            trees_hit += 1;
        }
    }
    trees_hit
}

fn part2(inp: &str) -> i64 {
    let mut trees = HashSet::<Coord>::new();
    for (row, line) in inp.trim().lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == '#' {
                trees.insert((col as i64, row as i64));
            }
        }
    }
    let width = trees.iter().max_by_key(|(x, _)| x).unwrap().0 + 1;
    let height = trees.iter().max_by_key(|(_, y)| y).unwrap().1 + 1;

    let hits = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|(dx, dy)| {
            let mut pos: Coord = (0, 0);
            let mut trees_hit: i64 = 0;
            while pos.1 < height {
                pos.0 += dx;
                pos.1 += dy;
                if trees.contains(&(pos.0.rem_euclid(width), pos.1)) {
                    trees_hit += 1;
                }
            }
            //println!("{} {} {}", dx, dy, trees_hit);
            trees_hit
        });
    hits.reduce(|a, b| {
        //println!("reducing {} {} {}", a, b, a * b);
        a * b
    })
    .unwrap()
}

xaoc::xaoc!();
