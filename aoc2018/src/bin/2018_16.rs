use itertools::Itertools;
use num::FromPrimitive;
use num_derive::FromPrimitive as FromPrimitiveMacro;
use sscanf::scanf;
use std::collections::{HashMap, HashSet};

#[derive(FromPrimitiveMacro)]
enum Opcode {
    Addr = 0,
    Addi = 1,
    Mulr = 2,
    Muli = 3,
    Banr = 4,
    Bani = 5,
    Borr = 6,
    Bori = 7,
    Setr = 8,
    Seti = 9,
    Gtir = 10,
    Gtri = 11,
    Gtrr = 12,
    Eqir = 13,
    Eqri = 14,
    Eqrr = 15,
}

#[derive(Copy, Clone)]
struct Machine([usize; 4]);

impl Machine {
    fn step(&self, cmd: Opcode, a: usize, b: usize, c: usize) -> Option<Self> {
        let mut next = *self;
        *next.0.get_mut(c)? = match cmd {
            Opcode::Addr => self.0.get(a)? + self.0.get(b)?,
            Opcode::Addi => self.0.get(a)? + b,
            Opcode::Mulr => self.0.get(a)? * self.0.get(b)?,
            Opcode::Muli => self.0.get(a)? * b,
            Opcode::Banr => self.0.get(a)? & self.0.get(b)?,
            Opcode::Bani => self.0.get(a)? & b,
            Opcode::Borr => self.0.get(a)? | self.0.get(b)?,
            Opcode::Bori => self.0.get(a)? | b,
            Opcode::Setr => *self.0.get(a)?,
            Opcode::Seti => a,
            Opcode::Gtir => (a > *self.0.get(b)?) as usize,
            Opcode::Gtri => (*self.0.get(a)? > b) as usize,
            Opcode::Gtrr => (self.0.get(a)? > self.0.get(b)?) as usize,
            Opcode::Eqir => (a == *self.0.get(b)?) as usize,
            Opcode::Eqri => (*self.0.get(a)? == b) as usize,
            Opcode::Eqrr => (self.0.get(a)? == self.0.get(b)?) as usize,
        };
        Some(next)
    }
}

fn collect4(mut iter: impl Iterator<Item = usize>) -> [usize; 4] {
    let ret = [
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
    ];
    assert!(iter.next().is_none());
    ret
}

fn part1(inp: &str) -> usize {
    let mut count = 0;
    let samples = inp.split("\n\n\n\n").next().unwrap();
    for sample in samples.split("\n\n") {
        let (before, instr, after) = sample.split('\n').tuples().next().unwrap();
        let input = collect4(
            scanf!(before, "Before: [{}]", str)
                .unwrap()
                .split(", ")
                .map(|s| s.parse().unwrap()),
        );
        let (_, a, b, c) = instr
            .split_ascii_whitespace()
            .map(|s| s.parse().unwrap())
            .tuples()
            .next()
            .unwrap();
        let expected = collect4(
            scanf!(after, "After:  [{}]", str)
                .unwrap()
                .split(", ")
                .map(|s| s.parse().unwrap()),
        );
        let mut match_count = 0;
        let m = Machine(input);
        for cmd in 0..16 {
            let cmd = Opcode::from_usize(cmd).unwrap();
            if let Some(res) = m.step(cmd, a, b, c) {
                if res.0 == expected {
                    match_count += 1;
                }
            }
        }
        if match_count >= 3 {
            count += 1;
        }
    }
    count
}

fn part2(inp: &str) -> usize {
    let (samples, prog) = inp.split_once("\n\n\n\n").unwrap();
    let mut possibles = std::iter::repeat(HashSet::<usize>::from_iter(0..16))
        .take(16)
        .collect_vec();
    for sample in samples.split("\n\n") {
        let (before, instr, after) = sample.lines().tuples().next().unwrap();
        let input = collect4(
            scanf!(before, "Before: [{}]", str)
                .unwrap()
                .split(", ")
                .map(|s| s.parse().unwrap()),
        );
        let (orig_cmd, a, b, c) = instr
            .split_ascii_whitespace()
            .map(|s| s.parse().unwrap())
            .tuples()
            .next()
            .unwrap();
        let expected = collect4(
            scanf!(after, "After:  [{}]", str)
                .unwrap()
                .split(", ")
                .map(|s| s.parse().unwrap()),
        );
        let m = Machine(input);
        let mut choices = HashSet::new();
        for icmd in 0..16 {
            let cmd = Opcode::from_usize(icmd).unwrap();
            if let Some(res) = m.step(cmd, a, b, c) {
                if res.0 == expected {
                    choices.insert(icmd);
                }
            }
        }
        possibles[orig_cmd] = &possibles[orig_cmd] & &choices;
    }
    let mut opmap = HashMap::new();
    while opmap.len() < possibles.len() {
        for i in 0..16 {
            if opmap.contains_key(&i) {
                continue;
            }
            if possibles[i].len() == 1 {
                let to = *possibles[i].iter().next().unwrap();
                opmap.insert(i, to);
                for (j, poss) in possibles.iter_mut().enumerate() {
                    if i != j {
                        poss.remove(&to);
                    }
                }
            }
        }
    }
    let mut m = Machine([0, 0, 0, 0]);
    for line in prog.lines() {
        let (orig_cmd, a, b, c) = line
            .split_ascii_whitespace()
            .map(|s| s.parse().unwrap())
            .tuples()
            .next()
            .unwrap();
        m = m
            .step(Opcode::from_usize(opmap[&orig_cmd]).unwrap(), a, b, c)
            .unwrap();
    }
    m.0[0]
}

xaoc::xaoc!(no_sample = true);
