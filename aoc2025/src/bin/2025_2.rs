use count_digits::CountDigits as _;
use hashbrown::HashSet;
use rangemap::RangeInclusiveSet;
use sscanf::scanf;
use std::ops::RangeInclusive;

fn parse(inp: &str) -> impl Iterator<Item = RangeInclusive<u64>> {
    inp.split(',').map(|s| {
        let (start, end) = scanf!(s.trim(), "{u64}-{u64}").unwrap();
        start..=end
    })
}

fn part1(inp: &str) -> u64 {
    let set = RangeInclusiveSet::from_iter(parse(inp));
    let mut ret = 0;
    let max = *set.last().unwrap().end();
    for seqlen in 1..=max.count_digits() {
        let mult = 10u64.pow(seqlen as u32) + 1;
        for seq in 10u64.pow(seqlen as u32 - 1)..=(10u64.pow(seqlen as u32) - 1) {
            let val = seq * mult;
            if val > max {
                break;
            }
            if set.contains(&val) {
                ret += val;
            }
        }
    }
    ret
}

fn part2(inp: &str) -> u64 {
    let set = RangeInclusiveSet::from_iter(parse(inp));
    let mut ret = HashSet::new();
    let max = *set.last().unwrap().end();
    let max_digits = max.count_digits();
    for seqlen in 1..=max_digits / 2 + 1 {
        let mult = 10u64.pow(seqlen as u32);
        for seq in 10u64.pow(seqlen as u32 - 1)..=(10u64.pow(seqlen as u32) - 1) {
            if seq > max {
                break;
            }
            let mut val = seq;
            loop {
                val = val * mult + seq;
                if val > max {
                    break;
                }
                if set.contains(&val) {
                    ret.insert(val);
                }
            }
        }
    }
    ret.into_iter().sum()
}

xaoc::xaoc!();
