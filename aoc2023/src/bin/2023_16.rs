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
struct Map(Array2<Tile>);

impl Map {
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
        Map(map)
    }

    fn handle(&self, initial: Beam) -> usize {
        let mut lit = HashSet::new();
        let mut seen = HashSet::new();
        let mut stack = vec![initial];
        while let Some(mut beam) = stack.pop() {
            if !self.h(&beam) || seen.contains(&beam) {
                continue;
            }
            lit.insert((beam.row, beam.col));
            seen.insert(beam);
            let tile = self.g(&beam);
            let b2 = beam.advance(tile);
            if let Some(b2) = b2 {
                stack.push(b2);
            }
            stack.push(beam);
        }
        lit.len()
    }

    fn g(&self, b: &Beam) -> Tile {
        self.0[(b.row as usize, b.col as usize)]
    }

    fn h(&self, b: &Beam) -> bool {
        self.0.get((b.row as usize, b.col as usize)).is_some()
    }
}

fn part1(inp: &str) -> usize {
    let st = Map::parse(inp);
    st.handle(Beam {
        row: 0,
        col: 0,
        d: Dir::Right,
    })
}

fn part2(inp: &str) -> usize {
    let st = Map::parse(inp);
    let initial = (0..st.0.nrows())
        .flat_map(|row| {
            [
                Beam {
                    row: row as i64,
                    col: 0,
                    d: Dir::Right,
                },
                Beam {
                    row: row as i64,
                    col: st.0.ncols() as i64 - 1,
                    d: Dir::Left,
                },
            ]
        })
        .chain((0..st.0.ncols()).flat_map(|col| {
            [
                Beam {
                    row: 0,
                    col: col as i64,
                    d: Dir::Down,
                },
                Beam {
                    row: st.0.nrows() as i64 - 1,
                    col: col as i64,
                    d: Dir::Up,
                },
            ]
        }))
        .collect::<Vec<_>>();
    initial.into_par_iter().map(|b| st.handle(b)).max().unwrap()
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
