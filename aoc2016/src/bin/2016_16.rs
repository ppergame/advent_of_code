use itertools::Itertools;

// 1 3 7 15 31 63 (n**2 - 1)

fn checksum(v: &[bool]) -> Vec<bool> {
    let mut res = v.to_vec();
    while res.len() % 2 == 0 {
        res = res.into_iter().tuples().map(|(a, b)| a == b).collect();
    }
    res
}

fn to_string(v: &[bool]) -> String {
    v.iter().map(|b| if *b { '1' } else { '0' }).collect()
}

fn part1(mut inp: &str) -> String {
    let mut target = 272;
    if inp == "0" {
        inp = "10000";
        target = 20;
    }
    let mut data = inp
        .chars()
        .map(|c| match c {
            '0' => false,
            '1' => true,
            _ => unreachable!(),
        })
        .collect::<Vec<_>>();
    while data.len() < target {
        let acc = data.iter().rev().map(|b| !*b).collect::<Vec<_>>();
        data.push(false);
        data.extend(acc);
    }
    to_string(&checksum(&data[0..target]))
}

fn part2(mut inp: &str) -> String {
    let mut target = 35651584;
    if inp == "0" {
        inp = "10000";
        target = 20;
    }
    let mut data = inp
        .chars()
        .map(|c| match c {
            '0' => false,
            '1' => true,
            _ => unreachable!(),
        })
        .collect::<Vec<_>>();
    while data.len() < target {
        let acc = data.iter().rev().map(|b| !*b).collect::<Vec<_>>();
        data.push(false);
        data.extend(acc);
    }
    to_string(&checksum(&data[0..target]))
}

xaoc::xaoc!();
