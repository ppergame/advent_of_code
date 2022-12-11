use std::collections::HashMap;

use sscanf::scanf;

enum Oper {
    Old,
    Num(i64),
}

impl Oper {
    fn new(s: &str) -> Self {
        if s == "old" {
            return Oper::Old;
        }
        Oper::Num(s.parse().unwrap())
    }

    fn apply(&self, op: char, i: i64) -> i64 {
        let oper = match self {
            Oper::Old => i,
            Oper::Num(n) => *n,
        };
        match op {
            '*' => i * oper,
            '+' => i + oper,
            _ => unreachable!(),
        }
    }
}

struct Monkey {
    items: Vec<i64>,
    op: char,
    oper: Oper,
    div_oper: i64,
    throw_true: usize,
    throw_false: usize,
    inspect_count: usize,
}

fn parse(inp: &str) -> (i64, Vec<Monkey>) {
    let mut divisor = 1;
    let mut monkeys = vec![];
    for blob in inp.split("\n\n") {
        let mut li = blob.lines();
        let _ = scanf!(li.next().unwrap(), "Monkey {}:", usize).unwrap();
        let items = scanf!(li.next().unwrap(), "  Starting items: {}", str).unwrap();
        let items = items
            .split(',')
            .map(|s| s.trim().parse::<i64>().unwrap())
            .collect();
        let (op, oper) = scanf!(
            li.next().unwrap(),
            "  Operation: new = old {} {}",
            char,
            str
        )
        .unwrap();
        let oper = Oper::new(oper);
        let div_oper = scanf!(li.next().unwrap(), "  Test: divisible by {}", i64).unwrap();
        divisor *= div_oper;
        let throw_true =
            scanf!(li.next().unwrap(), "    If true: throw to monkey {}", usize).unwrap();
        let throw_false = scanf!(
            li.next().unwrap(),
            "    If false: throw to monkey {}",
            usize
        )
        .unwrap();
        monkeys.push(Monkey {
            items,
            op,
            oper,
            div_oper,
            throw_true,
            throw_false,
            inspect_count: 0,
        });
    }
    (divisor, monkeys)
}

fn part1(inp: &str) -> usize {
    let (_, mut monkeys) = parse(inp);
    for _round in 0..20 {
        for cur in 0..monkeys.len() {
            let cur = &mut monkeys[cur];
            let mut dispatch = HashMap::<usize, Vec<i64>>::new();
            for item in cur.items.drain(..) {
                cur.inspect_count += 1;
                let item = cur.oper.apply(cur.op, item) / 3;
                let dest = if item % cur.div_oper == 0 {
                    cur.throw_true
                } else {
                    cur.throw_false
                };
                dispatch.entry(dest).or_default().push(item);
            }
            for (dest, items) in dispatch {
                monkeys[dest].items.extend(items);
            }
        }
    }
    let mut ic = monkeys
        .iter()
        .map(|cur| cur.inspect_count)
        .collect::<Vec<_>>();
    ic.sort();
    ic.reverse();
    ic[0] * ic[1]
}

fn part2(inp: &str) -> usize {
    let (divisor, mut monkeys) = parse(inp);
    for _round in 1..=10000 {
        for i in 0..monkeys.len() {
            let cur = &mut monkeys[i];
            let mut dispatch = HashMap::<usize, Vec<i64>>::new();
            for item in cur.items.drain(..) {
                cur.inspect_count += 1;
                let item = cur.oper.apply(cur.op, item) % divisor;
                let dest = if item % cur.div_oper == 0 {
                    cur.throw_true
                } else {
                    cur.throw_false
                };
                dispatch.entry(dest).or_default().push(item);
            }
            for (dest, items) in dispatch {
                monkeys[dest].items.extend(items);
            }
        }
    }
    let mut ic = monkeys
        .iter()
        .map(|cur| cur.inspect_count)
        .collect::<Vec<_>>();
    ic.sort();
    ic.reverse();
    ic[0] * ic[1]
}

xaoc::xaoc!();
