use std::collections::VecDeque;

use arrayvec::ArrayVec;
use ndarray::{Array2, ArrayView, Axis};

#[derive(Default, Copy, Clone, PartialEq, Eq)]
enum Tile {
    Path,
    #[default]
    Wall,
    Up,
    Right,
    Down,
    Left,
}

type Map = Array2<Tile>;

fn parse(inp: &str) -> Map {
    let width = inp.lines().next().unwrap().chars().count();
    let mut map = Array2::default((0, width));
    for line in inp.lines() {
        let mut row = vec![];
        for c in line.chars() {
            row.push(match c {
                '#' => Tile::Wall,
                '.' => Tile::Path,
                '>' => Tile::Right,
                '<' => Tile::Left,
                '^' => Tile::Up,
                'v' => Tile::Down,
                _ => unreachable!(),
            });
        }
        map.append(
            Axis(0),
            ArrayView::from(&row).into_shape((1, width)).unwrap(),
        )
        .unwrap();
    }
    map
}

struct Part1 {
    map: Map,
    seen: Array2<bool>,
    goal: (usize, usize),
}

impl Part1 {
    fn new(inp: &str) -> Self {
        let map = parse(inp);
        let seen = Array2::default(map.raw_dim());
        let goal = (map.nrows() - 1, map.ncols() - 2);
        Self { map, seen, goal }
    }

    fn find_longest(&mut self, pos: (usize, usize)) -> Option<usize> {
        if self.seen[pos] {
            return None;
        }
        if pos == self.goal {
            return Some(0);
        }
        self.seen[pos] = true;
        let ret = self
            .succ(pos)
            .into_iter()
            .filter_map(|n| self.find_longest(n))
            .map(|n| n + 1)
            .max();
        self.seen[pos] = false;
        ret
    }

    fn succ(&self, (row, col): (usize, usize)) -> Adj {
        let mut ret = ArrayVec::new();
        match self.map[(row, col)] {
            Tile::Path => {
                if row > 0 && self.map[(row - 1, col)] != Tile::Wall {
                    ret.push((row - 1, col));
                }
                if row < self.map.nrows() - 1 && self.map[(row + 1, col)] != Tile::Wall {
                    ret.push((row + 1, col));
                }
                if col > 0 && self.map[(row, col - 1)] != Tile::Wall {
                    ret.push((row, col - 1));
                }
                if col < self.map.ncols() - 1 && self.map[(row, col + 1)] != Tile::Wall {
                    ret.push((row, col + 1));
                }
                return ret;
            }
            Tile::Wall => unreachable!(),
            Tile::Up => ret.push((row - 1, col)),
            Tile::Right => ret.push((row, col + 1)),
            Tile::Down => ret.push((row + 1, col)),
            Tile::Left => ret.push((row, col - 1)),
        }
        ret
    }
}

fn part1(inp: &str) -> usize {
    let mut part1 = Part1::new(inp);
    part1.find_longest((0, 1)).unwrap()
}

type Succ = ArrayVec<((usize, usize), usize), 4>;

struct Part2 {
    succ: Array2<Succ>,
    seen: Array2<bool>,
    goal: (usize, usize),
}

impl Part2 {
    fn new(inp: &str) -> Self {
        let map = parse(inp);
        let goal = (map.nrows() - 1, map.ncols() - 2);

        let mut succ = Array2::default(map.raw_dim());
        let mut seen = Array2::default(map.raw_dim());
        let mut todo = vec![(0, 1)];
        while let Some(pos) = todo.pop() {
            if seen[pos] {
                continue;
            }
            seen[pos] = true;
            succ[pos] = ext_adj(&map, pos);
            todo.extend(succ[pos].iter().map(|&(n, _)| n));
        }
        let seen = Array2::default(map.raw_dim());
        Self { succ, seen, goal }
    }

    fn find_longest(&mut self, pos: (usize, usize)) -> Option<usize> {
        if self.seen[pos] {
            return None;
        }
        if pos == self.goal {
            return Some(0);
        }
        self.seen[pos] = true;
        let ret = self.succ[pos]
            .clone()
            .into_iter()
            .filter_map(|(n, steps)| self.find_longest(n).map(|n| n + steps))
            .max();
        self.seen[pos] = false;
        ret
    }
}

fn ext_adj(map: &Map, pos: (usize, usize)) -> Succ {
    adj(map, pos)
        .into_iter()
        .map(|mut cur| {
            let mut steps = 1;
            let mut prev = pos;
            loop {
                let nadj = adj(map, cur)
                    .into_iter()
                    .filter(|&nn| nn != prev)
                    .collect::<Adj>();
                if nadj.len() != 1 {
                    break;
                }
                prev = cur;
                cur = nadj[0];
                steps += 1;
            }
            (cur, steps)
        })
        .collect()
}

type Adj = ArrayVec<(usize, usize), 4>;

fn adj(map: &Map, (row, col): (usize, usize)) -> Adj {
    let mut ret = ArrayVec::new();
    if row > 0 && map[(row - 1, col)] != Tile::Wall {
        ret.push((row - 1, col));
    }
    if row < map.nrows() - 1 && map[(row + 1, col)] != Tile::Wall {
        ret.push((row + 1, col));
    }
    if col > 0 && map[(row, col - 1)] != Tile::Wall {
        ret.push((row, col - 1));
    }
    if col < map.ncols() - 1 && map[(row, col + 1)] != Tile::Wall {
        ret.push((row, col + 1));
    }
    ret
}

fn part2(inp: &str) -> usize {
    let mut part2 = Part2::new(inp);
    // part2.find_longest((0, 1)).unwrap()
    part2.find_longest((0, 1)).unwrap()
}

xaoc::xaoc!(
    sample = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#"
);
