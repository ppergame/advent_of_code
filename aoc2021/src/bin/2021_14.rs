use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Input {
    t: String,
    rules: HashMap<String, String>,
}

fn parse(inp: &str) -> Input {
    let (t, rr) = inp.split_once("\n\n").unwrap();
    let rules = rr
        .lines()
        .map(|line| {
            let (d, s) = line.split_once(" -> ").unwrap();
            (d.to_owned(), s.to_owned())
        })
        .collect();
    Input {
        t: t.to_owned(),
        rules,
    }
}

type Freq = HashMap<char, usize>;

fn fadd(f1: Freq, f2: Freq) -> Freq {
    let mut ret = f1;
    for (c, n) in f2 {
        *ret.entry(c).or_insert(0) += n;
    }
    ret
}

fn get_freq(
    t: &str,
    rules: &HashMap<String, String>,
    memo: &mut HashMap<(String, usize), Freq>,
    iter: usize,
) -> Freq {
    if let Some(freq) = memo.get(&(t.to_owned(), iter)) {
        return freq.clone();
    }
    if iter == 0 {
        let mut freq = HashMap::new();
        for c in t.chars() {
            *freq.entry(c).or_insert(0) += 1;
        }
        memo.insert((t.to_owned(), iter), freq.clone());
        return freq;
    }
    let mut freq = t
        .chars()
        .tuple_windows()
        .map(|(x, y)| {
            let key = x.to_string() + &y.to_string();
            let c = &rules[&key];

            let f1 = get_freq(&(x.to_string() + c), rules, memo, iter - 1);
            let f2 = get_freq(&(c.to_string() + &y.to_string()), rules, memo, iter - 1);
            let mut freq = fadd(f1, f2);
            *freq.get_mut(&c.chars().next().unwrap()).unwrap() -= 1;
            *freq.get_mut(&y).unwrap() -= 1;
            freq
        })
        .fold(HashMap::new(), fadd);
    *freq.get_mut(&t.chars().last().unwrap()).unwrap() += 1;
    memo.insert((t.to_owned(), iter), freq.clone());
    freq
}

fn part1(inp: &str) -> usize {
    let inp = parse(inp);
    let freq = get_freq(&inp.t, &inp.rules, &mut HashMap::new(), 10);
    freq.values().max().unwrap() - freq.values().min().unwrap()
}

fn part2(inp: &str) -> usize {
    let inp = parse(inp);
    let freq = get_freq(&inp.t, &inp.rules, &mut HashMap::new(), 40);
    freq.values().max().unwrap() - freq.values().min().unwrap()
}

xaoc::xaoc!();
