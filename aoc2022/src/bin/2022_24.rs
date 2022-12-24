use array2d::Array2D;
use pathfinding::prelude::*;

#[derive(Copy, Clone)]
enum Blizz {
    Up,
    Right,
    Down,
    Left,
}

impl Blizz {
    fn parse(c: char) -> Option<Self> {
        match c {
            '^' => Some(Blizz::Up),
            '>' => Some(Blizz::Right),
            'v' => Some(Blizz::Down),
            '<' => Some(Blizz::Left),
            _ => None,
        }
    }

    fn as_char(&self) -> char {
        match self {
            Blizz::Up => '^',
            Blizz::Right => '>',
            Blizz::Down => 'v',
            Blizz::Left => '<',
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct State {
    pos: Option<(usize, usize)>,
    turn: usize,
}

impl State {
    fn succ(&self, map: &mut Map) -> Vec<Self> {
        let mut ret = vec![];
        let turn = self.turn + 1;
        match self.pos {
            None => {
                ret.push(Self {
                    pos: self.pos,
                    turn,
                });
                if map.safe_at(turn, 0, 0) {
                    ret.push(Self {
                        pos: Some((0, 0)),
                        turn,
                    });
                }
            }
            Some((row, col)) => {
                if row == 0 && col == 0 {
                    ret.push(Self { pos: None, turn });
                }
                if row == map.height - 1 && col == map.width - 1 {
                    ret.push(Self {
                        pos: Some((map.height, map.width - 1)),
                        turn,
                    });
                }
                if row == map.height && col == map.width - 1 {
                    ret.push(Self {
                        pos: Some((row - 1, col)),
                        turn,
                    });
                    ret.push(Self {
                        pos: Some((row, col)),
                        turn,
                    });
                    return ret;
                }
                if map.safe_at(turn, row, col) {
                    ret.push(Self {
                        pos: Some((row, col)),
                        turn,
                    });
                }
                if row > 0 && map.safe_at(turn, row - 1, col) {
                    ret.push(Self {
                        pos: Some((row - 1, col)),
                        turn,
                    });
                }
                if col > 0 && map.safe_at(turn, row, col - 1) {
                    ret.push(Self {
                        pos: Some((row, col - 1)),
                        turn,
                    });
                }
                if row < map.height - 1 && map.safe_at(turn, row + 1, col) {
                    ret.push(Self {
                        pos: Some((row + 1, col)),
                        turn,
                    });
                }
                if col < map.width - 1 && map.safe_at(turn, row, col + 1) {
                    ret.push(Self {
                        pos: Some((row, col + 1)),
                        turn,
                    });
                }
            }
        }
        ret
    }

    fn is_start(&self) -> bool {
        self.pos.is_none()
    }

    fn is_goal(&self, height: usize, width: usize) -> bool {
        self.pos == Some((height, width - 1))
    }
}

struct Map {
    blizz_at: Vec<Array2D<Vec<Blizz>>>,
    width: usize,
    height: usize,
}

impl Map {
    fn parse(inp: &str) -> Self {
        let height = inp.lines().count() - 2;
        let width = inp.lines().next().unwrap().len() - 2;
        let mut blizz_at = Array2D::filled_with(Vec::new(), height, width);
        for (row, line) in inp.lines().skip(1).take(height).enumerate() {
            for (col, c) in line.chars().skip(1).take(width).enumerate() {
                if let Some(blizz) = Blizz::parse(c) {
                    blizz_at[(row, col)].push(blizz);
                }
            }
        }
        Self {
            blizz_at: vec![blizz_at],
            width,
            height,
        }
    }

    fn safe_at(&mut self, turn: usize, row: usize, col: usize) -> bool {
        self.update(turn);
        self.blizz_at[turn][(row, col)].is_empty()
    }

    fn update(&mut self, turn: usize) {
        for _ in self.blizz_at.len()..=turn {
            let last = self.blizz_at.last().unwrap();
            let mut next = Array2D::filled_with(Vec::new(), self.height, self.width);
            for row in 0..self.height {
                for col in 0..self.width {
                    for blizz in &last[(row, col)] {
                        let (row, col) = match blizz {
                            Blizz::Up => {
                                if row == 0 {
                                    (self.height - 1, col)
                                } else {
                                    (row - 1, col)
                                }
                            }
                            Blizz::Right => {
                                if col == self.width - 1 {
                                    (row, 0)
                                } else {
                                    (row, col + 1)
                                }
                            }
                            Blizz::Down => {
                                if row == self.height - 1 {
                                    (0, col)
                                } else {
                                    (row + 1, col)
                                }
                            }
                            Blizz::Left => {
                                if col == 0 {
                                    (row, self.width - 1)
                                } else {
                                    (row, col - 1)
                                }
                            }
                        };
                        next[(row, col)].push(*blizz);
                    }
                }
            }
            self.blizz_at.push(next);
        }
    }

    #[allow(dead_code)]
    fn print(&self, state: State) {
        print!("#");
        if state.pos.is_none() {
            print!("E");
        } else {
            print!(".");
        }
        for _ in 0..self.width {
            print!("#");
        }
        println!();
        for row in 0..self.height {
            print!("#");
            for col in 0..self.width {
                let len = self.blizz_at[state.turn][(row, col)].len();
                if state.pos == Some((row, col)) {
                    assert_eq!(len, 0, "{row} {col}");
                    print!("E");
                } else {
                    match len {
                        0 => print!("."),
                        1 => print!("{}", self.blizz_at[state.turn][(row, col)][0].as_char()),
                        _ => print!("{}", len),
                    }
                }
            }
            print!("#");
            println!();
        }
        for _ in 0..self.width + 2 {
            print!("#");
        }
        println!();
    }
}

fn part1(inp: &str) -> usize {
    let mut map = Map::parse(inp);
    let initial = State { pos: None, turn: 0 };
    let height = map.height;
    let width = map.width;
    let (path, _) = dijkstra(
        &initial,
        |state| state.succ(&mut map).into_iter().map(|n| (n, 1)),
        |state| state.is_goal(height, width),
    )
    .unwrap();
    path.last().unwrap().turn
}

fn part2(inp: &str) -> usize {
    let mut map = Map::parse(inp);
    let initial = State { pos: None, turn: 0 };
    let height = map.height;
    let width = map.width;
    let (path, _) = dijkstra(
        &initial,
        |state| state.succ(&mut map).into_iter().map(|n| (n, 1)),
        |state| state.is_goal(height, width),
    )
    .unwrap();
    let (path, _) = dijkstra(
        path.last().unwrap(),
        |state| state.succ(&mut map).into_iter().map(|n| (n, 1)),
        |state| state.is_start(),
    )
    .unwrap();
    let (path, _) = dijkstra(
        path.last().unwrap(),
        |state| state.succ(&mut map).into_iter().map(|n| (n, 1)),
        |state| state.is_goal(height, width),
    )
    .unwrap();
    path.last().unwrap().turn
}

xaoc::xaoc!(
    sample = r#"#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#"#
);
