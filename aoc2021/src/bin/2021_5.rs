use std::collections::HashMap;

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
pub struct Point(i64, i64);

#[derive(Debug)]
pub struct Line {
    start: Point,
    end: Point,
}

fn parse(inp: &str) -> Vec<Line> {
    inp.lines()
        .map(|line| {
            let sp = line.split_whitespace().collect::<Vec<_>>();
            let start = sp[0].split_once(',').unwrap();
            let end = sp[2].split_once(',').unwrap();
            Line {
                start: Point(start.0.parse().unwrap(), start.1.parse().unwrap()),
                end: Point(end.0.parse().unwrap(), end.1.parse().unwrap()),
            }
        })
        .collect()
}

fn part1(inp: &str) -> usize {
    let lines = parse(inp);
    let mut map = HashMap::<Point, i64>::new();
    for line in lines {
        if line.start.0 == line.end.0 {
            let x = line.start.0;
            for y in
                std::cmp::min(line.start.1, line.end.1)..=std::cmp::max(line.start.1, line.end.1)
            {
                *map.entry(Point(x, y)).or_insert(0) += 1;
            }
        } else if line.start.1 == line.end.1 {
            let y = line.start.1;
            for x in
                std::cmp::min(line.start.0, line.end.0)..=std::cmp::max(line.end.0, line.start.0)
            {
                *map.entry(Point(x, y)).or_insert(0) += 1;
            }
        }
    }
    map.values().filter(|v| **v >= 2).count()
}

fn part2(inp: &str) -> usize {
    let lines = parse(inp);
    let mut map = HashMap::<Point, i64>::new();
    for line in lines {
        if line.start.0 == line.end.0 {
            let x = line.start.0;
            for y in
                std::cmp::min(line.start.1, line.end.1)..=std::cmp::max(line.start.1, line.end.1)
            {
                *map.entry(Point(x, y)).or_insert(0) += 1;
            }
        } else if line.start.1 == line.end.1 {
            let y = line.start.1;
            for x in
                std::cmp::min(line.start.0, line.end.0)..=std::cmp::max(line.end.0, line.start.0)
            {
                *map.entry(Point(x, y)).or_insert(0) += 1;
            }
        } else {
            let dx = if line.start.0 > line.end.0 { -1 } else { 1 };
            let dy = if line.start.1 > line.end.1 { -1 } else { 1 };
            let Point(mut x, mut y) = line.start;
            let Point(tx, ty) = line.end;
            while x != tx && y != ty {
                *map.entry(Point(x, y)).or_insert(0) += 1;
                x += dx;
                y += dy;
            }
            *map.entry(Point(x, y)).or_insert(0) += 1;
        }
    }
    map.values().filter(|v| **v >= 2).count()
}

xaoc::xaoc!();
