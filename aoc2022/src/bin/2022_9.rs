use itertools::Itertools;
use sscanf::scanf;
use std::collections::{HashMap, HashSet};

lazy_static::lazy_static! {
    static ref DIRS: HashMap<char, (i64, i64)> = HashMap::from([
        ('R', (0, 1)), ('U', (-1, 0)), ('L', (0, -1)), ('D', (1, 0))]);
}

fn catch_up(mut tail: (i64, i64), head: (i64, i64)) -> (i64, i64) {
    let (tdr, tdc) = (head.0 - tail.0, head.1 - tail.1);
    let tdrs = tdr.signum();
    let tdcs = tdc.signum();
    if tdr.abs() > 1 || tdc.abs() > 1 {
        tail = (tail.0 + tdrs, tail.1 + tdcs);
    }
    tail
}

fn part1(inp: &str) -> usize {
    let mut head = (0, 0);
    let mut tail = (0, 0);
    let mut map = HashSet::new();
    map.insert(tail);
    for line in inp.lines() {
        let (dir, count) = scanf!(line, "{} {}", char, i64).unwrap();
        for _ in 0..count {
            let (dr, dc) = DIRS[&dir];
            head.0 += dr;
            head.1 += dc;
            tail = catch_up(tail, head);
            map.insert(tail);
        }
    }
    map.len()
}

fn part2(mut inp: &str) -> usize {
    if inp.lines().count() < 20 {
        inp = r#"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"#;
    }
    let mut rope = std::iter::repeat((0, 0)).take(10).collect_vec();
    let mut map = HashSet::new();
    map.insert(*rope.last().unwrap());
    for line in inp.lines() {
        let (dir, count) = scanf!(line, "{} {}", char, i64).unwrap();
        for _ in 0..count {
            let (dr, dc) = DIRS[&dir];
            rope[0].0 += dr;
            rope[0].1 += dc;
            for i in 0..rope.len() - 1 {
                rope[i + 1] = catch_up(rope[i + 1], rope[i]);
                map.insert(*rope.last().unwrap());
            }
        }
    }
    map.len()
}

xaoc::xaoc!(sample_idx = 5);
