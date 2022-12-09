use itertools::Itertools;
use sscanf::scanf;
use std::collections::HashMap;

enum Cmd {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

impl Cmd {
    fn parse(inp: &str) -> Vec<Self> {
        let mut ret = vec![];
        for cmd in inp.split(',') {
            let cmd = if let Ok(x) = scanf!(cmd, "s{}", usize) {
                Cmd::Spin(x)
            } else if let Ok((a, b)) = scanf!(cmd, "x{}/{}", usize, usize) {
                Cmd::Exchange(a, b)
            } else if let Ok((a, b)) = scanf!(cmd, "p{}/{}", char, char) {
                Cmd::Partner(a, b)
            } else {
                unreachable!();
            };
            ret.push(cmd);
        }
        ret
    }
}

struct Progs {
    progs: Vec<char>,
}

impl Progs {
    fn step(&mut self, cmds: &[Cmd]) {
        for cmd in cmds {
            match *cmd {
                Cmd::Spin(x) => self.progs.rotate_right(x),
                Cmd::Exchange(a, b) => self.progs.swap(a, b),
                Cmd::Partner(a, b) => {
                    let i1 = self.progs.iter().position(|&c| c == a).unwrap();
                    let i2 = self.progs.iter().position(|&c| c == b).unwrap();
                    self.progs.swap(i1, i2);
                }
            }
        }
    }

    fn string(&self) -> String {
        self.progs.iter().collect()
    }
}

impl Default for Progs {
    fn default() -> Self {
        Self {
            progs: ('a'..='p').collect_vec(),
        }
    }
}

fn part1(inp: &str) -> String {
    let cmds = Cmd::parse(inp);
    let mut progs = Progs::default();
    progs.step(&cmds);
    progs.string()
}

fn part2(inp: &str) -> String {
    let cmds = Cmd::parse(inp);
    let mut progs = Progs::default();
    let mut cache = HashMap::new();
    let mut base;
    let period;
    let mut idx = 0;
    loop {
        let s = progs.string();
        if let Some(&prev_idx) = cache.get(&s) {
            base = prev_idx;
            period = idx - prev_idx;
            break;
        }
        cache.insert(s, idx);
        progs.step(&cmds);
        idx += 1;
    }
    let target = 1000000000;
    base += (target - base) / period * period;
    for _ in base..target {
        progs.step(&cmds);
    }
    progs.string()
}

xaoc::xaoc!(no_sample = true);
