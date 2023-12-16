use ndarray::{Array2, ArrayView, Axis};
use rayon::prelude::*;
use std::collections::HashSet;

#[derive(Debug, Default, Copy, Clone)]
enum Tile {
    #[default]
    Empty,
    // "/"
    FMirror,
    // "\"
    BMirror,
    // "-"
    HSplit,
    // "|"
    VSplit,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Beam {
    row: i64,
    col: i64,
    d: Dir,
}

impl Beam {
    fn advance(&mut self, tile: Tile) -> Option<Beam> {
        match tile {
            Tile::Empty => self.advance_dir(),
            Tile::FMirror => {
                self.d = match self.d {
                    Dir::Up => Dir::Right,
                    Dir::Right => Dir::Up,
                    Dir::Down => Dir::Left,
                    Dir::Left => Dir::Down,
                };
                self.advance_dir();
            }
            Tile::BMirror => {
                self.d = match self.d {
                    Dir::Up => Dir::Left,
                    Dir::Right => Dir::Down,
                    Dir::Down => Dir::Right,
                    Dir::Left => Dir::Up,
                };
                self.advance_dir();
            }
            Tile::HSplit => match self.d {
                Dir::Right | Dir::Left => self.advance_dir(),
                Dir::Up | Dir::Down => {
                    let mut b2 = *self;
                    self.d = Dir::Right;
                    self.advance_dir();
                    b2.d = Dir::Left;
                    b2.advance_dir();
                    return Some(b2);
                }
            },
            Tile::VSplit => match self.d {
                Dir::Up | Dir::Down => self.advance_dir(),
                Dir::Right | Dir::Left => {
                    let mut b2 = *self;
                    self.d = Dir::Up;
                    self.advance_dir();
                    b2.d = Dir::Down;
                    b2.advance_dir();
                    return Some(b2);
                }
            },
        }
        None
    }

    fn advance_dir(&mut self) {
        match self.d {
            Dir::Up => self.row -= 1,
            Dir::Right => self.col += 1,
            Dir::Down => self.row += 1,
            Dir::Left => self.col -= 1,
        }
    }
}

#[derive(Debug, Clone)]
struct State {
    map: Array2<Tile>,
    lit: HashSet<(i64, i64)>,
    seen: HashSet<Beam>,
}

impl State {
    fn parse(inp: &str) -> Self {
        let width = inp.lines().next().unwrap().chars().count();
        let mut map = Array2::default((0, width));
        for line in inp.lines() {
            let mut row = vec![];
            for c in line.chars() {
                row.push(match c {
                    '.' => Tile::Empty,
                    '/' => Tile::FMirror,
                    '\\' => Tile::BMirror,
                    '-' => Tile::HSplit,
                    '|' => Tile::VSplit,
                    _ => panic!("bad char"),
                });
            }
            map.append(
                Axis(0),
                ArrayView::from(&row).into_shape((1, width)).unwrap(),
            )
            .unwrap();
        }
        State {
            map,
            lit: HashSet::new(),
            seen: HashSet::new(),
        }
    }

    fn handle(&mut self, mut beam: Beam) {
        loop {
            if self.seen.contains(&beam) || !self.h(&beam) {
                return;
            }
            self.lit.insert((beam.row, beam.col));
            self.seen.insert(beam);
            let tile = self.g(&beam);
            let b2 = beam.advance(tile);
            if let Some(b2) = b2 {
                self.handle(b2);
            }
        }
    }

    fn g(&self, b: &Beam) -> Tile {
        self.map[(b.row as usize, b.col as usize)]
    }

    fn h(&self, b: &Beam) -> bool {
        self.map.get((b.row as usize, b.col as usize)).is_some()
    }
}

fn part1(inp: &str) -> usize {
    let mut st = State::parse(inp);
    st.handle(Beam {
        row: 0,
        col: 0,
        d: Dir::Right,
    });
    st.lit.len()
}

fn part2(inp: &str) -> usize {
    let st = State::parse(inp);
    let initial = (0..st.map.nrows())
        .flat_map(|row| {
            [
                Beam {
                    row: row as i64,
                    col: 0,
                    d: Dir::Right,
                },
                Beam {
                    row: row as i64,
                    col: st.map.ncols() as i64 - 1,
                    d: Dir::Left,
                },
            ]
        })
        .chain((0..st.map.ncols()).flat_map(|col| {
            [
                Beam {
                    row: 0,
                    col: col as i64,
                    d: Dir::Down,
                },
                Beam {
                    row: st.map.nrows() as i64 - 1,
                    col: col as i64,
                    d: Dir::Up,
                },
            ]
        }))
        .collect::<Vec<_>>();
    initial
        .into_par_iter()
        .map(|b| {
            let mut st = st.clone();
            st.handle(b);
            st.lit.len()
        })
        .max()
        .unwrap()
}

xaoc::xaoc!(
    sample = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."
);
