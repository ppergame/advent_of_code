use itertools::Itertools as _;

#[derive(Debug)]
struct Machine {
    target: i64,
    buttons1: Vec<i64>,
    buttons2: Vec<Vec<usize>>,
    joltage: Vec<i64>,
}

impl Machine {
    fn parse(inp: &str) -> Vec<Self> {
        inp.lines()
            .map(|l| {
                let mut chunks = l.split_ascii_whitespace().peekable();
                let target = chunks
                    .next()
                    .unwrap()
                    .trim_matches(['[', ']'])
                    .chars()
                    .rev()
                    .fold(0, |acc, c| match c {
                        '.' => acc << 1,
                        '#' => acc << 1 | 1,
                        _ => unreachable!(),
                    });
                let mut buttons1 = vec![];
                let mut buttons2 = vec![];
                let joltage;
                loop {
                    let chunk = chunks.next().unwrap();
                    if chunks.peek().is_none() {
                        joltage = chunk
                            .trim_matches(['{', '}'])
                            .split(',')
                            .map(|s| s.parse().unwrap())
                            .collect();
                        break;
                    }
                    let mut button1 = 0;
                    let mut button2 = vec![];
                    for n in chunk.trim_matches(['(', ')']).split(',') {
                        let n = n.parse().unwrap();
                        button1 |= 1i64 << n;
                        button2.push(n);
                    }
                    buttons1.push(button1);
                    buttons2.push(button2);
                }
                Self {
                    target,
                    buttons1,
                    buttons2,
                    joltage,
                }
            })
            .collect()
    }
}

fn part1(inp: &str) -> usize {
    let machines = Machine::parse(inp);
    machines
        .into_iter()
        .map(|m| {
            (1..m.buttons1.len())
                .flat_map(|i| m.buttons1.iter().combinations(i))
                .find(|c| c.iter().fold(0, |acc, &b| acc ^ b) == m.target)
                .unwrap()
                .len()
        })
        .sum()
}

fn search_z3(m: &Machine) -> usize {
    let opt = z3::Optimize::new();
    let counts: Vec<_> = (0..m.buttons2.len())
        .map(|i| z3::ast::Int::new_const(format!("c_{}", i)))
        .collect();
    let mut counter_exprs: Vec<_> = (0..m.joltage.len())
        .map(|_| z3::ast::Int::from_i64(0))
        .collect();
    for (btn_idx, affected_indices) in m.buttons2.iter().enumerate() {
        for &counter_idx in affected_indices {
            let idx = counter_idx;
            counter_exprs[idx] = &counter_exprs[idx] + &counts[btn_idx];
        }
    }
    for (expr, &target_val) in counter_exprs.iter().zip(&m.joltage) {
        opt.assert(&expr.eq(z3::ast::Int::from_i64(target_val)));
    }
    let zero = z3::ast::Int::from_i64(0);
    for c in &counts {
        opt.assert(&c.ge(&zero));
    }
    let total_presses = z3::ast::Int::add(&counts.iter().collect::<Vec<_>>());
    opt.minimize(&total_presses);
    match opt.check(&[]) {
        z3::SatResult::Sat => {
            let model = opt.get_model().unwrap();
            model.eval(&total_presses, true).unwrap().as_i64().unwrap() as usize
        }
        r => panic!("z3 failed to solve: {r:?}"),
    }
}

fn part2(inp: &str) -> usize {
    let machines = Machine::parse(inp);
    machines.iter().map(search_z3).sum()
}

xaoc::xaoc!();
