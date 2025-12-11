use hashbrown::HashMap;
use pathfinding::prelude::count_paths;

fn parse<'a>(inp: &'a str) -> impl Fn(&&str) -> Vec<&'a str> + 'a {
    let next = inp
        .lines()
        .map(|l| {
            let (node, outs) = l.split_once(": ").unwrap();
            (node, outs.split_whitespace().collect::<Vec<_>>())
        })
        .collect::<HashMap<_, _>>();
    move |s| next.get(s).cloned().unwrap_or_default()
}

fn part1(inp: &str) -> usize {
    let next = parse(inp);
    count_paths("you", next, |&s| s == "out")
}

fn part2<'a>(inp: &'a str) -> usize {
    let next = parse(inp);
    let count = |from: &'a str, to: &str| count_paths(from, &next, |&s| s == to);
    count("svr", "dac") * count("dac", "fft") * count("fft", "out")
        + count("svr", "fft") * count("fft", "dac") * count("dac", "out")
}

xaoc::xaoc!(
    sample2 = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out"
);
