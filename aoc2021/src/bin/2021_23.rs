use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref HALLWAY: &'static [Point] =
        &[(1, 1), (2, 1), (4, 1), (6, 1), (8, 1), (10, 1), (11, 1),];
    static ref A_ROOM: &'static [Point] = &[(3, 2), (3, 3), (3, 4), (3, 5)];
    static ref B_ROOM: &'static [Point] = &[(5, 2), (5, 3), (5, 4), (5, 5)];
    static ref C_ROOM: &'static [Point] = &[(7, 2), (7, 3), (7, 4), (7, 5)];
    static ref D_ROOM: &'static [Point] = &[(9, 2), (9, 3), (9, 4), (9, 5)];
    static ref ALLOWED_ROOMS: HashMap<Cell, &'static [Point]> = HashMap::from_iter([
        (Cell::A, *A_ROOM),
        (Cell::B, *B_ROOM),
        (Cell::C, *C_ROOM),
        (Cell::D, *D_ROOM),
    ]);
    static ref COST: HashMap<Cell, usize> =
        HashMap::from_iter([(Cell::A, 1), (Cell::B, 10), (Cell::C, 100), (Cell::D, 1000)]);
}

type Point = (usize, usize);

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
enum Cell {
    Void,
    Wall,
    Open,
    A,
    B,
    C,
    D,
}

impl Cell {
    fn is_am(&self) -> bool {
        matches!(self, Cell::A | Cell::B | Cell::C | Cell::D)
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Game {
    map: Vec<Vec<Cell>>,
    limit: usize,
}

impl Game {
    fn get(&self, p: Point) -> Cell {
        let (x, y) = p;
        self.map[y][x]
    }

    fn set(&mut self, p: Point, cell: Cell) {
        let (x, y) = p;
        self.map[y][x] = cell;
    }

    fn win(&self) -> bool {
        for &rp in A_ROOM.iter().take(self.limit) {
            if self.get(rp) != Cell::A {
                return false;
            }
        }
        for &rp in B_ROOM.iter().take(self.limit) {
            if self.get(rp) != Cell::B {
                return false;
            }
        }
        for &rp in C_ROOM.iter().take(self.limit) {
            if self.get(rp) != Cell::C {
                return false;
            }
        }
        for &rp in D_ROOM.iter().take(self.limit) {
            if self.get(rp) != Cell::D {
                return false;
            }
        }
        true
    }

    fn succ(&self) -> Vec<(Game, usize)> {
        let mut ret = vec![];
        'outer: for &hp in HALLWAY.iter() {
            let cell = self.get(hp);
            if !cell.is_am() {
                continue;
            }
            let room = &ALLOWED_ROOMS[&cell][..self.limit];
            for &rp in room {
                let rcell = self.get(rp);
                if rcell.is_am() && rcell != cell {
                    continue 'outer;
                }
            }
            if let Some((plen, dest)) = room
                .iter()
                .rev()
                .find_map(|&rp| self.path(hp, rp).map(|plen| (plen, rp)))
            {
                let mut succ = self.clone();
                succ.set(hp, Cell::Open);
                succ.set(dest, cell);
                ret.push((succ, plen * COST[&cell]));
            }
        }

        for &rp in [
            &A_ROOM[..self.limit],
            &B_ROOM[..self.limit],
            &C_ROOM[..self.limit],
            &D_ROOM[..self.limit],
        ]
        .into_iter()
        .flatten()
        {
            let cell = self.get(rp);
            if !cell.is_am() {
                continue;
            }
            for &hp in HALLWAY.iter() {
                if let Some(plen) = self.path(rp, hp) {
                    let mut succ = self.clone();
                    succ.set(rp, Cell::Open);
                    succ.set(hp, cell);
                    ret.push((succ, plen * COST[&cell]));
                }
            }
        }

        ret
    }

    fn path(&self, p1: Point, p2: Point) -> Option<usize> {
        pathfinding::directed::dfs::dfs(
            p1,
            |&(x, y)| {
                [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
                    .into_iter()
                    .filter(|&p| self.get(p) == Cell::Open)
            },
            |&p| p == p2,
        )
        .map(|path| path.len() - 1)
    }
}

fn parse(inp: &str) -> Game {
    let mut map = vec![];
    for line in inp.lines() {
        let mut row = vec![];
        for c in line.chars() {
            row.push(match c {
                ' ' => Cell::Void,
                '#' => Cell::Wall,
                '.' => Cell::Open,
                'A' => Cell::A,
                'B' => Cell::B,
                'C' => Cell::C,
                'D' => Cell::D,
                _ => unreachable!(),
            });
        }
        map.push(row);
    }
    Game { map, limit: 2 }
}

fn part1(inp: &str) -> usize {
    let inp = parse(inp);
    let (_, cost) =
        pathfinding::directed::dijkstra::dijkstra(&inp, |g| g.succ(), |g| g.win()).unwrap();
    cost
}

fn part2(inp: &str) -> usize {
    let mut game = parse(inp);
    game.limit = 4;
    game.map.insert(
        3,
        vec![
            Cell::Void,
            Cell::Void,
            Cell::Wall,
            Cell::D,
            Cell::Wall,
            Cell::B,
            Cell::Wall,
            Cell::A,
            Cell::Wall,
            Cell::C,
            Cell::Wall,
            Cell::Void,
            Cell::Void,
        ],
    );
    game.map.insert(
        3,
        vec![
            Cell::Void,
            Cell::Void,
            Cell::Wall,
            Cell::D,
            Cell::Wall,
            Cell::C,
            Cell::Wall,
            Cell::B,
            Cell::Wall,
            Cell::A,
            Cell::Wall,
            Cell::Void,
            Cell::Void,
        ],
    );
    let (_, cost) =
        pathfinding::directed::dijkstra::dijkstra(&game, |g| g.succ(), |g| g.win()).unwrap();
    cost
}

xaoc::xaoc!();
