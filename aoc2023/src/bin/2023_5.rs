use itertools::Itertools as _;
use rangemap::RangeMap;
use sscanf::scanf;

type RMap = RangeMap<usize, MapItem>;

#[derive(Eq, PartialEq, Copy, Clone)]
struct MapItem {
    start_from: usize,
    start_to: Option<usize>,
    len: usize,
}

fn parse_maps<'a>(lines: impl Iterator<Item = &'a str>) -> Vec<RMap> {
    let mut ret = vec![];
    let mut lines = lines.peekable();
    let mut max = 0;
    while lines.peek().is_some() {
        let mut map = RMap::new();
        assert!(lines.next().unwrap().ends_with(" map:"));
        for line in &mut lines {
            if line.is_empty() {
                break;
            }
            let mut sp = line.split_ascii_whitespace();
            let start_to = sp.next().unwrap().parse().unwrap();
            let start_from = sp.next().unwrap().parse().unwrap();
            let len = sp.next().unwrap().parse().unwrap();
            assert!(sp.next().is_none());
            map.insert(
                start_from..start_from + len,
                MapItem {
                    start_from,
                    start_to: Some(start_to),
                    len,
                },
            );
            max = max.max(start_from + len);
        }
        ret.push(map);
    }
    for map in &mut ret {
        let gaps = map.gaps(&(0..max)).collect::<Vec<_>>();
        for gap in gaps {
            map.insert(
                gap.clone(),
                MapItem {
                    start_from: gap.start,
                    start_to: None,
                    len: gap.len(),
                },
            );
        }
    }
    ret
}

fn part1(inp: &str) -> usize {
    let mut lines = inp.lines();
    let rest = scanf!(lines.next().unwrap(), "seeds: {str}").unwrap();
    let seeds = rest
        .trim()
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    assert!(lines.next().unwrap().is_empty());
    let maps = parse_maps(lines);
    seeds
        .into_iter()
        .map(|mut seed| {
            for map in &maps {
                let mi = map.get(&seed).unwrap();
                seed = match mi.start_to {
                    Some(start_to) => start_to + (seed - mi.start_from),
                    None => seed,
                };
            }
            seed
        })
        .min()
        .unwrap()
}

fn part2(inp: &str) -> usize {
    let mut lines = inp.lines();
    let rest = scanf!(lines.next().unwrap(), "seeds: {str}").unwrap();
    let mut vals = rest
        .trim()
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .tuples()
        .map(|(from, len)| (from, from + len))
        .collect::<Vec<_>>();
    assert!(lines.next().unwrap().is_empty());
    let maps = parse_maps(lines);
    for map in maps {
        let mut new_vals = vec![];
        for (from, to) in vals {
            let mut val = from;
            while val < to {
                let mi = map.get(&val).unwrap();
                match mi.start_to {
                    Some(start_to) => {
                        let end_from = to.min(mi.start_from + mi.len);
                        let end_to = start_to + (end_from - mi.start_from);
                        let val_to = start_to + (val - mi.start_from);
                        new_vals.push((val_to, end_to));
                        val = end_from;
                    }
                    None => {
                        let end = to.min(mi.start_from + mi.len);
                        new_vals.push((val, end));
                        val = end;
                    }
                }
            }
        }
        vals = new_vals;
    }
    vals.into_iter().map(|(from, _)| from).min().unwrap()
}

xaoc::xaoc!(
    sample = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"
);
