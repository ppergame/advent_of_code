use ndarray::{Array2, ArrayView, Axis};

#[derive(Default, Copy, Clone, PartialEq, Eq)]
enum State {
    #[default]
    Empty,
    Stop,
    Roll,
}

impl State {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '#' => Self::Stop,
            'O' => Self::Roll,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
struct Map {
    a: Array2<State>,
}

impl Map {
    fn parse(inp: &str) -> Self {
        let width = inp.lines().next().unwrap().chars().count();
        let mut a = Array2::default((0, width));
        for line in inp.lines() {
            let mut row = vec![];
            for c in line.chars() {
                row.push(State::from_char(c));
            }
            a.append(
                Axis(0),
                ArrayView::from(&row).into_shape((1, width)).unwrap(),
            )
            .unwrap();
        }
        Self { a }
    }

    fn roll(&mut self) {
        for row in 1..self.a.nrows() {
            for col in 0..self.a.ncols() {
                if self.a[(row, col)] == State::Roll {
                    for new_row in (0..=row).rev() {
                        if new_row == 0 || self.a[(new_row - 1, col)] != State::Empty {
                            self.a[(row, col)] = State::Empty;
                            self.a[(new_row, col)] = State::Roll;
                            break;
                        }
                    }
                }
            }
        }
    }

    fn score(&self) -> i64 {
        let mut score = 0;
        for ((row, _), val) in self.a.indexed_iter() {
            if *val == State::Roll {
                score += self.a.nrows() - row;
            }
        }
        score as i64
    }

    #[allow(dead_code)]
    fn print(&self) {
        for ((_, col), val) in self.a.indexed_iter() {
            let c = match val {
                State::Empty => '.',
                State::Stop => '#',
                State::Roll => 'O',
            };
            print!("{c}");
            if col == self.a.ncols() - 1 {
                println!();
            }
        }
        println!();
    }

    fn rot(&mut self) {
        let mut v = self.a.t();
        v.invert_axis(Axis(1));
        self.a = v.to_owned();
    }

    fn cycle(&mut self) {
        for _ in 0..4 {
            self.roll();
            self.rot();
        }
    }
}

fn part1(inp: &str) -> i64 {
    let mut map = Map::parse(inp);
    map.roll();
    map.score()
}

fn part2(inp: &str) -> i64 {
    let mut map1 = Map::parse(inp);
    let mut map2 = map1.clone();
    const GOAL: i64 = 1000000000;
    let mut c1 = 0;
    map2.cycle();
    let mut c2 = 1;
    while c1 < GOAL {
        if map1 == map2 {
            let len = c2 - c1;
            c1 += (GOAL - c1) / len * len;
            break;
        }
        map1.cycle();
        c1 += 1;
        map2.cycle();
        map2.cycle();
        c2 += 2;
    }
    while c1 < GOAL {
        map1.cycle();
        c1 += 1;
    }
    map1.score()
}

xaoc::xaoc!(
    sample = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."
);
