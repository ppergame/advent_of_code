use bimap::BiHashMap;
use std::collections::HashMap;

fn part1(inp: &str) -> i64 {
    let mm = [('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]
        .into_iter()
        .collect::<BiHashMap<_, _>>();
    let pp = [(')', 3), (']', 57), ('}', 1197), ('>', 25137)]
        .into_iter()
        .collect::<HashMap<_, _>>();
    let mut score = 0;
    for line in inp.lines() {
        let mut stack = vec![];
        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                ')' | ']' | '}' | '>' => {
                    let last = stack.pop().unwrap();
                    if last != *mm.get_by_right(&c).unwrap() {
                        score += pp.get(&c).unwrap();
                        break;
                    }
                }
                _ => unreachable!(),
            }
        }
    }
    score
}

fn part2(inp: &str) -> i64 {
    let mm = [('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]
        .into_iter()
        .collect::<BiHashMap<_, _>>();
    let pp = [(')', 1), (']', 2), ('}', 3), ('>', 4)]
        .into_iter()
        .collect::<HashMap<_, _>>();
    let mut scores = vec![];
    'outer: for line in inp.lines() {
        let mut score = 0;
        let mut stack = vec![];
        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                ')' | ']' | '}' | '>' => {
                    let last = stack.pop().unwrap();
                    if last != *mm.get_by_right(&c).unwrap() {
                        continue 'outer;
                    }
                }
                _ => unreachable!(),
            }
        }
        for c in stack.into_iter().rev() {
            score = score * 5 + pp[mm.get_by_left(&c).unwrap()];
        }
        scores.push(score);
    }
    scores.sort_unstable();
    scores[scores.len() / 2]
}

xaoc::xaoc!();
