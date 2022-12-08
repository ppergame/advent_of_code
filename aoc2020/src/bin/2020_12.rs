struct Coord(i32, i32);

#[derive(Clone, Copy)]
enum Dir {
    N,
    S,
    W,
    E,
}

impl Dir {
    fn dxy(self) -> (i32, i32) {
        match self {
            Dir::N => (0, -1),
            Dir::S => (0, 1),
            Dir::W => (-1, 0),
            Dir::E => (1, 0),
        }
    }

    fn turn(self, deg: i32) -> Dir {
        let mut new_dir = self;
        assert_eq!(deg % 90, 0);
        let turns = (deg / 90).rem_euclid(4);
        for _ in 0..turns {
            new_dir = match new_dir {
                Dir::N => Dir::E,
                Dir::S => Dir::W,
                Dir::W => Dir::N,
                Dir::E => Dir::S,
            };
        }
        new_dir
    }
}

struct Ship {
    pos: Coord,
    dwp: (i32, i32),
    heading: Dir,
}

impl Ship {
    fn new() -> Ship {
        Ship {
            pos: Coord(0, 0),
            dwp: (10, -1), // (1, 10), (-10, 1), (-1, -10)
            heading: Dir::E,
        }
    }

    fn apply(&mut self, action: Action) {
        match action {
            Action::N(arg) => self.pos.1 -= arg,
            Action::S(arg) => self.pos.1 += arg,
            Action::W(arg) => self.pos.0 -= arg,
            Action::E(arg) => self.pos.0 += arg,
            Action::L(arg) => self.heading = self.heading.turn(-arg),
            Action::R(arg) => self.heading = self.heading.turn(arg),
            Action::F(arg) => {
                let (dx, dy) = self.heading.dxy();
                let Coord(x, y) = self.pos;
                self.pos = Coord(x + dx * arg, y + dy * arg);
            }
        };
    }

    fn apply2(&mut self, action: Action) {
        match action {
            Action::N(arg) => self.dwp.1 -= arg,
            Action::S(arg) => self.dwp.1 += arg,
            Action::W(arg) => self.dwp.0 -= arg,
            Action::E(arg) => self.dwp.0 += arg,
            Action::L(arg) => self.turn_wp(-arg),
            Action::R(arg) => self.turn_wp(arg),
            Action::F(arg) => {
                let (dx, dy) = self.dwp;
                let Coord(x, y) = self.pos;
                self.pos = Coord(x + dx * arg, y + dy * arg);
            }
        };
    }

    fn turn_wp(&mut self, deg: i32) {
        assert_eq!(deg % 90, 0);
        let turns = (deg / 90).rem_euclid(4);
        for _ in 0..turns {
            let (x, y) = self.dwp;
            self.dwp.0 = -y;
            self.dwp.1 = x;
        }
    }
}

enum Action {
    N(i32),
    S(i32),
    W(i32),
    E(i32),
    L(i32),
    R(i32),
    F(i32),
}

impl Action {
    fn parse(s: &str) -> Action {
        let (act, arg) = s.split_at(1);
        let arg: i32 = arg.parse().unwrap();
        match act {
            "N" => Action::N(arg),
            "S" => Action::S(arg),
            "E" => Action::E(arg),
            "W" => Action::W(arg),
            "L" => Action::L(arg),
            "R" => Action::R(arg),
            "F" => Action::F(arg),
            _ => panic!("bad act letter"),
        }
    }
}

fn parse_actions(inp: &str) -> Vec<Action> {
    inp.lines().map(Action::parse).collect()
}

fn part1(inp: &str) -> i32 {
    let mut ship = Ship::new();
    let actions = parse_actions(inp);
    for action in actions {
        ship.apply(action);
    }
    let Coord(x, y) = ship.pos;
    x.abs() + y.abs()
}

fn part2(inp: &str) -> i32 {
    let mut ship = Ship::new();
    let actions = parse_actions(inp);
    for action in actions {
        ship.apply2(action);
    }
    let Coord(x, y) = ship.pos;
    x.abs() + y.abs()
}

xaoc::xaoc!();
