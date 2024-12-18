use hashbrown::HashSet;
use pathfinding::prelude::{astar_bag, dijkstra};

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
enum Dir {
    North,
    East,
    South,
    West,
}

impl Dir {
    fn delta(&self) -> (i64, i64) {
        match self {
            Self::North => (-1, 0),
            Self::East => (0, 1),
            Self::South => (1, 0),
            Self::West => (0, -1),
        }
    }

    fn turn_left(&self) -> Self {
        match self {
            Self::North => Self::West,
            Self::East => Self::North,
            Self::South => Self::East,
            Self::West => Self::South,
        }
    }

    fn turn_right(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Pos {
    row: i64,
    col: i64,
    dir: Dir,
}

struct Map {
    walls: HashSet<(i64, i64)>,
    #[allow(unused)]
    max_row: i64,
    #[allow(unused)]
    max_col: i64,
    start: (i64, i64),
    goal: (i64, i64),
}

impl Map {
    fn parse(inp: &str) -> Self {
        let mut walls = HashSet::new();
        let mut max_row = 0;
        let mut max_col = 0;
        let mut start = (0, 0);
        let mut goal = (0, 0);
        for (row, line) in inp.lines().enumerate() {
            let row = row as i64;
            max_row = max_row.max(row);
            for (col, c) in line.chars().enumerate() {
                let col = col as i64;
                max_col = max_col.max(col);
                match c {
                    '#' => {
                        walls.insert((row, col));
                    }
                    'S' => {
                        start = (row, col);
                    }
                    'E' => {
                        goal = (row, col);
                    }
                    _ => {}
                }
            }
        }
        Self {
            walls,
            max_row,
            max_col,
            start,
            goal,
        }
    }

    fn succ(&self, pos: Pos) -> Vec<(Pos, u64)> {
        let mut ret = vec![];
        let (dr, dc) = pos.dir.delta();
        let (nr, nc) = (pos.row + dr, pos.col + dc);
        if !self.walls.contains(&(nr, nc)) {
            ret.push((
                Pos {
                    row: nr,
                    col: nc,
                    dir: pos.dir,
                },
                1,
            ));
        }
        ret.push((
            Pos {
                row: pos.row,
                col: pos.col,
                dir: pos.dir.turn_left(),
            },
            1000,
        ));
        ret.push((
            Pos {
                row: pos.row,
                col: pos.col,
                dir: pos.dir.turn_right(),
            },
            1000,
        ));
        ret
    }
}

fn part1(inp: &str) -> u64 {
    let map = Map::parse(inp);
    let (sr, sc) = map.start;
    let (gr, gc) = map.goal;
    dijkstra(
        &Pos {
            row: sr,
            col: sc,
            dir: Dir::East,
        },
        |pos| map.succ(*pos),
        |pos| pos.row == gr && pos.col == gc,
    )
    .unwrap()
    .1
}

fn part2(inp: &str) -> usize {
    let map = Map::parse(inp);
    let (sr, sc) = map.start;
    let (gr, gc) = map.goal;
    let mut ret = HashSet::new();
    for path in astar_bag(
        &Pos {
            row: sr,
            col: sc,
            dir: Dir::East,
        },
        |pos| map.succ(*pos),
        |pos| pos.row.abs_diff(gr) + pos.col.abs_diff(gc),
        |pos| pos.row == gr && pos.col == gc,
    )
    .unwrap()
    .0
    {
        for pos in path {
            ret.insert((pos.row, pos.col));
        }
    }
    ret.len()
}

xaoc::xaoc!(
    sample = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################"
);
