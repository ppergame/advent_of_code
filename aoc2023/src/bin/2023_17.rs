use ndarray::{Array2, ArrayView, Axis};

struct Map(Array2<u8>);

impl Map {
    fn parse(inp: &str) -> Self {
        let width = inp.lines().next().unwrap().chars().count();
        let mut map = Array2::default((0, width));
        for line in inp.lines() {
            let mut row = vec![];
            for c in line.chars() {
                row.push(c.to_digit(10).unwrap() as u8);
            }
            map.append(
                Axis(0),
                ArrayView::from(&row).into_shape((1, width)).unwrap(),
            )
            .unwrap();
        }
        Map(map)
    }

    #[allow(dead_code)]
    fn print(&self, path: &[State]) {
        let mut map = self.0.map(|&x| char::from_digit(x as u32, 10).unwrap());
        for st in path {
            map[(st.row, st.col)] = match st.dir {
                Dir::Up => '^',
                Dir::Right => '>',
                Dir::Down => 'v',
                Dir::Left => '<',
            };
        }
        for ((_, col), val) in map.indexed_iter() {
            eprint!("{val}");
            if col == map.ncols() - 1 {
                eprintln!();
            }
        }
        eprintln!();
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl Dir {
    fn turn_left(&mut self) {
        *self = match self {
            Dir::Up => Dir::Left,
            Dir::Right => Dir::Up,
            Dir::Down => Dir::Right,
            Dir::Left => Dir::Down,
        }
    }

    fn turn_right(&mut self) {
        *self = match self {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
        }
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct State {
    row: usize,
    col: usize,
    dir: Dir,
    straight: u8,
    start: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            row: 0,
            col: 0,
            dir: Dir::Right,
            straight: 0,
            start: true,
        }
    }
}

impl State {
    fn succ(&self, map: &Map) -> Vec<(State, usize)> {
        let mut ret = vec![];
        if self.straight < 3 {
            let mut next = *self;
            next.straight += 1;
            ret.push(next);
        }
        let mut next = *self;
        next.dir.turn_left();
        next.straight = 1;
        ret.push(next);

        let mut next = *self;
        next.dir.turn_right();
        next.straight = 1;
        ret.push(next);

        ret.into_iter()
            .filter_map(|mut n| {
                if !n.advance(map) {
                    return None;
                }
                Some((n, map.0[(n.row, n.col)] as usize))
            })
            .collect()
    }

    fn succ2(&self, map: &Map) -> Vec<(State, usize)> {
        let mut ret = vec![];
        if self.straight < 10 || self.start {
            let mut next = *self;
            next.straight += 1;
            ret.push(next);
        }
        if self.straight > 3 || self.start {
            let mut next = *self;
            next.dir.turn_left();
            next.straight = 1;
            ret.push(next);

            let mut next = *self;
            next.dir.turn_right();
            next.straight = 1;
            ret.push(next);
        }

        ret.into_iter()
            .filter_map(|mut n| {
                if !n.advance(map) {
                    return None;
                }
                n.start = false;
                Some((n, map.0[(n.row, n.col)] as usize))
            })
            .collect()
    }

    fn advance(&mut self, map: &Map) -> bool {
        match self.dir {
            Dir::Up => {
                if self.row == 0 {
                    return false;
                }
                self.row -= 1;
            }
            Dir::Right => {
                if self.col == map.0.ncols() - 1 {
                    return false;
                }
                self.col += 1;
            }
            Dir::Down => {
                if self.row == map.0.nrows() - 1 {
                    return false;
                }
                self.row += 1;
            }
            Dir::Left => {
                if self.col == 0 {
                    return false;
                }
                self.col -= 1;
            }
        }
        true
    }
}

fn part1(inp: &str) -> usize {
    let map = Map::parse(inp);
    let (erow, ecol) = (map.0.nrows() - 1, map.0.ncols() - 1);
    pathfinding::directed::dijkstra::dijkstra(
        &State::default(),
        |s| s.succ(&map),
        |s: &State| s.row == erow && s.col == ecol,
    )
    .unwrap()
    .1
}

fn part2(inp: &str) -> usize {
    let map = Map::parse(inp);
    let (erow, ecol) = (map.0.nrows() - 1, map.0.ncols() - 1);
    let (_path, cost) = pathfinding::directed::dijkstra::dijkstra(
        &State::default(),
        |s| s.succ2(&map),
        |s: &State| s.row == erow && s.col == ecol && s.straight > 3,
    )
    .unwrap();
    cost
}

xaoc::xaoc!();
