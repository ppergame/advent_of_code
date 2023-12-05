use std::collections::HashSet;

fn spnum(s: &str) -> HashSet<usize> {
    s.trim()
        .split_ascii_whitespace()
        .map(|c| c.parse().unwrap())
        .collect()
}

fn part1(inp: &str) -> usize {
    let mut score = 0;
    for line in inp.lines() {
        let (_, rest) = line.split_once(':').unwrap();
        let (winning, have) = rest.split_once(" | ").unwrap();
        let winning = spnum(winning);
        let have = spnum(have);
        let c = winning.intersection(&have).count();
        if c > 0 {
            score += 1 << (c - 1);
        }
    }
    score
}

#[derive(Copy, Clone)]
struct Card {
    score: usize,
    count: usize,
}

fn part2(inp: &str) -> usize {
    let mut cards = vec![];
    for line in inp.lines() {
        let (_, rest) = line.split_once(':').unwrap();
        let (winning, have) = rest.split_once(" | ").unwrap();
        let winning = spnum(winning);
        let have = spnum(have);
        let score = winning.intersection(&have).count();
        cards.push(Card { score, count: 1 });
    }
    for idx in 0..cards.len() {
        let card = cards[idx];
        for card2 in cards.iter_mut().take(idx + card.score + 1).skip(idx + 1) {
            card2.count += card.count;
        }
    }
    cards.iter().map(|c| c.count).sum()
}

xaoc::xaoc!(
    sample = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
);
