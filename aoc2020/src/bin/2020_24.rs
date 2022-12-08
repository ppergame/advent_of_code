use std::collections::HashSet;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

type Coord = (i64, i64, i64);

#[derive(Debug, EnumIter)]
enum Dir {
    E,
    Se,
    Sw,
    W,
    Nw,
    Ne,
}

impl Dir {
    fn delta(self) -> Coord {
        match self {
            Dir::E => (1, -1, 0),
            Dir::Se => (0, -1, 1),
            Dir::Sw => (-1, 0, 1),
            Dir::W => (-1, 1, 0),
            Dir::Nw => (0, 1, -1),
            Dir::Ne => (1, 0, -1),
        }
    }
}

fn neigh(c: Coord) -> Vec<Coord> {
    let (x, y, z) = c;
    Dir::iter()
        .map(|d| {
            let (dx, dy, dz) = d.delta();
            (x + dx, dy + y, dz + z)
        })
        .collect()
}

fn parse(inp: &str) -> Vec<Vec<Dir>> {
    let mut ret = Vec::new();
    for line in inp.lines() {
        let mut td = Vec::new();
        let mut prev = ' ';
        for c in line.chars() {
            let d = match c {
                'e' => {
                    if prev == 's' {
                        prev = ' ';
                        Dir::Se
                    } else if prev == 'n' {
                        prev = ' ';
                        Dir::Ne
                    } else {
                        Dir::E
                    }
                }
                'w' => {
                    if prev == 's' {
                        prev = ' ';
                        Dir::Sw
                    } else if prev == 'n' {
                        prev = ' ';
                        Dir::Nw
                    } else {
                        Dir::W
                    }
                }
                'n' => {
                    prev = 'n';
                    continue;
                }
                's' => {
                    prev = 's';
                    continue;
                }
                _ => panic!(),
            };
            td.push(d);
        }
        assert_eq!(prev, ' ');
        ret.push(td);
    }
    ret
}

fn part1(inp: &str) -> usize {
    let vvd = parse(inp);
    let mut map = HashSet::<Coord>::new();
    for vd in vvd {
        let mut c = (0, 0, 0);
        for d in vd {
            let (x, y, z) = c;
            let (dx, dy, dz) = d.delta();
            c = (x + dx, y + dy, z + dz);
        }
        if map.contains(&c) {
            map.remove(&c);
        } else {
            map.insert(c);
        }
    }
    map.len()
}

fn part2(inp: &str) -> usize {
    let vvd = parse(inp);
    let mut map = HashSet::<Coord>::new();
    for vd in vvd {
        let mut c = (0, 0, 0);
        for d in vd {
            let (x, y, z) = c;
            let (dx, dy, dz) = d.delta();
            c = (x + dx, y + dy, z + dz);
        }
        if map.contains(&c) {
            map.remove(&c);
        } else {
            map.insert(c);
        }
    }

    for _ in 0..100 {
        let mut to_check = HashSet::<Coord>::new();
        let mut newmap = HashSet::<Coord>::new();
        let mut check = |c| {
            let count = neigh(c).iter().filter(|x| map.contains(x)).count();
            if map.contains(&c) {
                if count == 1 || count == 2 {
                    newmap.insert(c);
                }
            } else if count == 2 {
                newmap.insert(c);
            }
        };
        for &c in &map {
            for nc in neigh(c) {
                to_check.insert(nc);
            }
            check(c);
        }
        for c in to_check {
            check(c);
        }
        map = newmap;
    }
    map.len()
}

xaoc::xaoc!();
