use std::collections::HashSet;

type Point = (i64, i64);

pub struct Input {
    map: HashSet<Point>,
    folds: Vec<(char, i64)>,
}

impl Input {
    fn fold(&mut self) -> bool {
        if self.folds.is_empty() {
            return false;
        }
        let (c, n) = self.folds.remove(0);
        let mut newmap = HashSet::new();
        match c {
            'x' => {
                for &(x, y) in &self.map {
                    match x.cmp(&n) {
                        std::cmp::Ordering::Less => {
                            newmap.insert((x, y));
                        }
                        std::cmp::Ordering::Equal => (),
                        std::cmp::Ordering::Greater => {
                            newmap.insert((n * 2 - x, y));
                        }
                    }
                }
            }
            'y' => {
                for &(x, y) in &self.map {
                    match y.cmp(&n) {
                        std::cmp::Ordering::Less => {
                            newmap.insert((x, y));
                        }
                        std::cmp::Ordering::Equal => (),
                        std::cmp::Ordering::Greater => {
                            newmap.insert((x, n * 2 - y));
                        }
                    }
                }
            }
            _ => unreachable!(),
        }
        self.map = newmap;
        true
    }
}

fn parse(inp: &str) -> Input {
    let (points, folds) = inp.split_once("\n\n").unwrap();
    let map = points
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect::<HashSet<_>>();
    let folds = folds
        .lines()
        .map(|line| {
            let t = line.split_whitespace().last().unwrap();
            let (c, n) = t.split_once('=').unwrap();
            (c.chars().next().unwrap(), n.parse().unwrap())
        })
        .collect::<Vec<_>>();
    Input { map, folds }
}

fn part1(inp: &str) -> usize {
    let mut inp = parse(inp);
    inp.fold();
    inp.map.len()
}

fn part2(inp: &str) -> String {
    let mut inp = parse(inp);
    while inp.fold() {}
    let maxx = *inp.map.iter().map(|(x, _)| x).max().unwrap();
    let maxy = *inp.map.iter().map(|(_, y)| y).max().unwrap();
    let mut s = "\n".to_owned();
    for y in 0..=maxy {
        for x in 0..=maxx {
            s.push(if inp.map.contains(&(x, y)) { '#' } else { ' ' });
        }
        s.push('\n');
    }
    s
}

xaoc::xaoc!();
