use std::collections::HashMap;

#[derive(Debug)]
enum Mode {
    Reg,
    Rep1,
    Rep2(usize),
    RepAcc(usize, usize),
}

fn part1(inp: &str) -> usize {
    let mut ret = String::new();
    let mut mode = Mode::Reg;
    let mut acc = String::new();
    for c in inp.chars() {
        match mode {
            Mode::Reg => match c {
                '(' => mode = Mode::Rep1,
                _ => ret.push(c),
            },
            Mode::Rep1 => match c {
                'x' => mode = Mode::Rep2(std::mem::take(&mut acc).parse().unwrap()),
                _ => acc.push(c),
            },
            Mode::Rep2(num_chars) => match c {
                ')' => mode = Mode::RepAcc(num_chars, std::mem::take(&mut acc).parse().unwrap()),
                _ => acc.push(c),
            },
            Mode::RepAcc(1, times) => {
                acc.push(c);
                for _ in 0..times {
                    ret.push_str(&acc);
                }
                acc.clear();
                mode = Mode::Reg;
            }
            Mode::RepAcc(num_chars, times) => {
                acc.push(c);
                mode = Mode::RepAcc(num_chars - 1, times);
            }
        }
    }
    ret.len()
}

#[derive(Default)]
struct Decompressor {
    map: HashMap<String, usize>,
}

impl Decompressor {
    fn decompress(&mut self, s: &str) -> usize {
        if let Some(&len) = self.map.get(s) {
            return len;
        }
        let mut count = 0;
        let mut mode = Mode::Reg;
        let mut acc = String::new();
        for c in s.chars() {
            match mode {
                Mode::Reg => match c {
                    '(' => mode = Mode::Rep1,
                    _ => count += 1,
                },
                Mode::Rep1 => match c {
                    'x' => mode = Mode::Rep2(std::mem::take(&mut acc).parse().unwrap()),
                    _ => acc.push(c),
                },
                Mode::Rep2(num_chars) => match c {
                    ')' => {
                        mode = Mode::RepAcc(num_chars, std::mem::take(&mut acc).parse().unwrap())
                    }
                    _ => acc.push(c),
                },
                Mode::RepAcc(1, times) => {
                    acc.push(c);
                    let len = self.decompress(&acc);
                    count += times * len;
                    acc.clear();
                    mode = Mode::Reg;
                }
                Mode::RepAcc(num_chars, times) => {
                    acc.push(c);
                    mode = Mode::RepAcc(num_chars - 1, times);
                }
            }
        }
        self.map.insert(s.to_string(), count);
        count
    }
}

fn part2(inp: &str) -> usize {
    Decompressor::default().decompress(inp)
}

xaoc::xaoc!();
