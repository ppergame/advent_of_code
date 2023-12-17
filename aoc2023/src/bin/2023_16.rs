use arrayvec::ArrayVec;
use ndarray::{Array2, ArrayView, Axis};
use rayon::prelude::*;

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
    Up = 1,
    Right = 2,
    Down = 4,
    Left = 8,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Beam {
    row: usize,
    col: usize,
    d: Dir,
}

impl Beam {
    fn advance(mut self, tile: Tile, map: &Map) -> ArrayVec<Beam, 2> {
        let mut ret = ArrayVec::new();
        match tile {
            Tile::Empty => (),
            Tile::FMirror => {
                self.d = match self.d {
                    Dir::Up => Dir::Right,
                    Dir::Right => Dir::Up,
                    Dir::Down => Dir::Left,
                    Dir::Left => Dir::Down,
                };
            }
            Tile::BMirror => {
                self.d = match self.d {
                    Dir::Up => Dir::Left,
                    Dir::Right => Dir::Down,
                    Dir::Down => Dir::Right,
                    Dir::Left => Dir::Up,
                };
            }
            Tile::HSplit => match self.d {
                Dir::Right | Dir::Left => (),
                Dir::Up | Dir::Down => {
                    let mut b2 = self;
                    self.d = Dir::Right;
                    b2.d = Dir::Left;
                    if b2.advance_dir(map) {
                        ret.push(b2);
                    }
                }
            },
            Tile::VSplit => match self.d {
                Dir::Up | Dir::Down => (),
                Dir::Right | Dir::Left => {
                    let mut b2 = self;
                    self.d = Dir::Up;
                    b2.d = Dir::Down;
                    if b2.advance_dir(map) {
                        ret.push(b2);
                    }
                }
            },
        }
        if self.advance_dir(map) {
            ret.push(self);
        }
        ret
    }

    fn advance_dir(&mut self, map: &Map) -> bool {
        match self.d {
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
        let mut seen = Array2::<u8>::zeros(self.0.dim());
        let mut stack = vec![initial];
        while let Some(beam) = stack.pop() {
            if seen[(beam.row, beam.col)] & beam.d as u8 != 0 {
                continue;
            }
            seen[(beam.row, beam.col)] |= beam.d as u8;
            let tile = self.0[(beam.row, beam.col)];
            stack.extend(beam.advance(tile, self));
        }
        seen.iter().filter(|&&x| x != 0).count()
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
                    row,
                    col: 0,
                    d: Dir::Right,
                },
                Beam {
                    row,
                    col: st.0.ncols() - 1,
                    d: Dir::Left,
                },
            ]
        })
        .chain((0..st.0.ncols()).flat_map(|col| {
            [
                Beam {
                    row: 0,
                    col,
                    d: Dir::Down,
                },
                Beam {
                    row: st.0.nrows() - 1,
                    col,
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
