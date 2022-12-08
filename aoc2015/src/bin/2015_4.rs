use rayon::prelude::*;
use xaoc::md5;

fn part1(inp: &str) -> usize {
    eprintln!("{inp:?}");
    // let inp = "abcdef";
    // let inp = "pqrstuv";
    (0..10000000)
        .into_par_iter()
        .find_first(|i| {
            format!(
                "{:x}",
                md5([inp.as_bytes(), i.to_string().as_bytes()].concat())
            )
            .starts_with("00000")
        })
        .unwrap()
}

fn part2(inp: &str) -> usize {
    (0..10000000)
        .into_par_iter()
        .find_first(|i| {
            format!(
                "{:x}",
                md5([inp.as_bytes(), i.to_string().as_bytes()].concat())
            )
            .starts_with("000000")
        })
        .unwrap()
}

xaoc::xaoc!();
