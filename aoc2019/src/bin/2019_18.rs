use bimap::BiHashMap;
use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::collections::{HashMap, HashSet, VecDeque};
use std::convert::TryInto;

type Coord = (i8, i8);

trait CoordMethods {
    fn neigh(&self) -> Vec<Coord>;
}

impl CoordMethods for Coord {
    fn neigh(&self) -> Vec<Coord> {
        let (x, y) = *self;

        vec![(x, y - 1), (x + 1, y), (x, y + 1), (x - 1, y)]
    }
}

type BM = u32;

fn mask(b: char) -> u32 {
    1 << (b as u8 - b'a')
}

trait BMMethods {
    fn set(&self, b: char) -> BM;

    fn unset(&self, b: char) -> BM;

    fn get(&self, b: char) -> bool;
}

impl BMMethods for BM {
    fn set(&self, b: char) -> BM {
        *self | mask(b)
    }

    fn unset(&self, b: char) -> BM {
        *self & !mask(b)
    }

    fn get(&self, b: char) -> bool {
        (*self & mask(b)) != 0
    }
}

type Key = char;
type Cost = u32;

struct Map<const N: usize> {
    // only contains passable cells
    map: HashSet<Coord>,
    graph: HashMap<Coord, Vec<(Coord, Cost)>>,
    keys: BiHashMap<Coord, Key>,
    doors: BiHashMap<Coord, Key>,
    initial_pos: [Coord; N],
}

impl<const N: usize> Map<N> {
    fn adjust_map_and_make_initial_pos(map: &mut HashSet<Coord>, at: Coord) -> [Coord; N] {
        match N {
            1 => vec![at].try_into().unwrap(),
            4 => {
                map.remove(&at);
                for nc in at.neigh() {
                    map.remove(&nc);
                }
                let (x, y) = at;
                vec![
                    (x - 1, y - 1),
                    (x + 1, y - 1),
                    (x + 1, y + 1),
                    (x - 1, y + 1),
                ]
                .try_into()
                .unwrap()
            }
            _ => panic!("N={} is not supported", N),
        }
    }

    fn new(inp: &str) -> Map<N> {
        let mut map = HashSet::<Coord>::new();
        let mut keys = BiHashMap::<Coord, Key>::new();
        let mut doors = BiHashMap::<Coord, Key>::new();
        let mut pos = None;
        for (row, line) in inp.lines().enumerate() {
            for (col, b) in line.chars().enumerate() {
                assert!(col < 127);
                assert!(row < 127);
                let c = (col as i8, row as i8);
                let passable = match b {
                    '#' => false,
                    '.' => true,
                    '@' => {
                        pos = Some(c);
                        true
                    }
                    k @ 'a'..='z' => {
                        keys.insert(c, k);
                        true
                    }
                    d @ 'A'..='Z' => {
                        doors.insert(c, d.to_ascii_lowercase());
                        true
                    }
                    _ => panic!("bad block {}", b),
                };
                if passable {
                    map.insert(c);
                }
            }
        }

        let at = pos.unwrap();
        let initial_pos = Self::adjust_map_and_make_initial_pos(&mut map, at);

        let mut m = Map {
            map,
            graph: HashMap::new(),
            keys,
            doors,
            initial_pos,
        };
        m.make_graph();
        m
    }

    fn make_graph(&mut self) {
        let mut seen = HashSet::<Coord>::new();
        for pos in self.initial_pos {
            seen.insert(pos);
        }
        let mut queue = VecDeque::<Coord>::new();
        for pos in self.initial_pos {
            queue.push_back(pos);
        }

        while let Some(c) = queue.pop_front() {
            let mut entry = Vec::<(Coord, Cost)>::new();
            for mut nc in c.neigh() {
                if !self.map.contains(&nc) {
                    continue;
                }
                let mut cost = 1;
                let mut prev = c;
                while !self.keys.contains_left(&nc) && !self.doors.contains_left(&nc) {
                    let nncs = nc
                        .neigh()
                        .into_iter()
                        .filter(|x| self.map.contains(x) && *x != prev)
                        .collect::<Vec<Coord>>();
                    if nncs.len() != 1 {
                        break;
                    }
                    prev = nc;
                    nc = nncs[0];
                    cost += 1;
                }
                entry.push((nc, cost));
                if !seen.contains(&nc) {
                    seen.insert(nc);
                    queue.push_back(nc);
                }
            }
            self.graph.insert(c, entry);
        }
    }

    fn paths(&self, pos: Coord, skeys: BM) -> Vec<(Coord, Key, Cost)> {
        let mut dist = HashMap::<Coord, Cost>::new();
        let mut queue = VecDeque::<(Coord, Cost)>::new();
        queue.push_back((pos, 0));
        while let Some((c, depth)) = queue.pop_front() {
            match dist.entry(c) {
                std::collections::hash_map::Entry::Occupied(_) => continue,
                std::collections::hash_map::Entry::Vacant(v) => {
                    v.insert(depth);
                    if let Some(key) = self.keys.get_by_left(&c) {
                        if !skeys.get(*key) {
                            continue;
                        }
                    }
                }
            };

            for (nc, cost) in &self.graph[&c] {
                if let Some(door) = self.doors.get_by_left(nc) {
                    if !skeys.get(*door) {
                        continue;
                    }
                }
                queue.push_back((*nc, depth + cost));
            }
        }

        let mut ret = Vec::new();
        for (c, key) in &self.keys {
            if skeys.get(*key) {
                continue;
            }
            match dist.get(c) {
                None => continue,
                Some(d) => ret.push((*c, *key, *d)),
            }
        }
        ret
    }

    fn search(&self) -> Cost {
        let mut q = PriorityQueue::<([Coord; N], BM), Reverse<Cost>>::new();
        let mut dist = HashMap::<([Coord; N], BM), Cost>::new();
        let mut seen = HashSet::<([Coord; N], BM)>::new();
        let mut goal: BM = 0;
        for (_, key) in &self.keys {
            goal = goal.set(*key);
        }
        let goal = goal;
        dist.insert((self.initial_pos, 0), 0);
        q.push((self.initial_pos, 0), Reverse(0));
        while !q.is_empty() {
            let ((pos, skeys), _) = q.pop().unwrap();
            let item = (pos, skeys);
            if skeys == goal {
                return dist[&item];
            }
            seen.insert(item);
            for (bot_idx, bot_pos) in pos.iter().enumerate() {
                for (new_bot_pos, key, cost) in self.paths(*bot_pos, skeys) {
                    let mut newpos = pos;
                    newpos[bot_idx] = new_bot_pos;
                    let newskeys = skeys.set(key);
                    let newitem = (newpos, newskeys);
                    if seen.contains(&newitem) {
                        continue;
                    }
                    let alt = dist[&item] + cost;
                    if dist.get(&newitem).map_or(true, |d| alt < *d) {
                        dist.insert(newitem, alt);
                        q.push_increase(newitem, Reverse(alt));
                    }
                }
            }
        }
        panic!("not found");
    }
}

fn part1(inp: &str) -> u32 {
    let map = Map::<1>::new(inp);
    //println!("{:?}", map.paths(map.initial_pos, 0));
    map.search()
}

fn part2(inp: &str) -> u32 {
    let map = Map::<4>::new(inp);
    //println!("{:?}", map.paths(map.initial_pos, 0));
    map.search()
}

xaoc::xaoc!();
