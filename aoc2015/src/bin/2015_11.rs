use itertools::Itertools;

fn inc(s: &[u8]) -> Vec<u8> {
    let mut s = s.to_vec();
    for i in (0..s.len()).rev() {
        if s[i] != b'z' {
            s[i] += 1;
            break;
        }
        s[i] = b'a';
    }
    s
}

fn prs(s: &[u8]) -> String {
    std::str::from_utf8(s).unwrap().to_owned()
}

fn has_straight(s: &[u8]) -> bool {
    for (i, c) in s.iter().enumerate().skip(2) {
        if s[i - 1] == c - 1 && s[i - 2] == c - 2 {
            return true;
        }
    }
    false
}

fn has_iol(s: &[u8]) -> bool {
    s.iter().any(|&c| c == b'i' || c == b'o' || c == b'l')
}

fn has_2pairs(s: &[u8]) -> bool {
    let mut pairs = vec![];
    for (i, &c) in s.iter().enumerate().take(s.len() - 1) {
        if s[i + 1] == c {
            pairs.push((i, c));
        }
    }
    pairs.into_iter().permutations(2).any(|perm| {
        let (i1, c1) = perm[0];
        let (i2, c2) = perm[1];
        i1.abs_diff(i2) > 1 && c1 != c2
    })
}

fn part1(inp: &str) -> String {
    let s = inp.as_bytes().to_vec();
    let mut s = inc(&s);
    while !has_straight(&s) || has_iol(&s) || !has_2pairs(&s) {
        s = inc(&s);
    }
    prs(&s)
}

fn part2(inp: &str) -> String {
    let s = inp.as_bytes().to_vec();
    let mut s = inc(&s);
    while !has_straight(&s) || has_iol(&s) || !has_2pairs(&s) {
        s = inc(&s);
    }
    let mut s = inc(&s);
    while !has_straight(&s) || has_iol(&s) || !has_2pairs(&s) {
        s = inc(&s);
    }
    prs(&s)
}

xaoc::xaoc!();
