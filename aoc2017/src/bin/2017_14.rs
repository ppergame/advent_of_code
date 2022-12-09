use std::collections::HashSet;

use itertools::Itertools;
use pathfinding::prelude::strongly_connected_components;

fn knot_hash(s: &str) -> String {
    let mut s = s.chars().map(|c| c as usize).collect_vec();
    s.extend([17, 31, 73, 47, 23]);
    let mut list = (0..=255).collect_vec();
    let mut pos = 0;
    let mut skip = 0;
    for _ in 0..64 {
        for &len in &s {
            list[..len].reverse();
            pos = (pos + len + skip) % list.len();
            let list_len = list.len();
            list.rotate_left((len + skip) % list_len);
            skip += 1;
        }
    }
    list.rotate_right(pos);
    let v = list
        .chunks(16)
        .map(|chunk| chunk.iter().copied().reduce(|a, b| a ^ b).unwrap())
        .collect_vec();
    hex::encode(v)
}

fn part1(inp: &str) -> u32 {
    let mut count = 0;
    for i in 0..128 {
        for c in knot_hash(&format!("{inp}-{i}")).chars() {
            count += c.to_digit(16).unwrap().count_ones();
        }
    }
    count
}

fn part2(inp: &str) -> usize {
    let mut map = HashSet::new();
    for row in 0..128 {
        for (quad, c) in knot_hash(&format!("{inp}-{row}")).chars().enumerate() {
            for (subquad, d) in format!("{:4b}", c.to_digit(16).unwrap())
                .chars()
                .enumerate()
            {
                if d == '1' {
                    map.insert((row as i64, (quad * 4 + subquad) as i64));
                }
            }
        }
    }
    strongly_connected_components(&map.iter().copied().collect_vec(), |&(row, col)| {
        let mut ret = vec![];
        for (nr, nc) in [
            (row - 1, col),
            (row + 1, col),
            (row, col - 1),
            (row, col + 1),
        ] {
            if map.contains(&(nr, nc)) {
                ret.push((nr, nc));
            }
        }
        ret
    })
    .len()
}

xaoc::xaoc!(sample = "flqrgnkx");
