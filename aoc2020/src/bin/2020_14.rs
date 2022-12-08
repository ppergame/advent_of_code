use bitvec::{order::Lsb0, vec::BitVec};
use itertools::Itertools as _;
use std::collections::HashMap;

#[derive(Debug)]
struct Instr {
    mask_x: u64,
    mask_1: u64,
    mem: Vec<(u64, u64)>,
}

fn parse_mask<const C: char>(m: &str) -> u64 {
    m.chars()
        .rev()
        .fold((1, 0), |(shift, num), c| {
            let mut new_num = num;
            if c == C {
                new_num |= shift;
            }
            (shift << 1, new_num)
        })
        .1
}

impl Instr {
    fn new(m: &str) -> Instr {
        Instr {
            mask_x: parse_mask::<'X'>(m),
            mask_1: parse_mask::<'1'>(m),
            mem: Vec::new(),
        }
    }
}

fn parse(inp: &str) -> Vec<Instr> {
    let mut ret = Vec::new();
    let mut maybe_instr = None;
    for line in inp.lines() {
        let (k, v) = line.split_once(" = ").unwrap();
        if k == "mask" {
            if let Some(instr) = maybe_instr.replace(Instr::new(v)) {
                ret.push(instr);
            }
            continue;
        }
        let k = k.strip_prefix("mem[").unwrap();
        let k = k.strip_suffix(']').unwrap();
        maybe_instr
            .as_mut()
            .unwrap()
            .mem
            .push((k.parse().unwrap(), v.parse().unwrap()));
    }
    if let Some(instr) = maybe_instr {
        ret.push(instr);
    }
    ret
}

fn part1(inp: &str) -> u64 {
    let instrs = parse(inp);
    let mut vals = HashMap::<u64, u64>::new();
    for instr in instrs {
        for (k, v) in instr.mem {
            vals.insert(k, (v & instr.mask_x) | instr.mask_1);
        }
    }
    vals.values().cloned().sum()
}

fn part2(inp: &str) -> u64 {
    let instrs = parse(inp);
    let mut vals = HashMap::<u64, u64>::new();
    for instr in instrs {
        let bv = BitVec::<u64, Lsb0>::from_element(instr.mask_x);
        let true_bits: Vec<_> = bv
            .iter()
            .enumerate()
            .filter_map(|(i, bit)| if *bit { Some(i) } else { None })
            .collect();
        let mut x_masks = Vec::new();
        for size in 0..=true_bits.len() {
            for indexes in true_bits.iter().combinations(size) {
                let mut num = 0;
                for i in indexes {
                    num |= 1 << i;
                }
                x_masks.push(num);
            }
        }
        for (k, v) in instr.mem {
            let k = k | instr.mask_1;
            let k = k & !instr.mask_x;
            for mask in &x_masks {
                vals.insert(k | mask, v);
            }
        }
    }
    vals.values().cloned().sum()
}

xaoc::xaoc!();
