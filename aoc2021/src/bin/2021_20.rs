use std::collections::HashMap;
use std::ops::RangeInclusive;

type Point = (i64, i64);

#[derive(Debug, Clone)]
pub struct Input {
    algo: Vec<bool>,
    img: HashMap<Point, bool>,
}

struct Board {
    img: HashMap<Point, bool>,
    default: bool,
}

impl Board {
    fn bounds(&self) -> (RangeInclusive<i64>, RangeInclusive<i64>) {
        let minx = *self.img.keys().map(|(x, _)| x).min().unwrap();
        let maxx = *self.img.keys().map(|(x, _)| x).max().unwrap();
        let miny = *self.img.keys().map(|(_, y)| y).min().unwrap();
        let maxy = *self.img.keys().map(|(_, y)| y).max().unwrap();

        (minx..=maxx, miny..=maxy)
    }

    fn transform(&self, algo: &[bool]) -> Board {
        let (xr, yr) = self.bounds();

        let mut ret = HashMap::new();
        for y in yr.start() - 1..=yr.end() + 1 {
            for x in xr.start() - 1..=xr.end() + 1 {
                let num = self.p_to_num((x, y));
                ret.insert((x, y), algo[num]);
            }
        }
        let mut default = self.default;
        if default && !algo[511] {
            default = false;
        } else if !default && algo[0] {
            default = true;
        }
        Board { img: ret, default }
    }

    fn p_to_num(&self, p: Point) -> usize {
        let (x, y) = p;
        let mut num = 0;
        for dy in -1..=1 {
            for dx in -1..=1 {
                num *= 2;
                if *self.img.get(&(x + dx, y + dy)).unwrap_or(&self.default) {
                    num += 1;
                }
            }
        }
        num
    }

    #[allow(dead_code)]
    fn print(&self) {
        let (xr, yr) = self.bounds();
        for y in yr {
            for x in xr.clone() {
                print!(
                    "{}",
                    if *self.img.get(&(x, y)).unwrap_or(&self.default) {
                        '#'
                    } else {
                        '.'
                    },
                );
            }
            println!();
        }
        println!();
    }
}

fn parse(inp: &str) -> Input {
    let (algo, img_s) = inp.split_once("\n\n").unwrap();
    let algo = algo.chars().map(|p| p == '#').collect::<Vec<_>>();
    let mut img = HashMap::new();
    for (y, line) in img_s.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            img.insert(
                (x as i64, y as i64),
                match c {
                    '#' => true,
                    '.' => false,
                    _ => unreachable!(),
                },
            );
        }
    }
    assert_eq!(algo.len(), 512);
    Input { algo, img }
}

fn part1(inp: &str) -> usize {
    let inp = parse(inp);
    let b = Board {
        img: inp.img.clone(),
        default: false,
    };
    let b = b.transform(&inp.algo);
    let b = b.transform(&inp.algo);
    b.img.values().filter(|b| **b).count()
}

fn part2(inp: &str) -> usize {
    let inp = parse(inp);
    let mut b = Board {
        img: inp.img.clone(),
        default: false,
    };
    for _ in 0..50 {
        b = b.transform(&inp.algo);
    }
    b.img.values().filter(|b| **b).count()
}

xaoc::xaoc!();
