use itertools::Itertools;
use pathfinding::prelude::*;
use rayon::prelude::*;
use std::collections::{BTreeSet, HashMap, HashSet};

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Coord(i32, i32);

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct State {
    collected: BTreeSet<char>,
    needed: BTreeSet<char>,
    pos: Coord,
}

impl State {
    fn new(map: &Map) -> Self {
        Self {
            collected: BTreeSet::new(),
            needed: map.digits.keys().filter(|&&c| c != '0').copied().collect(),
            pos: map.digits[&'0'],
        }
    }

    fn succ(&self, map: &Map) -> Vec<(Self, i32)> {
        if self.needed.is_empty() {
            let zpos = map.digits[&'0'];
            if self.pos == zpos {
                return vec![];
            }
            let mut nst = self.clone();
            nst.pos = zpos;
            let cost = map.path(self.pos, nst.pos);
            return vec![(nst, cost)];
        }

        self.needed
            .iter()
            .map(|&c| {
                let mut nst = self.clone();
                nst.needed.remove(&c);
                nst.collected.insert(c);
                nst.pos = map.digits[&c];
                let cost = map.path(self.pos, nst.pos);
                (nst, cost)
            })
            .collect()
    }
}

struct Map {
    map: HashSet<Coord>,
    digits: HashMap<char, Coord>,
    digit_dists: HashMap<(Coord, Coord), i32>,
}

impl Map {
    fn parse(inp: &str) -> Self {
        let mut map = HashSet::new();
        let mut digits = HashMap::new();
        for (y, line) in inp.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let x = x as i32;
                let y = y as i32;
                match c {
                    '#' => (),
                    '.' => {
                        map.insert(Coord(x, y));
                    }
                    d @ '0'..='9' => {
                        map.insert(Coord(x, y));
                        digits.insert(d, Coord(x, y));
                    }
                    _ => unreachable!(),
                }
            }
        }
        let mut map = Self {
            map,
            digits,
            digit_dists: HashMap::new(),
        };
        map.update_dists();
        map
    }

    fn update_dists(&mut self) {
        let digits = self.digits.values().copied().collect_vec();
        self.digit_dists.extend(
            digits
                .par_iter()
                .flat_map_iter(|digit| {
                    let mut remaining = HashSet::<&Coord>::from_iter(self.digits.values());
                    let paths = dijkstra_partial(
                        digit,
                        |coord| self.adj(*coord),
                        |coord| {
                            remaining.remove(coord);
                            remaining.is_empty()
                        },
                    )
                    .0;
                    digits.iter().filter_map(move |digit2| {
                        if digit == digit2 {
                            None
                        } else {
                            Some(((*digit, *digit2), paths[digit2].1))
                        }
                    })
                })
                .collect::<Vec<_>>(),
        );
    }

    fn adj(&self, c: Coord) -> Vec<(Coord, i32)> {
        let Coord(x, y) = c;
        [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
            .iter()
            .filter_map(|&(x, y)| {
                let nc = Coord(x, y);
                if self.map.contains(&nc) {
                    Some((nc, 1))
                } else {
                    None
                }
            })
            .collect()
    }

    fn path(&self, c1: Coord, c2: Coord) -> i32 {
        self.digit_dists[&(c1, c2)]
    }
}

fn part1(inp: &str) -> i32 {
    let map = Map::parse(inp);
    let state = State::new(&map);
    pathfinding::directed::dijkstra::dijkstra(
        &state,
        |state| state.succ(&map),
        |state| state.needed.is_empty(),
    )
    .unwrap()
    .1
}

fn part2(inp: &str) -> i32 {
    let map = Map::parse(inp);
    let state = State::new(&map);
    pathfinding::directed::dijkstra::dijkstra(
        &state,
        |state| state.succ(&map),
        |state| state.needed.is_empty() && state.pos == map.digits[&'0'],
    )
    .unwrap()
    .1
}

xaoc::xaoc!(sample_idx = 3);
