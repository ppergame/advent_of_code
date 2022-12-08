use std::collections::HashMap;

fn part1(inp: &str) -> usize {
    let mut ads = inp
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<usize>>();
    ads.sort_unstable();
    let mut prev = 0;
    let mut jumps1 = 0;
    let mut jumps3 = 0;
    for el in ads {
        if el - prev == 1 {
            jumps1 += 1;
        }
        if el - prev == 3 {
            jumps3 += 1;
        }
        prev = el;
    }
    jumps3 += 1;
    jumps1 * jumps3
}

fn part2(inp: &str) -> usize {
    let mut ads = inp
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<usize>>();
    ads.sort_unstable();
    ads.insert(0, 0);
    ads.push(ads.last().unwrap() + 3);
    let mut memo = HashMap::<usize, usize>::new();
    memo.insert(0, 1);
    for (cur, el) in ads.iter().enumerate().skip(1) {
        let mut sum = 0;
        for prev in (0..cur).rev() {
            if el - ads[prev] > 3 {
                break;
            }
            sum += memo[&prev];
        }
        memo.insert(cur, sum);
    }
    memo[&(ads.len() - 1)]
}

xaoc::xaoc!();
