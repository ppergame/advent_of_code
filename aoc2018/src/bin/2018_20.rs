use pathfinding::prelude::*;
use std::collections::HashMap;

#[derive(Default, Debug)]
struct Room {
    n: bool,
    e: bool,
    s: bool,
    w: bool,
}

impl Room {
    fn succ(&self, row: i64, col: i64) -> impl Iterator<Item = ((i64, i64), usize)> {
        let mut ret = vec![];
        if self.n {
            ret.push((row - 1, col));
        }
        if self.e {
            ret.push((row, col + 1));
        }
        if self.s {
            ret.push((row + 1, col));
        }
        if self.w {
            ret.push((row, col - 1));
        }
        ret.into_iter().map(|i| (i, 1))
    }
}

fn paths(inp: &str) -> HashMap<(i64, i64), ((i64, i64), usize)> {
    let mut cc = inp.chars();
    assert_eq!(cc.next(), Some('^'));
    let mut stack = vec![];
    let mut row = 0;
    let mut col = 0;
    let mut map = HashMap::<(i64, i64), Room>::new();
    for c in cc {
        match c {
            '$' => break,
            'N' => {
                map.entry((row, col)).or_default().n = true;
                row -= 1;
                map.entry((row, col)).or_default().s = true;
            }
            'E' => {
                map.entry((row, col)).or_default().e = true;
                col += 1;
                map.entry((row, col)).or_default().w = true;
            }
            'S' => {
                map.entry((row, col)).or_default().s = true;
                row += 1;
                map.entry((row, col)).or_default().n = true;
            }
            'W' => {
                map.entry((row, col)).or_default().w = true;
                col -= 1;
                map.entry((row, col)).or_default().e = true;
            }
            '(' => {
                stack.push((row, col));
            }
            '|' => {
                (row, col) = *stack.last().unwrap();
            }
            ')' => {
                (row, col) = stack.pop().unwrap();
            }
            _ => unreachable!(),
        }
    }
    dijkstra_all(&(0, 0), |&(row, col)| map[&(row, col)].succ(row, col))
}

fn part1(inp: &str) -> usize {
    paths(inp)
        .into_iter()
        .map(|(_, (_, cost))| cost)
        .max()
        .unwrap()
}

fn part2(inp: &str) -> usize {
    paths(inp)
        .into_iter()
        .filter(|(_, (_, cost))| *cost >= 1000)
        .count()
}

xaoc::xaoc!(sample = "^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$");
