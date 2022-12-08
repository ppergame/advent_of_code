use rayon::prelude::*;
use regex::Regex;
use std::ops::RangeInclusive;

type Point = (i64, i64);

#[derive(Debug)]
pub struct Input {
    xr: RangeInclusive<i64>,
    yr: RangeInclusive<i64>,
}

fn parse(inp: &str) -> Input {
    let re = Regex::new(r"target area: x=(-?\d+)..(-?\d+), y=(-?\d+)..(-?\d+)").unwrap();
    let cap = re.captures(inp).unwrap();
    Input {
        xr: cap[1].parse().unwrap()..=cap[2].parse().unwrap(),
        yr: cap[3].parse().unwrap()..=cap[4].parse().unwrap(),
    }
}

fn traj_past(mut vx: i64, mut vy: i64, tx: i64, ty: i64) -> Vec<Point> {
    let (mut x, mut y) = (0, 0);
    let mut ret = vec![];
    while x <= tx && y >= ty {
        ret.push((x, y));
        x += vx;
        y += vy;
        vx = match vx.cmp(&0) {
            std::cmp::Ordering::Less => vx + 1,
            std::cmp::Ordering::Equal => vx,
            std::cmp::Ordering::Greater => vx - 1,
        };
        vy -= 1;
    }
    ret
}

fn part1(inp: &str) -> i64 {
    let inp = parse(inp);
    let mut maxy = 0;
    for vx in 1..1000 {
        for vy in 1..1000 {
            let traj = traj_past(vx, vy, *inp.xr.end(), *inp.yr.start());
            if !traj
                .iter()
                .any(|(x, y)| inp.xr.contains(x) && inp.yr.contains(y))
            {
                continue;
            }
            maxy = maxy.max(traj.into_iter().map(|(_, y)| y).max().unwrap());
        }
    }
    maxy
}

fn part2(inp: &str) -> i64 {
    let inp = parse(inp);
    (-1000..1000)
        .into_par_iter()
        .map(|vx| {
            (-1000..1000)
                .into_par_iter()
                .map(|vy| {
                    let traj = traj_past(vx, vy, *inp.xr.end(), *inp.yr.start());
                    traj.into_iter()
                        .any(|(x, y)| inp.xr.contains(&x) && inp.yr.contains(&y))
                        as i64
                })
                .sum::<i64>()
        })
        .sum()
}

xaoc::xaoc!();
