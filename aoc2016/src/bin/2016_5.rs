use itertools::Itertools;
use rayon::prelude::*;
use xaoc::md5;

const STEP: usize = 1000000;

fn part1(inp: &str) -> String {
    let mut acc = vec![];
    for bi in (0..100000000).step_by(STEP) {
        let batch = (bi..bi + STEP)
            .into_par_iter()
            .filter_map(|i| {
                let sum = md5(format!("{inp}{i}"));
                if sum[0] == 0 && sum[1] == 0 && sum[2] & 0xF0 == 0 {
                    Some(sum[2] & 0xF)
                } else {
                    None
                }
            })
            .collect::<Vec<u8>>();
        acc.extend(batch);
        if acc.len() > 8 {
            return acc
                .into_iter()
                .take(8)
                .map(|b| char::from_digit(b as u32, 16).unwrap())
                .join("");
        }
    }
    unreachable!();
}

fn part2(inp: &str) -> String {
    let mut acc = vec![None; 8];
    for bi in (0..100000000).step_by(STEP) {
        let batch = (bi..bi + STEP)
            .into_par_iter()
            .filter_map(|i| {
                let sum = md5(format!("{inp}{i}"));
                if sum[0] == 0 && sum[1] == 0 && sum[2] & 0xF0 == 0 {
                    Some((sum[2] & 0xF, sum[3] >> 4))
                } else {
                    None
                }
            })
            .collect::<Vec<(u8, u8)>>();
        for (pos, b) in batch {
            if pos < 8 {
                acc.get_mut(pos as usize).unwrap().get_or_insert(b);
            }
        }
        if !acc.iter().any(|b| b.is_none()) {
            return acc
                .into_iter()
                .map(|b| char::from_digit(b.unwrap() as u32, 16).unwrap())
                .join("");
        }
    }
    unreachable!();
}

xaoc::xaoc!();
