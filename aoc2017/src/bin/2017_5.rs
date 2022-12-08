use itertools::Itertools;

fn part1(inp: &str) -> i64 {
    let mut jumps = inp.lines().map(|l| l.parse::<i64>().unwrap()).collect_vec();
    let mut pc: i64 = 0;
    let mut steps = 0;
    while pc >= 0 && pc < jumps.len() as i64 {
        steps += 1;
        let prev = pc;
        pc += jumps[pc as usize];
        jumps[prev as usize] += 1;
    }
    steps
}

fn part2(inp: &str) -> i64 {
    let mut jumps = inp.lines().map(|l| l.parse::<i64>().unwrap()).collect_vec();
    let mut pc: i64 = 0;
    let mut steps = 0;
    while pc >= 0 && pc < jumps.len() as i64 {
        steps += 1;
        let prev = pc;
        let offset = jumps[pc as usize];
        pc += jumps[pc as usize];
        if offset >= 3 {
            jumps[prev as usize] -= 1;
        } else {
            jumps[prev as usize] += 1;
        }
    }
    steps
}

xaoc::xaoc!();
