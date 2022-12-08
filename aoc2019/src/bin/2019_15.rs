use num_derive::{FromPrimitive, ToPrimitive};
use std::collections::HashMap;
use std::collections::HashSet;

use aoc2019::intcode::*;

type Coord = (i64, i64);

trait CoordMethods {
    fn neigh(&self) -> Vec<(Dir, Coord)>;
}

impl CoordMethods for Coord {
    fn neigh(&self) -> Vec<(Dir, Coord)> {
        let (x, y) = *self;
        vec![
            (Dir::N, (x, y - 1)),
            (Dir::E, (x + 1, y)),
            (Dir::S, (x, y + 1)),
            (Dir::W, (x - 1, y)),
        ]
    }
}

struct Map {
    // true -> corridor, false -> wall
    map: HashMap<Coord, bool>,
    oxy: Option<Coord>,
}

#[derive(Clone, Copy, FromPrimitive, ToPrimitive)]
enum Dir {
    N = 1,
    S = 2,
    W = 3,
    E = 4,
}

impl Dir {
    fn opp(&self) -> Dir {
        match self {
            Dir::N => Dir::S,
            Dir::S => Dir::N,
            Dir::W => Dir::E,
            Dir::E => Dir::W,
        }
    }
}

#[derive(FromPrimitive, ToPrimitive)]
enum Status {
    Wall = 0,
    Corr = 1,
    Oxy = 2,
}

impl Map {
    fn new() -> Map {
        let mut map = HashMap::new();
        map.insert((0, 0), true);
        Map { map, oxy: None }
    }

    fn explore(&mut self, ic: &mut Intcode) {
        let mut pos = (0, 0);
        let mut stack = Vec::<(Dir, Coord)>::new();
        loop {
            let neigh = pos
                .neigh()
                .into_iter()
                .find(|(_, nc)| !self.map.contains_key(nc));
            match neigh {
                None => match stack.pop() {
                    None => break,
                    Some((dir, c)) => {
                        match ic.rmove(dir.opp()) {
                            Status::Corr | Status::Oxy => (),
                            Status::Wall => panic!("ran into wall while backtracking"),
                        }
                        pos = c;
                        continue;
                    }
                },
                Some((dir, c)) => match ic.rmove(dir) {
                    Status::Wall => {
                        self.map.insert(c, false);
                    }
                    status @ (Status::Corr | Status::Oxy) => {
                        self.map.insert(c, true);
                        stack.push((dir, pos));
                        pos = c;
                        if let Status::Oxy = status {
                            self.oxy = Some(pos);
                        }
                    }
                },
            }
        }
    }

    #[allow(dead_code)]
    fn draw(&self) {
        let minx = self.map.keys().min_by_key(|(x, _)| x).unwrap().0;
        let maxx = self.map.keys().max_by_key(|(x, _)| x).unwrap().0;
        let miny = self.map.keys().min_by_key(|(_, y)| y).unwrap().1;
        let maxy = self.map.keys().max_by_key(|(_, y)| y).unwrap().1;
        for row in miny..=maxy {
            for col in minx..=maxx {
                print!(
                    "{}",
                    if (row, col) != (0, 0) {
                        match self.map.get(&(col, row)) {
                            None => "-",
                            Some(v) => match v {
                                true => " ",
                                false => "#",
                            },
                        }
                    } else {
                        "0"
                    }
                );
            }
            println!();
        }
    }

    fn path_len(&self, from: Coord, to: Coord) -> i64 {
        let mut q = HashSet::<Coord>::new();
        let mut dist = HashMap::<Coord, i64>::new();
        let mut prev = HashMap::<Coord, Option<Coord>>::new();
        for (c, _) in self.map.iter().filter(|&(_, &v)| v) {
            q.insert(*c);
            dist.insert(*c, i64::MAX / 2);
            prev.insert(*c, None);
        }
        dist.insert(from, 0);
        while !q.is_empty() {
            let u = *q.iter().min_by_key(|c| dist[c]).unwrap();
            q.remove(&u);
            let neigh = u
                .neigh()
                .into_iter()
                .filter(|(_, nc)| *self.map.get(nc).unwrap_or(&false));
            for (_, v) in neigh {
                let alt = dist[&u] + 1;
                if alt < dist[&v] {
                    dist.insert(v, alt);
                    prev.insert(v, Some(u));
                }
            }
        }
        let mut cur = to;
        let mut ret = 0;
        while cur != from {
            ret += 1;
            cur = prev[&cur].unwrap();
        }
        ret
    }

    fn fill_time(&self) -> i64 {
        let mut done = HashSet::<Coord>::new();
        done.insert(self.oxy.unwrap());
        let mut next = done.clone();
        let mut ret = 0;
        while !next.is_empty() {
            let mut nextnext = HashSet::<Coord>::new();
            for c in &next {
                for (_, nc) in c.neigh().into_iter().filter(|(_, nc)| {
                    *self.map.get(nc).unwrap_or(&false) && !done.contains(nc) && !next.contains(nc)
                }) {
                    nextnext.insert(nc);
                }
            }
            done.extend(next);
            next = nextnext;
            ret += 1
        }
        ret - 1
    }
}

trait RMove {
    fn rmove(&mut self, dir: Dir) -> Status;
}

impl RMove for Intcode {
    fn rmove(&mut self, dir: Dir) -> Status {
        assert!(matches!(self.run().unwrap(), IntcodeStatus::Input));
        self.input = Some(dir as i64);
        num_traits::FromPrimitive::from_i64(match self.run().unwrap() {
            IntcodeStatus::Output(output) => output,
            _ => panic!("bad status"),
        })
        .unwrap()
    }
}

fn part1(inp: &str) -> i64 {
    let mut ic = Intcode::new(inp);
    let mut map = Map::new();
    map.explore(&mut ic);
    //map.draw();
    //println!("{} {:?}", map.map.len(), map.oxy);
    map.path_len((0, 0), map.oxy.unwrap())
}

fn part2(inp: &str) -> i64 {
    let mut ic = Intcode::new(inp);
    let mut map = Map::new();
    map.explore(&mut ic);
    map.fill_time()
}

xaoc::xaoc!();
