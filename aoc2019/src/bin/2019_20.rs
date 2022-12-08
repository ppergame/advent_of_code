use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::collections::{HashMap, HashSet, VecDeque};

type Coord = (u32, u32);
type LevelCoord = (u8, Coord);

trait CoordMethods {
    fn neigh(&self) -> Vec<Coord>;
}

impl CoordMethods for Coord {
    fn neigh(&self) -> Vec<Coord> {
        let (x, y) = *self;

        vec![(x, y - 1), (x + 1, y), (x, y + 1), (x - 1, y)]
    }
}

type Cost = u32;

#[derive(Debug, Clone, Copy)]
enum PortalSide {
    Inner,
    Outer,
}

impl PortalSide {
    fn flip(&self) -> Self {
        match *self {
            PortalSide::Inner => PortalSide::Outer,
            PortalSide::Outer => PortalSide::Inner,
        }
    }
}

struct Map<const LEVELS: bool> {
    graph: HashMap<Coord, Vec<(Coord, Cost)>>,
    corridors: HashSet<Coord>,
    portals: HashMap<Coord, (Coord, PortalSide)>,
    start: Coord,
    end: Coord,
}

impl<const LEVELS: bool> Map<LEVELS> {
    fn new(inp: &str) -> Map<LEVELS> {
        let mut corridors = HashSet::<Coord>::new();
        let mut letters = HashMap::<Coord, char>::new();
        let mut portals = HashMap::<Coord, (Coord, PortalSide)>::new();

        let mut width = 0;
        let mut height = 0;

        for (row, line) in inp.lines().enumerate() {
            height = std::cmp::max(height, row);
            for (col, b) in line.chars().enumerate() {
                width = std::cmp::max(width, col);
                let (col, row) = (col as u32, row as u32);
                match b {
                    '.' => {
                        corridors.insert((col, row));
                    }
                    'A'..='Z' => {
                        letters.insert((col, row), b);
                    }
                    ' ' | '#' => (),
                    _ => panic!("unknown block"),
                }
            }
        }
        let width = width as u32;
        let height = height as u32;

        let mut tag_to_tile = HashMap::<String, Coord>::new();
        for (&c, &l) in &letters {
            let (x, y) = c;
            for (other_letter_c, tile_c) in [
                ((x - 1, y), (x + 1, y)),
                ((x - 1, y), (x - 2, y)),
                ((x, y - 1), (x, y + 1)),
                ((x, y - 1), (x, y - 2)),
            ] {
                if let Some(other_letter) = letters.get(&other_letter_c) {
                    if corridors.contains(&tile_c) {
                        let side = if x > 2 && x < width - 2 && y > 2 && y < height - 2 {
                            PortalSide::Inner
                        } else {
                            PortalSide::Outer
                        };
                        let tag = format!("{}{}", other_letter, l);
                        /*
                        println!(
                            "found tag {} @ {}, {} side {:?}",
                            tag, tile_c.0, tile_c.1, side
                        );
                        */
                        match tag_to_tile.get(&tag) {
                            None => {
                                tag_to_tile.insert(tag, tile_c);
                            }
                            Some(other_tile_c) => {
                                portals.insert(tile_c, (*other_tile_c, side));
                                portals.insert(*other_tile_c, (tile_c, side.flip()));
                            }
                        }
                        break;
                    }
                }
            }
        }
        //println!("{:?}", portals);

        let start = tag_to_tile["AA"];
        let end = tag_to_tile["ZZ"];

        let graph = HashMap::<Coord, Vec<(Coord, Cost)>>::new();
        let mut m = Map {
            graph,
            corridors,
            portals,
            start,
            end,
        };
        m.make_graph();
        m
    }

    fn neigh_corr(&self, c: Coord) -> Vec<Coord> {
        c.neigh()
            .into_iter()
            .filter(|x| self.corridors.contains(x))
            .collect()
    }

    fn make_graph(&mut self) {
        let mut seen = HashSet::<Coord>::new();
        seen.insert(self.start);
        seen.extend(self.portals.keys().clone());
        let mut queue = VecDeque::<Coord>::new();
        queue.extend(seen.clone());

        while let Some(c) = queue.pop_front() {
            let mut entry = Vec::<(Coord, Cost)>::new();
            for mut nc in self.neigh_corr(c) {
                let mut cost = 1;
                let mut prev = c;
                while nc != self.start && nc != self.end && !self.portals.contains_key(&nc) {
                    let nncs = self
                        .neigh_corr(nc)
                        .into_iter()
                        .filter(|&x| x != prev)
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

    fn search(&self) -> Cost {
        let start = (0, self.start);
        let end = (0, self.end);
        let mut q = PriorityQueue::<LevelCoord, Reverse<Cost>>::new();
        let mut dist = HashMap::<LevelCoord, Cost>::new();
        let mut seen = HashSet::<LevelCoord>::new();

        dist.insert(start, 0);
        q.push(start, Reverse(0));
        while !q.is_empty() {
            let (dc, _) = q.pop().unwrap();
            if dc == end {
                return dist[&dc];
            }
            seen.insert(dc);
            let (level, c) = dc;
            let mut neigh = self.graph[&c]
                .iter()
                .map(|&(x, cost)| ((level, x), cost))
                .collect::<Vec<(LevelCoord, Cost)>>();
            if let Some(&(target_c, side)) = self.portals.get(&c) {
                if LEVELS {
                    if let Some(ndc) = match side {
                        PortalSide::Inner => Some((level + 1, target_c)),
                        PortalSide::Outer => {
                            if level > 0 {
                                Some((level - 1, target_c))
                            } else {
                                None
                            }
                        }
                    } {
                        neigh.push((ndc, 1));
                    }
                } else {
                    neigh.push(((0, target_c), 1));
                }
            }
            for (ndc, cost) in neigh {
                if seen.contains(&ndc) {
                    continue;
                }
                let alt = dist[&dc] + cost;
                if dist.get(&ndc).map_or(true, |&d| alt < d) {
                    dist.insert(ndc, alt);
                    q.push_increase(ndc, Reverse(alt));
                }
            }
        }
        panic!("not found");
    }
}

fn part1(inp: &str) -> u32 {
    let m = Map::<false>::new(inp);
    m.search()
}

fn part2(inp: &str) -> u32 {
    let m = Map::<true>::new(inp);
    m.search()
}

xaoc::xaoc!();
