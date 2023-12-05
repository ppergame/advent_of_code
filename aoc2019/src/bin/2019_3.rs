use std::collections::HashMap;
use std::collections::HashSet;
use vecmat::{prelude::*, Vector};

type Vec2 = Vector<i32, 2>;

fn path_to_steps(s: &[u8]) -> HashMap<(i32, i32), i32> {
    let mut res = HashMap::<(i32, i32), i32>::new();
    let mut loc = Vec2::zero();
    let mut totalsteps = 0;
    for p in s.split(|i| *i == b',') {
        let dloc = match p[0] {
            b'U' => Vec2::from([0, -1]),
            b'R' => Vec2::from([1, 0]),
            b'D' => Vec2::from([0, 1]),
            b'L' => Vec2::from([-1, 0]),
            x => panic!("unexpected direction {}", x),
        };
        let steps: u32 = std::str::from_utf8(&p[1..]).unwrap().parse().unwrap();
        for _ in 0..steps {
            loc += dloc;
            totalsteps += 1;
            res.entry((loc[0], loc[1])).or_insert(totalsteps);
        }
    }
    res
}

fn part1(inp: &str) -> i32 {
    let lines: Vec<&str> = inp.split_whitespace().collect();
    let s1 = path_to_steps(lines[0].as_bytes())
        .into_keys()
        .collect::<HashSet<(i32, i32)>>();
    let s2 = path_to_steps(lines[1].as_bytes())
        .into_keys()
        .collect::<HashSet<(i32, i32)>>();
    let mut common = s1
        .intersection(&s2)
        .map(|(x, y)| x.abs() + y.abs())
        .collect::<Vec<i32>>();
    common.sort_unstable();
    common[0]
}

fn part2(inp: &str) -> i32 {
    let lines: Vec<&str> = inp.split_whitespace().collect();
    let s1 = path_to_steps(lines[0].as_bytes());
    let s2 = path_to_steps(lines[1].as_bytes());
    let mut lowest = -1;
    for (loc, steps1) in s1 {
        if let Some(steps2) = s2.get(&loc) {
            if lowest == -1 || lowest > steps1 + steps2 {
                lowest = steps1 + steps2;
            }
        }
    }
    lowest
}

xaoc::xaoc!();
