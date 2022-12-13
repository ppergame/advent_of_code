use std::{cmp::Ordering, iter::Peekable};

use itertools::Itertools;

#[derive(Debug, Clone)]
enum Item {
    List(Vec<Item>),
    Num(u32),
}

fn parse(inp: &str) -> Item {
    let mut iter = inp.chars().peekable();
    _parse(&mut iter)
}

fn _parse(inp: &mut Peekable<impl Iterator<Item = char>>) -> Item {
    match inp.next().unwrap() {
        '[' => {
            let mut acc = vec![];
            if *inp.peek().unwrap() != ']' {
                loop {
                    acc.push(_parse(inp));
                    if inp.next().unwrap() == ']' {
                        break;
                    }
                }
            } else {
                inp.next();
            }
            Item::List(acc)
        }
        d @ '0'..='9' => {
            let mut acc = d.to_digit(10).unwrap();
            while ('0'..='9').contains(inp.peek().unwrap()) {
                acc *= 10;
                acc += inp.next().unwrap().to_digit(10).unwrap();
            }
            Item::Num(acc)
        }
        c => unreachable!("bad char {c:?}"),
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Item {}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Item::List(l1), Item::List(l2)) => {
                for res in l1.iter().zip_longest(l2) {
                    match res {
                        itertools::EitherOrBoth::Both(i1, i2) => {
                            let st = i1.cmp(i2);
                            if st != Ordering::Equal {
                                return st;
                            }
                        }
                        itertools::EitherOrBoth::Left(_) => return Ordering::Greater,
                        itertools::EitherOrBoth::Right(_) => return Ordering::Less,
                    }
                }
                Ordering::Equal
            }
            (Item::List(_), Item::Num(_)) => self.cmp(&Item::List(vec![other.clone()])),
            (Item::Num(_), Item::List(_)) => Item::List(vec![self.clone()]).cmp(other),
            (Item::Num(n1), Item::Num(n2)) => n1.cmp(n2),
        }
    }
}

fn part1(inp: &str) -> usize {
    let mut count = 0;
    for (idx, pair) in inp.split("\n\n").enumerate() {
        let mut li = pair.split('\n');
        let line1 = li.next().unwrap();
        let line2 = li.next().unwrap();
        let i1 = parse(line1);
        let i2 = parse(line2);
        if i1 < i2 {
            count += idx + 1;
        }
    }
    count
}

fn part2(inp: &str) -> usize {
    let mut v = vec![];
    for line in inp.lines() {
        if line.is_empty() {
            continue;
        }
        v.push(parse(line));
    }
    let d1 = parse("[[2]]");
    let d2 = parse("[[6]]");
    v.push(d1.clone());
    v.push(d2.clone());
    v.sort();
    (v.iter().position(|l| l == &d1).unwrap() + 1) * (v.iter().position(|l| l == &d2).unwrap() + 1)
}

xaoc::xaoc!(
    sample = r#"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"#
);
