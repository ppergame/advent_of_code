use itertools::Itertools;
use sscanf::scanf;
use std::collections::{HashMap, VecDeque};

fn tbs(s: &str) -> VecDeque<bool> {
    s.chars()
        .map(|c| match c {
            '#' => true,
            '.' => false,
            _ => unreachable!(),
        })
        .collect()
}

fn part1(inp: &str) -> i64 {
    let mut li = inp.lines();
    let state = scanf!(li.next().unwrap(), "initial state: {}", str).unwrap();
    let mut pots = tbs(state);
    let mut left: i64 = 0;
    li.next();
    let mut rules = HashMap::<_, bool>::new();
    for line in li {
        let (left, right) = scanf!(line, "{} => {}", str, char).unwrap();
        let v = tbs(left);
        rules.insert((v[0], v[1], v[2], v[3], v[4]), right == '#');
    }
    for _gen in 0..20 {
        for _ in 0..3 {
            pots.push_front(false);
            pots.push_back(false);
        }
        pots = pots
            .into_iter()
            .tuple_windows()
            .map(|w| *rules.get(&w).unwrap_or(&false))
            .collect();
        left -= 1;
    }
    pots.into_iter()
        .enumerate()
        .filter_map(|(idx, p)| if p { Some(idx as i64 + left) } else { None })
        .sum()
}

fn part2(inp: &str) -> i64 {
    let mut li = inp.lines();
    let state = scanf!(li.next().unwrap(), "initial state: {}", str).unwrap();
    let mut pots = tbs(state);
    let mut left: i64 = 0;
    li.next();
    let mut rules = HashMap::<_, bool>::new();
    for line in li {
        let (left, right) = scanf!(line, "{} => {}", str, char).unwrap();
        let v = tbs(left);
        rules.insert((v[0], v[1], v[2], v[3], v[4]), right == '#');
    }
    for gen in 0..2000 {
        // eprintln!(
        //     "{gen:2} [{left:3}] {}",
        //     pots.iter()
        //         .map(|&p| if p { '#' } else { '.' })
        //         .collect::<String>()
        // );
        let mut next_pots = [false, false, false]
            .into_iter()
            .chain(pots.iter().copied())
            .chain([false, false, false])
            .tuple_windows()
            .map(|w| *rules.get(&w).unwrap_or(&false))
            .collect::<VecDeque<_>>();
        let mut new_left = left - 1;
        while !next_pots.front().unwrap() {
            next_pots.pop_front();
            new_left += 1;
        }
        while !next_pots.back().unwrap() {
            next_pots.pop_back();
        }
        // eprintln!("{pots:?}");
        // eprintln!("{next_pots:?}");
        if next_pots == pots {
            left += (new_left - left) * (50000000000 - gen);
            return pots
                .into_iter()
                .enumerate()
                .filter_map(|(idx, p)| if p { Some(idx as i64 + left) } else { None })
                .sum();
        }
        left = new_left;
        pots = next_pots;
    }
    unreachable!();
}

xaoc::xaoc!(sample_idx = 23);
