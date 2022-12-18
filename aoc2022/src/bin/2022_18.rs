use pathfinding::prelude::*;
use sscanf::scanf;
use std::collections::HashSet;

fn adj((x, y, z): (i64, i64, i64)) -> Vec<(i64, i64, i64)> {
    vec![
        (x - 1, y, z),
        (x + 1, y, z),
        (x, y - 1, z),
        (x, y + 1, z),
        (x, y, z - 1),
        (x, y, z + 1),
    ]
}

fn part1(inp: &str) -> i64 {
    let mut cubes = HashSet::new();
    for line in inp.lines() {
        let (x, y, z) = scanf!(line, "{},{},{}", i64, i64, i64).unwrap();
        cubes.insert((x, y, z));
    }
    let mut count = 0;
    for cube in &cubes {
        for adj_cube in adj(*cube) {
            if !cubes.contains(&adj_cube) {
                count += 1;
            }
        }
    }
    count
}

fn part2(inp: &str) -> i64 {
    let mut cubes = HashSet::new();
    let (mut min_x, mut max_x, mut min_y, mut max_y, mut min_z, mut max_z) =
        (i64::MAX, i64::MIN, i64::MAX, i64::MIN, i64::MAX, i64::MIN);
    for line in inp.lines() {
        let (x, y, z) = scanf!(line, "{},{},{}", i64, i64, i64).unwrap();
        cubes.insert((x, y, z));
        min_x = min_x.min(x);
        max_x = max_x.max(x);
        min_y = min_y.min(y);
        max_y = max_y.max(y);
        min_z = min_z.min(z);
        max_z = max_z.max(z);
    }
    min_x -= 1;
    max_x += 1;
    min_y -= 1;
    max_y += 1;
    min_z -= 1;
    max_z += 1;
    let air = dfs_reach((min_x, min_y, min_z), |&(x, y, z)| {
        let mut ret = adj((x, y, z));
        ret.retain(|&(ax, ay, az)| {
            ax >= min_x
                && ax <= max_x
                && ay >= min_y
                && ay <= max_y
                && az >= min_z
                && az <= max_z
                && !cubes.contains(&(ax, ay, az))
        });
        ret
    })
    .collect::<HashSet<_>>();

    let mut count = 0;
    for cube in &cubes {
        for adj_cube in adj(*cube) {
            if !cubes.contains(&adj_cube) && air.contains(&adj_cube) {
                count += 1;
            }
        }
    }
    count
}

xaoc::xaoc!(sample_idx = 4);
