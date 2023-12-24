use arrayvec::ArrayVec;
use ndarray::{Array2, ArrayView, Axis};
use std::borrow::Cow;

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

fn dfs<'a>(succ: impl Fn((usize, usize)) -> Cow<'a, Succ>, goal: (usize, usize)) -> usize {
    let mut seen = Array2::default((goal.0 + 1, goal.1 + 1));
    let mut todo = vec![((0, 1), Some(0))];
    let mut best = 0;
    while let Some((pos, steps)) = todo.pop() {
        let Some(steps) = steps else {
            seen[pos] = false;
            continue;
        };
        if pos == goal {
            best = best.max(steps);
            continue;
        }
        if seen[pos] {
            continue;
        }
        seen[pos] = true;
        todo.push((pos, None));
        for (pos, ssteps) in succ(pos).iter() {
            todo.push((*pos, Some(steps + ssteps)));
        }
    }
    best
}

struct Part1 {
    map: Map,
    goal: (usize, usize),
}

impl Part1 {
    fn new(inp: &str) -> Self {
        let map = parse(inp);
        let goal = (map.nrows() - 1, map.ncols() - 2);
        Self { map, goal }
    }

    fn succ(&self, (row, col): (usize, usize)) -> Cow<Succ> {
        let mut ret = ArrayVec::new();
        match self.map[(row, col)] {
            Tile::Path => {
                if row > 0 && self.map[(row - 1, col)] != Tile::Wall {
                    ret.push(((row - 1, col), 1));
                }
                if row < self.map.nrows() - 1 && self.map[(row + 1, col)] != Tile::Wall {
                    ret.push(((row + 1, col), 1));
                }
                if col > 0 && self.map[(row, col - 1)] != Tile::Wall {
                    ret.push(((row, col - 1), 1));
                }
                if col < self.map.ncols() - 1 && self.map[(row, col + 1)] != Tile::Wall {
                    ret.push(((row, col + 1), 1));
                }
                return Cow::Owned(ret);
            }
            Tile::Wall => unreachable!(),
            Tile::Up => ret.push(((row - 1, col), 1)),
            Tile::Right => ret.push(((row, col + 1), 1)),
            Tile::Down => ret.push(((row + 1, col), 1)),
            Tile::Left => ret.push(((row, col - 1), 1)),
        }
        Cow::Owned(ret)
    }
}

fn part1(inp: &str) -> usize {
    let part1 = Part1::new(inp);
    dfs(|pos| part1.succ(pos), part1.goal)
}

type Succ = ArrayVec<((usize, usize), usize), 4>;

struct Part2 {
    succ: Array2<Succ>,
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
        Self { succ, goal }
    }

    fn succ(&self, (row, col): (usize, usize)) -> Cow<'_, Succ> {
        Cow::Borrowed(&self.succ[(row, col)])
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
    let part2 = Part2::new(inp);
    dfs(|pos| part2.succ(pos), part2.goal)
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
