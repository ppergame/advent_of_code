use itertools::Itertools as _;
use sscanf::scanf;
use std::collections::HashMap;

struct Hand {
    id: String,
    bid: i64,
}

impl Hand {
    fn parse(line: &str) -> Self {
        let (id, bid) = scanf!(line, "{String} {i64}").unwrap();
        Self { id, bid }
    }

    fn as_num(&self) -> i64 {
        let mut num = 0;
        for c in self.id.chars() {
            num <<= 4;
            num |= match c {
                '2' => 0,
                '3' => 1,
                '4' => 2,
                '5' => 3,
                '6' => 4,
                '7' => 5,
                '8' => 6,
                '9' => 7,
                'T' => 8,
                'J' => 9,
                'Q' => 10,
                'K' => 11,
                'A' => 12,
                _ => unreachable!(),
            };
        }
        num
    }

    fn as_num2(&self) -> i64 {
        let mut num = 0;
        for c in self.id.chars() {
            num <<= 4;
            num |= match c {
                'J' => 0,
                '2' => 1,
                '3' => 2,
                '4' => 3,
                '5' => 4,
                '6' => 5,
                '7' => 6,
                '8' => 7,
                '9' => 8,
                'T' => 9,
                'Q' => 10,
                'K' => 11,
                'A' => 12,
                _ => unreachable!(),
            };
        }
        num
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Type {
    HighCard = 0,
    OnePair = 1,
    TwoPair = 2,
    ThreeOfAKind = 3,
    FullHouse = 4,
    FourOfAKind = 5,
    FiveOfAKind = 6,
}

impl Type {
    fn from_counts(counts: &HashMap<char, usize>) -> Self {
        let vals = counts.values().copied().sorted().collect::<Vec<_>>();
        if vals == [5] {
            return Self::FiveOfAKind;
        }
        if vals == [1, 4] {
            return Self::FourOfAKind;
        }
        if vals == [2, 3] {
            return Self::FullHouse;
        }
        if vals == [1, 1, 3] {
            return Self::ThreeOfAKind;
        }
        if vals == [1, 2, 2] {
            return Self::TwoPair;
        }
        if vals == [1, 1, 1, 2] {
            return Self::OnePair;
        }
        if vals == [1, 1, 1, 1, 1] {
            return Self::HighCard;
        }
        unreachable!();
    }

    fn parse1(id: &str) -> Self {
        let mut counts = HashMap::<char, usize>::new();
        for c in id.chars() {
            *counts.entry(c).or_default() += 1;
        }
        Self::from_counts(&counts)
    }

    fn parse2(id: &str) -> Self {
        if id == "JJJJJ" {
            return Self::FiveOfAKind;
        }
        let mut counts = HashMap::<char, usize>::new();
        for c in id.chars() {
            *counts.entry(c).or_default() += 1;
        }
        let j_count = counts.remove(&'J').unwrap_or(0);
        let max_key = *counts.iter().max_by_key(|(_, v)| **v).unwrap().0;
        *counts.get_mut(&max_key).unwrap() += j_count;
        Self::from_counts(&counts)
    }
}

fn part1(inp: &str) -> i64 {
    let mut hands = inp.trim().lines().map(Hand::parse).collect::<Vec<_>>();
    hands.sort_by_key(|h| (Type::parse1(&h.id), h.as_num()));
    let mut ret = 0;
    for (idx, hand) in hands.iter().enumerate() {
        ret += hand.bid * (idx as i64 + 1);
    }
    ret
}

fn part2(inp: &str) -> i64 {
    let mut hands = inp.trim().lines().map(Hand::parse).collect::<Vec<_>>();
    hands.sort_by_key(|h| (Type::parse2(&h.id), h.as_num2()));
    let mut ret = 0;
    for (idx, hand) in hands.iter().enumerate() {
        ret += hand.bid * (idx as i64 + 1);
    }
    ret
}

xaoc::xaoc!(
    sample = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"
);
