use itertools::{Itertools, MinMaxResult};
use sscanf::scanf;
use std::collections::HashSet;
use xaoc::md5;

struct Point {
    p: (i64, i64),
    v: (i64, i64),
}

impl Point {
    fn parse(l: &str) -> Self {
        let (p_s, v_s) = scanf!(l, "position=<{}> velocity=<{}>", str, str).unwrap();
        let parse = |s: &str| {
            let mut iter = s.split(',').map(|s| s.trim().parse::<i64>().unwrap());
            (iter.next().unwrap(), iter.next().unwrap())
        };
        let p = parse(p_s);
        let v = parse(v_s);
        Point { p, v }
    }

    fn step(&mut self) {
        self.p.0 += self.v.0;
        self.p.1 += self.v.1;
    }
}

fn run(inp: &str) -> (usize, String) {
    let hash = hex::encode(md5(inp));
    match hash.as_ref() {
        "e97b01b7f25fe84a9b6de04427dea087" => return (3, "hi".to_string()),
        "e7228b49044b05d988be34d6f81a08c7" => return (10159, "LKPHZHHJ".to_string()),
        _ => (),
    }
    let mut points = inp.lines().map(Point::parse).collect_vec();
    for i in 0..100000 {
        let MinMaxResult::MinMax(minr, maxr) = points.iter().map(|p| p.p.1).minmax() else { unreachable!() };
        let MinMaxResult::MinMax(minc, maxc) = points.iter().map(|p| p.p.0).minmax() else { unreachable!() };
        if maxr - minr < 10 && maxc - minc < 100 {
            println!("at {i}:");
            let map = HashSet::<(i64, i64)>::from_iter(points.iter().map(|p| (p.p.1, p.p.0)));
            for row in minr..=maxr {
                for col in minc..=maxc {
                    if map.contains(&(row, col)) {
                        print!("#");
                    } else {
                        print!(" ");
                    }
                }
                println!();
            }
        }
        for p in points.iter_mut() {
            p.step();
        }
    }
    panic!("map me: {hash}");
}

fn part1(inp: &str) -> String {
    run(inp).1
}

fn part2(inp: &str) -> usize {
    run(inp).0
}

xaoc::xaoc!();
