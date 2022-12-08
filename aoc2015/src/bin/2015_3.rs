use std::collections::HashSet;

fn part1(inp: &str) -> usize {
    let (mut x, mut y) = (0, 0);
    let mut visited = HashSet::new();
    visited.insert((x, y));
    for c in inp.chars() {
        let (dx, dy) = match c {
            '^' => (0, -1),
            'v' => (0, 1),
            '>' => (1, 0),
            '<' => (-1, 0),
            _ => unreachable!(),
        };
        x += dx;
        y += dy;
        visited.insert((x, y));
    }
    visited.len()
}

fn part2(inp: &str) -> usize {
    let mut to_move = 1;
    let (mut x1, mut y1) = (0, 0);
    let (mut x2, mut y2) = (0, 0);
    let mut visited = HashSet::new();
    visited.insert((x1, y1));
    for c in inp.chars() {
        let (dx, dy) = match c {
            '^' => (0, -1),
            'v' => (0, 1),
            '>' => (1, 0),
            '<' => (-1, 0),
            _ => unreachable!(),
        };
        if to_move == 1 {
            x1 += dx;
            y1 += dy;
            to_move = 2;
        } else {
            x2 += dx;
            y2 += dy;
            to_move = 1;
        }
        visited.insert((x1, y1));
        visited.insert((x2, y2));
    }
    visited.len()
}

xaoc::xaoc!();
