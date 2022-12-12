use sscanf::scanf;

#[derive(Clone)]
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

#[derive(Clone)]
struct Monkey {
    items: Vec<i64>,
    op: char,
    oper: Oper,
    div_oper: i64,
    throw_true: usize,
    throw_false: usize,
    inspect_count: usize,
}

impl Monkey {
    fn take(&mut self) -> Self {
        let items = std::mem::take(&mut self.items);
        let mut ret = self.clone();
        ret.items = items;
        ret
    }
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

fn run(inp: &str, part1: bool) -> usize {
    let (divisor, mut monkeys) = parse(inp);
    for _round in 0..if part1 { 20 } else { 10000 } {
        for cur in 0..monkeys.len() {
            let mut monkey = monkeys[cur].take();
            for item in monkey.items.drain(..) {
                monkey.inspect_count += 1;
                let item = if part1 {
                    monkey.oper.apply(monkey.op, item) / 3
                } else {
                    monkey.oper.apply(monkey.op, item) % divisor
                };
                let dest = if item % monkey.div_oper == 0 {
                    monkey.throw_true
                } else {
                    monkey.throw_false
                };
                monkeys[dest].items.push(item);
            }
            monkeys[cur].inspect_count = monkey.inspect_count;
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

fn part1(inp: &str) -> usize {
    run(inp, true)
}

fn part2(inp: &str) -> usize {
    run(inp, false)
}

xaoc::xaoc!();
