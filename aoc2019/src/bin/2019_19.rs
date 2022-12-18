use aoc2019::intcode::*;
use std::collections::HashMap;

type Coord = (u64, u64);
type Range = std::ops::Range<u64>;

fn part1(inp: &str) -> usize {
    let prog = inp
        .split(',')
        .map(|i| i.parse().unwrap())
        .collect::<Vec<i64>>();

    let mut map = HashMap::<Coord, bool>::new();
    for row in 0..50 {
        for col in 0..50 {
            let res = at(col, row, &prog);
            map.insert((col, row), res);
            // print!("{}", if res { '#' } else { '.' });
        }
        // println!();
    }

    map.values().filter(|x| **x).count()
}

fn at(x: u64, y: u64, prog: &[i64]) -> bool {
    //println!("calling at {} {}", x, y);
    let mut ic = Intcode::new_with_seq(prog);
    assert!(matches!(ic.run().unwrap(), IntcodeStatus::Input));
    ic.input = Some(x as i64);
    assert!(matches!(ic.run().unwrap(), IntcodeStatus::Input));
    ic.input = Some(y as i64);
    let hit = match ic.run().unwrap() {
        IntcodeStatus::Output(output) => output,
        _ => panic!("bad status"),
    };
    match hit {
        0 => false,
        1 => true,
        _ => panic!("bad output"),
    }
}

fn part2(inp: &str) -> u64 {
    let prog = inp
        .split(',')
        .map(|i| i.parse().unwrap())
        .collect::<Vec<i64>>();
    let mut ranges = Vec::<Range>::new();
    let mut prev_range = 0..0;
    for row in 0..100000 {
        let Some(start) = (prev_range.start..prev_range.end + 5)
            .find(|col| at(*col, row, &prog)) else {
                ranges.push(0..0);
                continue;
            };
        let end = (start.max(prev_range.end)..)
            .find(|col| !at(*col, row, &prog))
            .unwrap();
        prev_range = start..end;
        ranges.push(start..end);
        if prev_range.contains(&(start + (100 - 1))) {
            let toprow = row - (100 - 1);
            if ranges[toprow as usize].contains(&start)
                && ranges[toprow as usize].contains(&(start + (100 - 1)))
            {
                return start * 10000 + toprow;
            }
        }
    }
    unreachable!();
    //println!("{:?} {}", prev_range, prev_range.end - prev_range.start);
}

xaoc::xaoc!();
