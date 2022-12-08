use sscanf::scanf;
use std::collections::hash_map::Entry::Vacant;
use std::collections::HashMap;

#[derive(Debug)]
enum Target {
    Bot(usize),
    Output(usize),
}

impl Target {
    fn new(w: &str, n: usize) -> Self {
        match w {
            "bot" => Target::Bot(n),
            "output" => Target::Output(n),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Instr {
    low: Target,
    high: Target,
}

fn part1(inp: &str) -> usize {
    let mut bots = HashMap::<usize, Vec<usize>>::new();
    let mut bot_rules = HashMap::<usize, Instr>::new();
    let mut outputs = HashMap::<usize, Vec<usize>>::new();
    let mut work = vec![];
    for line in inp.lines() {
        if let Ok((value, bot)) = scanf!(line, "value {} goes to bot {}", usize, usize) {
            let v = bots.entry(bot).or_default();
            v.push(value);
            v.sort();
            assert!(v.len() <= 2);
            if v.len() == 2 {
                work.push(bot);
            }
        } else if let Ok((bot, low_w, low_n, high_w, high_n)) = scanf!(
            line,
            "bot {} gives low to {} {} and high to {} {}",
            usize,
            str,
            usize,
            str,
            usize
        ) {
            let Vacant(v) = bot_rules.entry(bot) else { unreachable!() };
            v.insert(Instr {
                low: Target::new(low_w, low_n),
                high: Target::new(high_w, high_n),
            });
        } else {
            unreachable!();
        }
    }
    while let Some(bot) = work.pop() {
        let instr = bot_rules.remove(&bot).unwrap();
        let &[low, high] = &bots.remove(&bot).unwrap()[..] else {unreachable!()};
        if low == 17 && high == 61 {
            return bot;
        }
        for (target, value) in [(&instr.low, low), (&instr.high, high)] {
            match target {
                Target::Bot(n) => {
                    let v = bots.entry(*n).or_default();
                    v.push(value);
                    v.sort();
                    assert!(v.len() <= 2);
                    if v.len() == 2 {
                        work.push(*n);
                    }
                }
                Target::Output(n) => outputs.entry(*n).or_default().push(value),
            }
        }
    }
    unreachable!();
}

fn part2(inp: &str) -> usize {
    let mut bots = HashMap::<usize, Vec<usize>>::new();
    let mut bot_rules = HashMap::<usize, Instr>::new();
    let mut outputs = HashMap::<usize, Vec<usize>>::new();
    let mut work = vec![];
    for line in inp.lines() {
        if let Ok((value, bot)) = scanf!(line, "value {} goes to bot {}", usize, usize) {
            let v = bots.entry(bot).or_default();
            v.push(value);
            v.sort();
            assert!(v.len() <= 2);
            if v.len() == 2 {
                work.push(bot);
            }
        } else if let Ok((bot, low_w, low_n, high_w, high_n)) = scanf!(
            line,
            "bot {} gives low to {} {} and high to {} {}",
            usize,
            str,
            usize,
            str,
            usize
        ) {
            let Vacant(v) = bot_rules.entry(bot) else { unreachable!() };
            v.insert(Instr {
                low: Target::new(low_w, low_n),
                high: Target::new(high_w, high_n),
            });
        } else {
            unreachable!();
        }
    }
    while let Some(bot) = work.pop() {
        let instr = bot_rules.remove(&bot).unwrap();
        let &[low, high] = &bots.remove(&bot).unwrap()[..] else {unreachable!()};
        for (target, value) in [(&instr.low, low), (&instr.high, high)] {
            match target {
                Target::Bot(n) => {
                    let v = bots.entry(*n).or_default();
                    v.push(value);
                    v.sort();
                    assert!(v.len() <= 2);
                    if v.len() == 2 {
                        work.push(*n);
                    }
                }
                Target::Output(n) => outputs.entry(*n).or_default().push(value),
            }
        }
    }
    outputs[&0].first().unwrap() * outputs[&1].first().unwrap() * outputs[&2].first().unwrap()
}

xaoc::xaoc!();
