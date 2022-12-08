use aoc2019::intcode::*;
use std::collections::HashMap;
use std::fmt::Write;

struct Robot {
    x: i64,
    y: i64,
    diri: usize,
    colors: HashMap<(i64, i64), i64>,
}

static DIRS: &[(i64, i64)] = &[
    // up
    (0, -1),
    // right
    (1, 0),
    // down
    (0, 1),
    // left
    (-1, 0),
];

impl Robot {
    fn step(&mut self, ic: &mut Intcode) -> bool {
        match ic.run().unwrap() {
            IntcodeStatus::Input => (),
            IntcodeStatus::Halt => return false,
            _ => panic!("unexpected status"),
        };
        ic.input = Some(*self.colors.get(&(self.x, self.y)).unwrap_or(&0));
        let color = match ic.run().unwrap() {
            IntcodeStatus::Output(output) => output,
            _ => panic!("needed output"),
        };
        match color {
            0 => (),
            1 => (),
            _ => panic!("bad color"),
        };
        self.colors.insert((self.x, self.y), color);
        let newdir = match ic.run().unwrap() {
            IntcodeStatus::Output(output) => output,
            _ => panic!("needed output"),
        };
        self.diri = match newdir {
            0 => self.diri - 1,
            1 => self.diri + 1,
            _ => panic!("bad newdir"),
        } % DIRS.len();
        let (dx, dy) = DIRS[self.diri];
        self.x += dx;
        self.y += dy;
        true
    }
}

fn part1(inp: &str) -> usize {
    let mut ic = Intcode::new(inp);
    let mut r = Robot {
        x: 0,
        y: 0,
        diri: 0,
        colors: HashMap::new(),
    };
    while r.step(&mut ic) {}
    r.colors.len()
}

fn part2(inp: &str) -> String {
    let mut ret = String::new();
    let mut ic = Intcode::new(inp);
    let mut r = Robot {
        x: 0,
        y: 0,
        diri: 0,
        colors: HashMap::new(),
    };
    r.colors.insert((0, 0), 1);
    while r.step(&mut ic) {}
    let minx = r.colors.keys().min_by_key(|(x, _)| x).unwrap().0;
    let maxx = r.colors.keys().max_by_key(|(x, _)| x).unwrap().0;
    let miny = r.colors.keys().min_by_key(|(_, y)| y).unwrap().1;
    let maxy = r.colors.keys().max_by_key(|(_, y)| y).unwrap().1;
    for row in miny..=maxy {
        for col in minx..=maxx {
            write!(
                &mut ret,
                "{}",
                match *r.colors.get(&(col, row)).unwrap_or(&0) {
                    0 => " ",
                    1 => "1",
                    _ => panic!("bruh"),
                }
            )
            .unwrap();
        }
        writeln!(&mut ret).unwrap();
    }
    ret
}

xaoc::xaoc!();
