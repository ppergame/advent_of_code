use regex::Regex;

fn parse(inp: &str) -> (u64, u64) {
    let caps = Regex::new(r"row (\d+), column (\d+)")
        .unwrap()
        .captures(inp)
        .unwrap();
    let mut iter = [2, 1]
        .iter()
        .map(|&n| caps.get(n).unwrap().as_str().parse().unwrap());
    (iter.next().unwrap(), iter.next().unwrap())
}

fn part1(inp: &str) -> u64 {
    let (tx, ty) = parse(inp);
    let mut x = 1;
    let mut y = 1;
    let mut num = 20151125;
    loop {
        if x == tx && y == ty {
            return num;
        }
        if y == 1 {
            y = x + 1;
            x = 1;
        } else {
            x += 1;
            y -= 1;
        }
        num = (num * 252533) % 33554393;
    }
}

fn part2(_inp: &str) -> u64 {
    0
}

xaoc::xaoc!();
