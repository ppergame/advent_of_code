use std::ops::RangeInclusive;

struct Password {
    r: RangeInclusive<usize>,
    c: char,
    pw: String,
}

fn part1(inp: &str) -> usize {
    let pws = inp.lines().map(|line| {
        let mut sp = line.split_ascii_whitespace();
        let mut sp2 = sp.next().unwrap().split('-');
        let r = sp2.next().unwrap().parse().unwrap()..=sp2.next().unwrap().parse().unwrap();
        let c = sp.next().unwrap().chars().next().unwrap();
        let pw = sp.next().unwrap().to_string();
        Password { r, c, pw }
    });
    pws.filter(|pw| pw.r.contains(&pw.pw.chars().filter(|&x| x == pw.c).count()))
        .count()
}

fn part2(inp: &str) -> usize {
    let pws = inp.lines().map(|line| {
        let mut sp = line.split_ascii_whitespace();
        let mut sp2 = sp.next().unwrap().split('-');
        let r = sp2.next().unwrap().parse().unwrap()..=sp2.next().unwrap().parse().unwrap();
        let c = sp.next().unwrap().chars().next().unwrap();
        let pw = sp.next().unwrap().to_string();
        Password { r, c, pw }
    });
    pws.filter(|pw| {
        let start = *pw.r.start();
        let end = *pw.r.end();
        let chars = pw.pw.chars().collect::<Vec<_>>();
        (chars[start - 1] == pw.c) ^ (chars[end - 1] == pw.c)
    })
    .count()
}

xaoc::xaoc!();
