use itertools::{Itertools, MinMaxResult};
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Track {
    Vert,
    Horiz,
    Cross,
    CornerR,
    CornerL,
}

impl Track {
    fn parse(c: char) -> Self {
        match c {
            '|' => Track::Vert,
            '-' => Track::Horiz,
            '+' => Track::Cross,
            '/' => Track::CornerR,
            '\\' => Track::CornerL,
            _ => unreachable!("bad track char {c:?}"),
        }
    }

    fn as_char(&self) -> char {
        match self {
            Track::Vert => '|',
            Track::Horiz => '-',
            Track::Cross => '+',
            Track::CornerR => '/',
            Track::CornerL => '\\',
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Dir {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

impl Dir {
    fn from_cart(c: char) -> Option<Self> {
        match c {
            '^' => Some(Dir::Up),
            '>' => Some(Dir::Right),
            'v' => Some(Dir::Down),
            '<' => Some(Dir::Left),
            _ => None,
        }
    }

    fn as_char(&self) -> char {
        match self {
            Dir::Up => '^',
            Dir::Right => '>',
            Dir::Down => 'v',
            Dir::Left => '<',
        }
    }

    fn as_track(&self) -> Track {
        match self {
            Dir::Up | Dir::Down => Track::Vert,
            Dir::Left | Dir::Right => Track::Horiz,
        }
    }

    fn delta(&self) -> (i64, i64) {
        match self {
            Dir::Up => (-1, 0),
            Dir::Right => (0, 1),
            Dir::Down => (1, 0),
            Dir::Left => (0, -1),
        }
    }

    fn prev(&self) -> Self {
        match self {
            Dir::Up => Dir::Left,
            Dir::Right => Dir::Up,
            Dir::Down => Dir::Right,
            Dir::Left => Dir::Down,
        }
    }

    fn next(&self) -> Self {
        match self {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
        }
    }

    fn turn(&self, turn: Turn) -> Self {
        match turn {
            Turn::Left => self.prev(),
            Turn::Straight => *self,
            Turn::Right => self.next(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Turn {
    Left = -1,
    Straight = 0,
    Right = 1,
}

impl Turn {
    fn next(&self) -> Self {
        match self {
            Turn::Left => Turn::Straight,
            Turn::Straight => Turn::Right,
            Turn::Right => Turn::Left,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Cart {
    dir: Dir,
    next_turn: Turn,
}

impl Cart {
    fn turn(&mut self, track: Track) {
        match track {
            Track::Vert => (),
            Track::Horiz => (),
            Track::Cross => {
                self.dir = self.dir.turn(self.next_turn);
                self.next_turn = self.next_turn.next();
            }
            Track::CornerR => {
                self.dir = match self.dir {
                    Dir::Up => Dir::Right,
                    Dir::Right => Dir::Up,
                    Dir::Down => Dir::Left,
                    Dir::Left => Dir::Down,
                };
            }
            Track::CornerL => {
                self.dir = match self.dir {
                    Dir::Up => Dir::Left,
                    Dir::Right => Dir::Down,
                    Dir::Down => Dir::Right,
                    Dir::Left => Dir::Up,
                };
            }
        }
    }
}

struct Map {
    map: HashMap<(i64, i64), Track>,
    carts: HashMap<(i64, i64), Cart>,
    first_crash: Option<(i64, i64)>,
}

impl Map {
    fn parse(inp: &str) -> Self {
        let mut map = HashMap::new();
        let mut carts = HashMap::new();
        for (row, line) in inp.lines().enumerate() {
            let row = row as i64;
            for (col, c) in line.chars().enumerate() {
                let col = col as i64;
                if c == ' ' {
                    continue;
                }
                let track = match Dir::from_cart(c) {
                    None => Track::parse(c),
                    Some(dir) => {
                        carts.insert(
                            (row, col),
                            Cart {
                                dir,
                                next_turn: Turn::Left,
                            },
                        );
                        dir.as_track()
                    }
                };
                map.insert((row, col), track);
            }
        }
        Map {
            map,
            carts,
            first_crash: None,
        }
    }

    fn step(&mut self) {
        for ((row, col), mut cart) in self
            .carts
            .clone()
            .into_iter()
            .sorted_by_key(|((row, col), _)| (*row, *col))
        {
            match self.carts.entry((row, col)) {
                std::collections::hash_map::Entry::Occupied(o) => o.remove(),
                std::collections::hash_map::Entry::Vacant(_) => continue,
            };
            let (dr, dc) = cart.dir.delta();
            let row = row + dr;
            let col = col + dc;

            match self.carts.entry((row, col)) {
                std::collections::hash_map::Entry::Occupied(o) => {
                    o.remove();
                    if self.first_crash.is_none() {
                        self.first_crash = Some((row, col));
                    }
                }
                std::collections::hash_map::Entry::Vacant(_) => {
                    cart.turn(self.map[&(row, col)]);
                    self.carts.insert((row, col), cart);
                }
            }
        }
    }

    #[allow(dead_code)]
    fn print(&self) {
        let MinMaxResult::MinMax(minr, maxr) = self.map.keys().map(|(row, _)| row).copied().minmax() else { unreachable!() };
        let MinMaxResult::MinMax(minc, maxc) = self.map.keys().map(|(_, col)| col).copied().minmax() else { unreachable!() };
        for row in minr..=maxr {
            for col in minc..=maxc {
                let c = match self.carts.get(&(row, col)) {
                    Some(cart) => cart.dir.as_char(),
                    None => match self.map.get(&(row, col)) {
                        Some(track) => track.as_char(),
                        None => ' ',
                    },
                };
                print!("{}", c);
            }
            println!();
        }
    }
}

fn part1(inp: &str) -> String {
    let mut map = Map::parse(inp);
    loop {
        if let Some((row, col)) = map.first_crash {
            return format!("{col},{row}");
        }
        map.step();
    }
}

fn part2(inp: &str) -> String {
    let mut map = Map::parse(inp);
    loop {
        if map.carts.len() == 1 {
            let (row, col) = map.carts.keys().next().unwrap();
            return format!("{col},{row}");
        }
        map.step();
    }
}

xaoc::xaoc!(
    sample = r#"/->-\        
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/"#,
    sample2 = r#"/>-<\  
|   |  
| /<+-\
| | | v
\>+</ |
  |   ^
  \<->/"#
);
