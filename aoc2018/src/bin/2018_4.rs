use itertools::Itertools;
use sscanf::scanf;
use std::collections::HashMap;

fn part1(inp: &str) -> usize {
    let mut sleeps = HashMap::<usize, Vec<(usize, usize)>>::new();
    let mut iter = inp.lines().sorted().peekable();
    while let Some(line) = iter.next() {
        let (_whatever, _min, guardno) =
            scanf!(line, "[{}:{}] Guard #{} begins shift", str, usize, usize).unwrap();
        loop {
            if iter
                .peek()
                .map_or(true, |line| line.contains("begins shift"))
            {
                break;
            }
            let (_whatever, start) =
                scanf!(iter.next().unwrap(), "[{}:{}] falls asleep", str, usize).unwrap();
            let (_whatever, end) =
                scanf!(iter.next().unwrap(), "[{}:{}] wakes up", str, usize).unwrap();
            sleeps.entry(guardno).or_default().push((start, end));
        }
    }
    let (guardno, ranges) = sleeps
        .into_iter()
        .max_by_key(|(_, ranges)| ranges.iter().map(|(start, end)| end - start).sum::<usize>())
        .unwrap();
    let mut minutes = counter::Counter::<usize>::new();
    for (start, end) in ranges {
        minutes.extend(start..end);
    }
    minutes.most_common()[0].0 * guardno
}

fn part2(inp: &str) -> usize {
    let mut sleeps = HashMap::<usize, Vec<(usize, usize)>>::new();
    let mut iter = inp.lines().sorted().peekable();
    while let Some(line) = iter.next() {
        let (_whatever, _min, guardno) =
            scanf!(line, "[{}:{}] Guard #{} begins shift", str, usize, usize).unwrap();
        loop {
            if iter
                .peek()
                .map_or(true, |line| line.contains("begins shift"))
            {
                break;
            }
            let (_whatever, start) =
                scanf!(iter.next().unwrap(), "[{}:{}] falls asleep", str, usize).unwrap();
            let (_whatever, end) =
                scanf!(iter.next().unwrap(), "[{}:{}] wakes up", str, usize).unwrap();
            sleeps.entry(guardno).or_default().push((start, end));
        }
    }
    let (guardno, (minute, _count)) = sleeps
        .into_iter()
        .map(|(guardno, ranges)| {
            let mut minutes = counter::Counter::<usize>::new();
            for (start, end) in ranges {
                minutes.extend(start..end);
            }
            (guardno, minutes.most_common()[0])
        })
        .max_by_key(|(_, (_, count))| *count)
        .unwrap();
    guardno * minute
}

xaoc::xaoc!();
