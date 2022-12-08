use aoc2016::assem::{BRes, Bunny};
use std::collections::HashMap;

fn part1(inp: &str) -> i64 {
    'outer: for a in 0..10000 {
        let mut expected = 0;
        let mut b = Bunny::parse(inp);
        b.a = a;
        let mut state_cache = HashMap::new();
        loop {
            let val = loop {
                match b.step() {
                    BRes::Ok => (),
                    BRes::Done => continue 'outer,
                    BRes::Out(val) => break val,
                }
            };
            if val != expected {
                continue 'outer;
            }
            expected = (expected == 0) as i64;
            let state = b.state();
            if let Some(prev_val) = state_cache.get(&state) {
                if *prev_val == val {
                    return a;
                } else {
                    continue 'outer;
                }
            }
            state_cache.insert(state, val);
        }
    }
    unreachable!();
}

fn part2(_inp: &str) -> i64 {
    0
}

xaoc::xaoc!(no_sample = true);
