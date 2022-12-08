use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

const MSG: &str = "children: 3
cats: 7
samoyeds: 2
pomeranians: 3
akitas: 0
vizslas: 0
goldfish: 5
trees: 3
cars: 2
perfumes: 1";

lazy_static! {
    static ref SUE_RE: Regex = Regex::new(r"Sue (\d+)").unwrap();
    static ref MSG_RE: Regex = Regex::new(r"(\w+): (\d+)").unwrap();
}

fn parse_msg(msg: &str) -> HashMap<String, usize> {
    msg.lines()
        .map(|line| {
            let caps = MSG_RE.captures(line).unwrap();
            (
                caps.get(1).unwrap().as_str().to_owned(),
                caps.get(2).unwrap().as_str().parse().unwrap(),
            )
        })
        .collect()
}

fn parse(inp: &str) -> Vec<(usize, HashMap<String, usize>)> {
    inp.lines()
        .map(|line| {
            (
                SUE_RE
                    .captures(line)
                    .unwrap()
                    .get(1)
                    .unwrap()
                    .as_str()
                    .parse()
                    .unwrap(),
                MSG_RE
                    .captures_iter(line)
                    .map(|m| {
                        (
                            m.get(1).unwrap().as_str().to_owned(),
                            m.get(2).unwrap().as_str().parse().unwrap(),
                        )
                    })
                    .collect(),
            )
        })
        .collect()
}

fn part1(inp: &str) -> usize {
    let msg = parse_msg(MSG);
    let sues = parse(inp);
    for (idx, map) in sues {
        if map.into_iter().all(|(k, v)| msg.get(&k) == Some(&v)) {
            return idx;
        }
    }
    unreachable!();
}

fn part2(inp: &str) -> usize {
    let msg = parse_msg(MSG);
    let sues = parse(inp);
    for (idx, map) in sues {
        if map.into_iter().all(|(k, v)| match k.as_str() {
            "cats" => v > msg[&k],
            "trees" => v > msg[&k],
            "pomeranians" => v < msg[&k],
            "goldfish" => v < msg[&k],
            _ => v == msg[&k],
        }) {
            return idx;
        }
    }
    unreachable!();
}

xaoc::xaoc!();
