use pathfinding::prelude::*;
use sscanf::scanf;
use std::collections::HashMap;

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Type {
    Rocky = 0,
    Wet = 1,
    Narrow = 2,
}

impl Type {
    fn new(ero: i64) -> Self {
        match ero % 3 {
            0 => Type::Rocky,
            1 => Type::Wet,
            2 => Type::Narrow,
            _ => unreachable!(),
        }
    }

    fn new_gear(&self, gear: Gear, new_type: Type) -> (Gear, usize) {
        if *self == new_type {
            return (gear, 0);
        }
        let new_gear = self.in_common(new_type);
        let cost = if gear == new_gear { 0 } else { 7 };
        (new_gear, cost)
    }

    fn in_common(&self, new_type: Type) -> Gear {
        assert_ne!(*self, new_type);
        match (self, new_type) {
            (Type::Rocky, Type::Wet) => Gear::Climbing,
            (Type::Rocky, Type::Narrow) => Gear::Torch,
            (Type::Wet, Type::Rocky) => Gear::Climbing,
            (Type::Wet, Type::Narrow) => Gear::Neither,
            (Type::Narrow, Type::Rocky) => Gear::Torch,
            (Type::Narrow, Type::Wet) => Gear::Neither,
            _ => unreachable!(),
        }
    }
}

struct Map {
    depth: i64,
    target: (i64, i64),
    map: HashMap<(i64, i64), i64>,
}

impl Map {
    fn parse(inp: &str) -> Self {
        let mut li = inp.lines();
        let depth = scanf!(li.next().unwrap(), "depth: {}", i64).unwrap();
        let (tcol, trow) = scanf!(li.next().unwrap(), "target: {},{}", i64, i64).unwrap();
        Self {
            depth,
            target: (trow, tcol),
            map: HashMap::new(),
        }
    }

    fn fill(&mut self, row: i64, col: i64) -> i64 {
        if let Some(&ero) = self.map.get(&(row, col)) {
            return ero;
        }
        let ero = match (row, col) {
            _ if (row, col) == self.target => 0,
            (0, 0) => 0,
            (0, _) => (col * 16807 + self.depth) % 20183,
            (_, 0) => (row * 48271 + self.depth) % 20183,
            (_, _) => (self.fill(row - 1, col) * self.fill(row, col - 1) + self.depth) % 20183,
        };
        self.map.insert((row, col), ero);
        ero
    }

    fn risk(&mut self, row: i64, col: i64) -> Type {
        Type::new(self.fill(row, col))
    }

    #[allow(dead_code)]
    fn print(&mut self, state: &State) {
        for row in 0..=self.target.0 {
            for col in 0..=self.target.1 {
                if (row, col) == (0, 0) {
                    print!("M");
                } else if (row, col) == self.target {
                    print!("T");
                } else if (row, col) == (state.row, state.col) {
                    print!(
                        "{}",
                        match state.gear {
                            Gear::Neither => 'n',
                            Gear::Torch => 't',
                            Gear::Climbing => 'c',
                        }
                    );
                } else {
                    let c = match self.risk(row, col) {
                        Type::Rocky => '.',
                        Type::Wet => '=',
                        Type::Narrow => '|',
                    };
                    print!("{c}");
                }
            }
            println!();
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
enum Gear {
    Neither,
    Torch,
    Climbing,
}

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
struct State {
    row: i64,
    col: i64,
    gear: Gear,
}

impl State {
    fn is_goal(&self, target: (i64, i64)) -> bool {
        self.gear == Gear::Torch && (self.row, self.col) == target
    }

    fn succ(&self, map: &mut Map) -> Vec<(Self, usize)> {
        if self.gear != Gear::Torch && (self.row, self.col) == map.target {
            let mut next = *self;
            next.gear = Gear::Torch;
            return vec![(next, 7)];
        }
        let mut neigh = vec![];
        let (row, col) = (self.row, self.col);
        if col > 0 {
            neigh.push((row, col - 1));
        }
        if row > 0 {
            neigh.push((row - 1, col));
        }
        neigh.push((row, col + 1));
        neigh.push((row + 1, col));
        let mut ret = vec![];
        for (nrow, ncol) in neigh {
            let mut next = *self;
            (next.row, next.col) = (nrow, ncol);
            let (new_gear, cost) = map.risk(row, col).new_gear(self.gear, map.risk(nrow, ncol));
            next.gear = new_gear;
            ret.push((next, cost + 1));
        }
        ret
    }
}

fn part1(inp: &str) -> i64 {
    let mut map = Map::parse(inp);
    let mut ret = 0;
    for row in 0..=map.target.0 {
        for col in 0..=map.target.1 {
            ret += map.risk(row, col) as i64;
        }
    }
    ret
}

fn part2(inp: &str) -> usize {
    let mut map = Map::parse(inp);
    let initial = State {
        row: 0,
        col: 0,
        gear: Gear::Torch,
    };
    let target = map.target;
    let (_path, cost) = dijkstra(&initial, |s| s.succ(&mut map), |s| s.is_goal(target)).unwrap();
    cost
}

xaoc::xaoc!(
    sample = r#"depth: 510
target: 10,10"#
);
