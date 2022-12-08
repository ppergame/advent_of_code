use std::collections::HashMap;

use itertools::Itertools;
use sscanf::scanf;

fn part1(inp: &str) -> usize {
    let mut sum = 0;
    for line in inp.lines() {
        let mut sp = line.split('-').collect::<Vec<_>>();
        let last = sp.pop().unwrap();
        let (sector, ck) = scanf!(last, "{}[{}]", usize, str).unwrap();
        let mut freq = HashMap::<char, i64>::new();
        for s in sp {
            for c in s.chars() {
                *freq.entry(c).or_default() += 1;
            }
        }
        let calc_ck = freq
            .iter()
            .sorted_by_key(|(c, f)| (-**f, *c))
            .map(|(c, _)| c)
            .take(5)
            .join("");
        if ck == calc_ck {
            sum += sector;
        }
    }
    sum
}

fn part2(inp: &str) -> usize {
    for line in inp.lines() {
        let mut sp = line.split('-').collect::<Vec<_>>();
        let last = sp.pop().unwrap();
        let (sector, ck) = scanf!(last, "{}[{}]", usize, str).unwrap();
        let mut freq = HashMap::<char, i64>::new();
        for s in &sp {
            for c in s.chars() {
                *freq.entry(c).or_default() += 1;
            }
        }
        let calc_ck = freq
            .iter()
            .sorted_by_key(|(c, f)| (-**f, *c))
            .map(|(c, _)| c)
            .take(5)
            .join("");
        if ck == calc_ck
            && sp
                .iter()
                .map(|s| {
                    s.chars()
                        .map(|c| {
                            char::from_u32(
                                (((((c as u8) - 97) as usize + sector) % 26) + 97) as u32,
                            )
                            .unwrap()
                        })
                        .join("")
                })
                .join(" ")
                == "northpole object storage"
        {
            return sector;
        }
    }
    unreachable!();
}

xaoc::xaoc!();
