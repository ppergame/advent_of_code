use regex::Regex;
use std::collections::HashMap;
use vecmat::{prelude::*, Vector};

type Vec3 = Vector<i64, 3>;

#[derive(Clone, Copy)]
struct Moon {
    pos: Vec3,
    vel: Vec3,
}

struct System {
    moons: Vec<Moon>,
}

fn cmp_to_dv(c: std::cmp::Ordering) -> i64 {
    match c {
        std::cmp::Ordering::Less => 1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => -1,
    }
}

impl System {
    fn step(&mut self) {
        for i in 0..self.moons.len() {
            for j in 0..self.moons.len() {
                if i == j {
                    continue;
                }
                for c in 0..=2 {
                    self.moons[i].vel[c] +=
                        cmp_to_dv(self.moons[i].pos[c].cmp(&self.moons[j].pos[c]));
                }
            }
        }
        for m in self.moons.iter_mut() {
            for c in 0..=2 {
                m.pos[c] += m.vel[c];
            }
        }
    }

    fn energy(&self) -> i64 {
        let mut ret = 0;
        for m in &self.moons {
            ret += m.pos.map(i64::abs).sum() * m.vel.map(i64::abs).sum();
        }
        ret
    }

    fn state(&self, c: usize) -> Vec<i64> {
        let mut ret = Vec::new();
        for m in &self.moons {
            ret.push(m.pos[c]);
            ret.push(m.vel[c]);
        }
        ret
    }
}

lazy_static::lazy_static! {
    static ref MOON_RE: Regex = Regex::new(r"<x=(-?\d+), y=(-?\d+), z=(-?\d+)>").unwrap();
}

fn parse(s: &str) -> System {
    let mut ret = System { moons: Vec::new() };
    for cap in MOON_RE.captures_iter(s) {
        let m = Moon {
            pos: Vec3::from([
                cap[1].parse().unwrap(),
                cap[2].parse().unwrap(),
                cap[3].parse().unwrap(),
            ]),
            vel: Vec3::zero(),
        };
        ret.moons.push(m);
    }
    ret
}

fn part1(inp: &str) -> i64 {
    let mut s = parse(inp);
    for _ in 0..1000 {
        s.step();
    }
    s.energy()
}

fn part2(inp: &str) -> i64 {
    let mut s = parse(inp);
    let mut caches: Vec<HashMap<Vec<i64>, Vec<usize>>> = Vec::new();
    let initial = (0..=2).map(|c| s.state(c)).collect::<Vec<Vec<i64>>>();
    let mut hit: Vec<i64> = vec![0, 0, 0];
    for _ in 0..=2 {
        caches.push(HashMap::new());
    }
    'outer: for step in 1..1000000 {
        s.step();
        for c in 0..=2 {
            let key = s.state(c);
            if initial[c] == key && hit[c] == 0 {
                hit[c] = step;
                if hit.iter().all(|h| *h > 0) {
                    break 'outer;
                }
            }
        }
    }
    num::integer::lcm(num::integer::lcm(hit[2], hit[1]), hit[0])
}

xaoc::xaoc!();
