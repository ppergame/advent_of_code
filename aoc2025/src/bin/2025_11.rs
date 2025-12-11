use hashbrown::HashMap;
use pathfinding::prelude::count_paths;

fn parse<'a>(inp: &'a str) -> impl Fn(&str, &str) -> usize + 'a {
    let next = inp
        .lines()
        .map(|l| {
            let (node, outs) = l.split_once(": ").unwrap();
            (node, outs.split_whitespace().collect::<Vec<_>>())
        })
        .collect::<HashMap<_, _>>();
    move |from, to| {
        count_paths(
            from,
            |s| next.get(s).cloned().unwrap_or_default(),
            |&s| s == to,
        )
    }
}

fn part1(inp: &str) -> usize {
    parse(inp)("you", "out")
}

fn part2(inp: &str) -> usize {
    let count = parse(inp);
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
