use sscanf::scanf;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;

fn part1(inp: &str) -> usize {
    inp.lines()
        .map(|line| {
            scanf!(
                line,
                "p=<{},{},{}>, v=<{},{},{}>, a=<{},{},{}>",
                i64,
                i64,
                i64,
                i64,
                i64,
                i64,
                i64,
                i64,
                i64
            )
            .unwrap()
        })
        .enumerate()
        .min_by_key(|(_, (x, y, z, vx, vy, vz, ax, ay, az))| {
            (
                ax.abs() + ay.abs() + az.abs(),
                vx.abs(),
                vy.abs(),
                vz.abs(),
                x.abs() + y.abs() + z.abs(),
            )
        })
        .unwrap()
        .0
}

struct Dim {
    p: i64,
    v: i64,
    a: i64,
}

impl Dim {
    fn step(&mut self) {
        self.v += self.a;
        self.p += self.v;
    }
}

struct Particle {
    x: Dim,
    y: Dim,
    z: Dim,
}

impl Debug for Particle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "p=<{},{},{}>, v=<{},{},{}>, a=<{},{},{}>",
            self.x.p,
            self.y.p,
            self.z.p,
            self.x.v,
            self.y.v,
            self.z.v,
            self.x.a,
            self.y.a,
            self.z.a,
        )
    }
}

impl Particle {
    fn parse(line: &str) -> Self {
        let (x, y, z, vx, vy, vz, ax, ay, az) = scanf!(
            line,
            "p=<{},{},{}>, v=<{},{},{}>, a=<{},{},{}>",
            i64,
            i64,
            i64,
            i64,
            i64,
            i64,
            i64,
            i64,
            i64
        )
        .unwrap();
        Self {
            x: Dim { p: x, v: vx, a: ax },
            y: Dim { p: y, v: vy, a: ay },
            z: Dim { p: z, v: vz, a: az },
        }
    }

    fn step(&mut self) {
        self.x.step();
        self.y.step();
        self.z.step();
    }
}

fn part2(inp: &str) -> usize {
    let mut pp = inp
        .lines()
        .map(|line| {
            let p = Particle::parse(line);
            ((p.x.p, p.y.p, p.z.p), p)
        })
        .collect::<HashMap<_, _>>();
    for _ in 0..1000 {
        let mut next = HashMap::new();
        let mut to_remove = HashSet::new();
        for mut p in pp.into_values() {
            p.step();
            let key = (p.x.p, p.y.p, p.z.p);
            match next.entry(key) {
                std::collections::hash_map::Entry::Occupied(_) => {
                    to_remove.insert(key);
                }
                std::collections::hash_map::Entry::Vacant(v) => {
                    v.insert(p);
                }
            }
        }
        for key in to_remove {
            next.remove(&key);
        }
        pp = next;
    }
    pp.len()
}

xaoc::xaoc!(no_sample = true);
