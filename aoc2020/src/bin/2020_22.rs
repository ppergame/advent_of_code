use std::collections::{HashSet, VecDeque};

fn parse(inp: &str) -> (VecDeque<u64>, VecDeque<u64>) {
    let mut lines = inp.lines();
    assert_eq!(lines.next().unwrap(), "Player 1:");
    let mut p1 = VecDeque::new();
    for line in &mut lines {
        if line.is_empty() {
            break;
        }
        p1.push_back(line.parse().unwrap());
    }
    assert_eq!(lines.next().unwrap(), "Player 2:");
    let mut p2 = VecDeque::new();
    for line in &mut lines {
        if line.is_empty() {
            break;
        }
        p2.push_back(line.parse().unwrap());
    }
    (p1, p2)
}

enum Player {
    P1,
    P2,
}

fn score(p: VecDeque<u64>) -> u64 {
    p.iter()
        .rev()
        .enumerate()
        .map(|(i, &c)| (i + 1) as u64 * c)
        .sum()
}

fn game<const REC: bool>(mut p1: VecDeque<u64>, mut p2: VecDeque<u64>) -> (Player, u64) {
    let mut memo = HashSet::<(VecDeque<u64>, VecDeque<u64>)>::new();
    while !p1.is_empty() && !p2.is_empty() {
        let key = (p1.clone(), p2.clone());
        if memo.contains(&key) {
            //panic!();
            return (Player::P1, 0xFFFFFFFF);
        }
        memo.insert(key);
        let p1c = p1.pop_front().unwrap();
        let p2c = p2.pop_front().unwrap();
        let winner = if REC {
            if p1.len() >= p1c as usize && p2.len() >= p2c as usize {
                let np1 = p1.make_contiguous()[0..p1c as usize]
                    .iter()
                    .copied()
                    .collect::<VecDeque<_>>();
                let np2 = p2.make_contiguous()[0..p2c as usize]
                    .iter()
                    .copied()
                    .collect::<VecDeque<_>>();
                game::<REC>(np1, np2).0
            } else if p1c > p2c {
                Player::P1
            } else {
                Player::P2
            }
        } else if p1c > p2c {
            Player::P1
        } else {
            Player::P2
        };
        match winner {
            Player::P1 => {
                p1.push_back(p1c);
                p1.push_back(p2c);
            }
            Player::P2 => {
                p2.push_back(p2c);
                p2.push_back(p1c);
            }
        }
    }
    if !p1.is_empty() {
        (Player::P1, score(p1))
    } else {
        (Player::P2, score(p2))
    }
}

fn part1(inp: &str) -> u64 {
    let (p1, p2) = parse(inp);
    game::<false>(p1, p2).1
}

fn part2(inp: &str) -> u64 {
    let (p1, p2) = parse(inp);
    game::<true>(p1, p2).1
}

xaoc::xaoc!();
