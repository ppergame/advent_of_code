use rayon::prelude::*;
use std::collections::{HashSet, VecDeque};
use xaoc::md5;

struct Item {
    i: usize,
    three: Option<u32>,
    fives: HashSet<u32>,
}

struct Reps {
    salt: String,
    stretch: usize,
    next: usize,
    fives: Vec<VecDeque<usize>>,
    threes: VecDeque<(usize, u32)>,
}

impl Reps {
    fn fill(&mut self) {
        let after = self.next + 1024;
        let mut items = vec![];
        (self.next..after)
            .into_par_iter()
            .map(|i| {
                let mut digest = md5(&format!("{}{}", self.salt, i));
                let mut hex: [u8; 32] = [0; 32];
                for _ in 0..self.stretch {
                    for (i, b) in digest.as_slice().iter().enumerate() {
                        hex[i * 2] = char::from_digit((b >> 4) as u32, 16).unwrap() as u8;
                        hex[i * 2 + 1] = char::from_digit((b & 0xF) as u32, 16).unwrap() as u8;
                    }
                    digest = md5(hex);
                }
                let s = format!("{:x}", digest);
                let three = s.as_bytes().windows(3).find_map(|v| {
                    if v[0] == v[1] && v[1] == v[2] {
                        Some((v[0] as char).to_digit(16).unwrap())
                    } else {
                        None
                    }
                });
                let fives = s
                    .as_bytes()
                    .windows(5)
                    .filter_map(|v| {
                        if v.windows(2).all(|w| w[0] == w[1]) {
                            Some((v[0] as char).to_digit(16).unwrap())
                        } else {
                            None
                        }
                    })
                    .collect();
                Item { i, three, fives }
            })
            .collect_into_vec(&mut items);
        self.next = after;
        for item in items {
            if let Some(three) = item.three {
                self.threes.push_back((item.i, three));
            }
            for five in item.fives {
                self.fives[five as usize].push_back(item.i);
            }
        }
    }

    fn next_key(&mut self) -> usize {
        loop {
            let Some(&(three, digit)) = self.threes.front() else { self.fill(); continue; };
            let fives = &mut self.fives[digit as usize];
            while fives.front().map_or(false, |&five| five <= three) {
                fives.pop_front();
            }
            if fives.back().map_or(true, |&five| five <= three + 1000) {
                self.fill();
                continue;
            }
            self.threes.pop_front();
            if *fives.front().unwrap() <= three + 1000 {
                return three;
            }
        }
    }
}

fn part1(inp: &str) -> usize {
    let mut reps = Reps {
        salt: inp.to_string(),
        stretch: 0,
        next: 0,
        fives: vec![Default::default(); 16],
        threes: Default::default(),
    };
    for _ in 0..63 {
        reps.next_key();
    }
    reps.next_key()
}

fn part2(inp: &str) -> usize {
    let mut reps = Reps {
        salt: inp.to_string(),
        stretch: 2016,
        next: 0,
        fives: vec![Default::default(); 16],
        threes: Default::default(),
    };
    for _ in 0..63 {
        reps.next_key();
    }
    reps.next_key()
}

xaoc::xaoc!();
