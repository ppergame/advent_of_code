use num::BigInt;

fn num(c: char) -> i64 {
    match c {
        '2' => 2,
        '1' => 1,
        '0' => 0,
        '-' => -1,
        '=' => -2,
        _ => unreachable!(),
    }
}

fn parse(s: &str) -> i64 {
    let mut ret = 0;
    for c in s.chars() {
        ret = ret * 5 + num(c);
    }
    ret
}

fn unparse(n: i64) -> String {
    let mut mid5 = 2;
    while mid5 < n {
        mid5 *= 5;
        mid5 += 2;
    }
    (BigInt::from(mid5) + n)
        .to_str_radix(5)
        .chars()
        .map(|c| match c {
            '4' => '2',
            '3' => '1',
            '2' => '0',
            '1' => '-',
            '0' => '=',
            _ => unreachable!(),
        })
        .collect()
}

fn part1(inp: &str) -> String {
    let mut sum = 0;
    for line in inp.lines() {
        sum += parse(line);
    }
    unparse(sum)
}

fn part2(_inp: &str) -> i64 {
    0
}

xaoc::xaoc!();
