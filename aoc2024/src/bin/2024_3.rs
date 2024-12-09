use regex::Regex;

lazy_static::lazy_static! {
    static ref RE: Regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    static ref RE2: Regex = Regex::new(r"(do\(\)|don't\(\)|mul\((\d{1,3}),(\d{1,3})\))").unwrap();
}

fn part1(inp: &str) -> i64 {
    RE.captures_iter(inp)
        .map(|cap| cap[1].parse::<i64>().unwrap() * cap[2].parse::<i64>().unwrap())
        .sum()
}

fn part2(inp: &str) -> i64 {
    let mut doing = true;
    let mut acc = 0;
    for cap in RE2.captures_iter(inp) {
        match &cap[0] {
            "do()" => doing = true,
            "don't()" => doing = false,
            _ if doing => acc += cap[2].parse::<i64>().unwrap() * cap[3].parse::<i64>().unwrap(),
            _ => (),
        }
    }
    acc
}

xaoc::xaoc!(
    sample = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
    sample2 = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
);
