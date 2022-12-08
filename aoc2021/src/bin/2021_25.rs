use std::collections::HashMap;

#[derive(Debug, Clone, Eq, PartialEq)]
enum Cell {
    East,
    South,
}

type Point = (i64, i64);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Input {
    map: HashMap<Point, Cell>,
    width: i64,
    height: i64,
}

impl Input {
    fn step(&self) -> Input {
        let mut map = HashMap::new();
        for ((x, y), cell) in &self.map {
            match cell {
                Cell::South => {
                    map.insert((*x, *y), Cell::South);
                }
                Cell::East => {
                    let new_x = (x + 1) % self.width;
                    if !self.map.contains_key(&(new_x, *y)) {
                        map.insert((new_x, *y), Cell::East);
                    } else {
                        map.insert((*x, *y), Cell::East);
                    }
                }
            }
        }
        let mut map2 = HashMap::new();
        for ((x, y), cell) in &map {
            match cell {
                Cell::East => {
                    map2.insert((*x, *y), Cell::East);
                }
                Cell::South => {
                    let new_y = (y + 1) % self.height;
                    if !map.contains_key(&(*x, new_y)) {
                        map2.insert((*x, new_y), Cell::South);
                    } else {
                        map2.insert((*x, *y), Cell::South);
                    }
                }
            }
        }
        Input {
            map: map2,
            width: self.width,
            height: self.height,
        }
    }

    #[allow(dead_code)]
    fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let c = match self.map.get(&(x, y)) {
                    Some(Cell::East) => '>',
                    Some(Cell::South) => 'v',
                    None => '.',
                };
                print!("{}", c);
            }
            println!();
        }
        println!();
    }
}

fn parse(inp: &str) -> Input {
    let mut map = HashMap::new();
    let mut width = 0;
    let mut height = 0;
    for (y, line) in inp.lines().enumerate() {
        height = height.max(y);
        for (x, c) in line.chars().enumerate() {
            width = width.max(x);
            match c {
                '.' => (),
                '>' => {
                    map.insert((x as i64, y as i64), Cell::East);
                }
                'v' => {
                    map.insert((x as i64, y as i64), Cell::South);
                }
                _ => unreachable!(),
            }
        }
    }
    Input {
        map,
        width: (width + 1) as i64,
        height: (height + 1) as i64,
    }
}

fn part1(inp: &str) -> i64 {
    let mut prev = parse(inp);
    let mut i = 0;
    loop {
        //println!("{}:", i);
        //prev.print();
        let next = prev.step();
        if next == prev {
            break;
        }
        prev = next;
        i += 1;
    }
    i + 1
}

fn part2(_inp: &str) -> i64 {
    0
}

xaoc::xaoc!();
