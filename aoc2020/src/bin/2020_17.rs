use std::{collections::HashSet, hash::Hash};

trait Coord: Sized + Eq + Hash + Copy {
    fn new(x: i32, y: i32) -> Self;
    fn neigh(self) -> Vec<Self>;
}

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
struct Coord3(i32, i32, i32);

impl Coord for Coord3 {
    fn new(x: i32, y: i32) -> Coord3 {
        Coord3(x, y, 0)
    }

    fn neigh(self) -> Vec<Coord3> {
        let mut ret = Vec::new();
        let Coord3(x, y, z) = self;
        for nx in [x - 1, x, x + 1] {
            for ny in [y - 1, y, y + 1] {
                for nz in [z - 1, z, z + 1] {
                    if (x, y, z) == (nx, ny, nz) {
                        continue;
                    }
                    ret.push(Coord3(nx, ny, nz));
                }
            }
        }
        ret
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
struct Coord4(i32, i32, i32, i32);

impl Coord for Coord4 {
    fn new(x: i32, y: i32) -> Coord4 {
        Coord4(x, y, 0, 0)
    }

    fn neigh(self) -> Vec<Coord4> {
        let mut ret = Vec::new();
        let Coord4(x, y, z, w) = self;
        for nx in [x - 1, x, x + 1] {
            for ny in [y - 1, y, y + 1] {
                for nz in [z - 1, z, z + 1] {
                    for nw in [w - 1, w, w + 1] {
                        if (x, y, z, w) == (nx, ny, nz, nw) {
                            continue;
                        }
                        ret.push(Coord4(nx, ny, nz, nw));
                    }
                }
            }
        }
        ret
    }
}

fn parse<T: Coord>(inp: &str) -> HashSet<T> {
    let mut map = HashSet::new();
    for (row, line) in inp.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == '#' {
                map.insert(T::new(col as i32, row as i32));
            }
        }
    }
    map
}

fn rules(c: bool, count: usize) -> bool {
    if c {
        count == 2 || count == 3
    } else {
        count == 3
    }
}

fn cycle<T: Coord>(map: &HashSet<T>) -> HashSet<T> {
    let mut newmap = HashSet::new();
    for &c in map {
        let mut count = 0;
        for nc in c.neigh() {
            if map.contains(&nc) {
                count += 1;
            }
            let ncount = nc.neigh().iter().filter(|nnc| map.contains(nnc)).count();
            if rules(map.contains(&nc), ncount) {
                newmap.insert(nc);
            }
        }
        if rules(map.contains(&c), count) {
            newmap.insert(c);
        }
    }
    newmap
}

fn part1(inp: &str) -> usize {
    let mut map = parse::<Coord3>(inp);
    for _ in 0..6 {
        map = cycle(&map);
    }
    map.len()
}

fn part2(inp: &str) -> usize {
    let mut map = parse::<Coord4>(inp);
    for _ in 0..6 {
        map = cycle(&map);
    }
    map.len()
}

xaoc::xaoc!();
