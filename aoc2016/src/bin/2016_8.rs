use sscanf::scanf;
use std::collections::HashSet;

const WIDTH: usize = 50;
const HEIGHT: usize = 6;

fn calc(inp: &str) -> HashSet<(usize, usize)> {
    let mut pix = HashSet::new();
    for line in inp.lines() {
        if let Ok((a, b)) = scanf!(line, "rect {}x{}", usize, usize) {
            for x in 0..a {
                for y in 0..b {
                    pix.insert((x, y));
                }
            }
        } else if let Ok((a, b)) = scanf!(line, "rotate row y={} by {}", usize, usize) {
            let mut row = (0..WIDTH)
                .map(|x| pix.contains(&(x, a)))
                .collect::<Vec<_>>();
            row.rotate_right(b);
            for (x, val) in row.into_iter().enumerate() {
                if val {
                    pix.insert((x, a));
                } else {
                    pix.remove(&(x, a));
                }
            }
        } else if let Ok((a, b)) = scanf!(line, "rotate column x={} by {}", usize, usize) {
            let mut col = (0..HEIGHT)
                .map(|y| pix.contains(&(a, y)))
                .collect::<Vec<_>>();
            col.rotate_right(b);
            for (y, val) in col.into_iter().enumerate() {
                if val {
                    pix.insert((a, y));
                } else {
                    pix.remove(&(a, y));
                }
            }
        } else {
            unreachable!();
        }
    }
    pix
}

fn part1(inp: &str) -> usize {
    calc(inp).len()
}

fn part2(inp: &str) -> String {
    let pix = calc(inp);
    let mut acc = String::new();
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if pix.contains(&(x, y)) {
                acc.push('#');
            } else {
                acc.push(' ');
            }
        }
        acc.push('\n');
    }
    acc
}

xaoc::xaoc!();
