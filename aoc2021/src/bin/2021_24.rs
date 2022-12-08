use itertools::Itertools;
use pathfinding::directed::dijkstra::dijkstra;
use std::collections::HashSet;

#[derive(Debug, Clone)]
struct Prog {
    div3: i64,
    add4: i64,
    add14: i64,
}

impl Prog {
    fn run(&self, w: i64, mut z: i64) -> i64 {
        // -1: inp w
        let mut x = 0; // 0: mul x 0
        x += z; // 1: add x z
        x %= 26; // 2: mod x 26
        z /= self.div3; // 3: div z 1
        x += self.add4; // 4: add x 10
        x = (x == w) as i64; // 5: eql x w
        x = (x == 0) as i64; // 6: eql x 0
        let mut y = 0; // 7: mul y 0
        y += 25; // 8: add y 25
        y *= x; // 9: mul y x
        y += 1; // 10: add y 1
        z *= y; // 11: mul z y
        y = 0; // 12: mul y 0
        y += w; // 13: add y w
        y += self.add14; // 14: add y 1
        y *= x; // 15: mul y x
        z += y; // 16: add z y
        z
    }
}

#[derive(Debug, Clone)]
pub struct Input {
    progs: Vec<Prog>,
}

fn parse(inp: &str) -> Input {
    let mut progs = vec![];
    for s in inp.split("inp w\n").skip(1) {
        let mut div3 = None;
        let mut add4 = None;
        let mut add14 = None;
        for (idx, line) in s.lines().enumerate() {
            if idx == 3 {
                let (rest, par) = line.rsplit_once(' ').unwrap();
                assert_eq!(rest, "div z");
                div3 = Some(par.parse().unwrap());
            } else if idx == 4 {
                let (rest, par) = line.rsplit_once(' ').unwrap();
                assert_eq!(rest, "add x");
                add4 = Some(par.parse().unwrap());
            } else if idx == 14 {
                let (rest, par) = line.rsplit_once(' ').unwrap();
                assert_eq!(rest, "add y");
                add14 = Some(par.parse().unwrap());
            }
        }
        progs.push(Prog {
            div3: div3.unwrap(),
            add4: add4.unwrap(),
            add14: add14.unwrap(),
        });
    }
    Input { progs }
}

// given prog, takes required values of z, returns possible (w, z) inputs to prog
fn reverse_values<const CHECK: bool>(prog: &Prog, z_val: i64) -> Vec<(i64, i64)> {
    let mut ret = HashSet::new();
    for w in 1..=9 {
        let magic_rem = w - prog.add4;
        let magic_rem = if (0..26).contains(&magic_rem) {
            Some(magic_rem)
        } else {
            None
        };
        if prog.div3 == 26 {
            for rem in 0..=25 {
                if Some(rem) == magic_rem {
                    ret.insert((w, z_val * 26 + rem));
                    continue;
                }
                if z_val % 26 == w + prog.add14 {
                    ret.insert((w, z_val / 26 * 26 + rem));
                }
            }
        } else if Some(z_val % 26) == magic_rem {
            ret.insert((w, z_val));
        } else if z_val % 26 == w + prog.add14 {
            ret.insert((w, z_val / 26));
        }
    }
    if CHECK {
        for (w, z) in &ret {
            assert_eq!(z_val, prog.run(*w, *z));
        }
    }
    ret.into_iter().sorted().collect::<Vec<_>>()
}

fn part1(inp: &str) -> String {
    let inp = parse(inp);
    let (_, cost) = dijkstra(
        &(14, 0),
        |(round, z)| {
            if *round == 0 {
                return vec![];
            }
            reverse_values::<true>(&inp.progs[round - 1], *z)
                .into_iter()
                .map(|(w, z)| ((round - 1, z), -(10_i64.pow((14 - round) as u32)) * w))
                .collect()
        },
        |goal| goal == &(0, 0),
    )
    .unwrap();
    (-cost).to_string()
}

fn part2(inp: &str) -> String {
    let inp = parse(inp);
    let (_, cost) = dijkstra(
        &(14, 0),
        |(round, z)| {
            if *round == 0 {
                return vec![];
            }
            reverse_values::<true>(&inp.progs[round - 1], *z)
                .into_iter()
                .map(|(w, z)| ((round - 1, z), 10_i64.pow((14 - round) as u32) * w))
                .collect()
        },
        |goal| goal == &(0, 0),
    )
    .unwrap();
    cost.to_string()
}

xaoc::xaoc!();
