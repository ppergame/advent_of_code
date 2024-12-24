use hashbrown::HashMap;
use itertools::Itertools as _;
use rand::Rng as _;
use sha2::{Digest as _, Sha256};
use sscanf::scanf;
use std::cell::RefCell;
use std::io::Write as _;
use z3::ast::Ast as _;

#[derive(Debug, Clone, Copy)]
enum Op {
    And,
    Or,
    Xor,
}

impl Op {
    fn parse(s: &str) -> Self {
        match s {
            "AND" => Op::And,
            "OR" => Op::Or,
            "XOR" => Op::Xor,
            _ => panic!("Invalid op: {s}"),
        }
    }
}

#[derive(Debug, Clone)]
struct Rule {
    in1: String,
    in2: String,
    op: Op,
}

#[derive(Debug, Clone)]
struct Map {
    vals: RefCell<HashMap<String, bool>>,
    rules: HashMap<String, Rule>,
}

impl Map {
    fn parse(inp: &str) -> Self {
        let mut it = inp.lines();
        let mut vals = HashMap::new();
        for l in &mut it {
            if l.is_empty() {
                break;
            }
            let (name, val) = scanf!(l, "{String}: {u64}").unwrap();
            vals.insert(name, val != 0);
        }
        let mut rules = HashMap::new();
        for l in it {
            let (in1, op, in2, name) = scanf!(l, "{String} {str} {String} -> {String}").unwrap();
            let op = Op::parse(op);
            rules.insert(name, Rule { in1, in2, op });
        }
        Self {
            vals: RefCell::new(vals),
            rules,
        }
    }

    fn run(&mut self) {
        for name in self.rules.keys().cloned().collect::<Vec<_>>() {
            self.resolve(name);
        }
    }

    fn resolve(&mut self, name: String) -> bool {
        if let Some(&val) = self.vals.borrow().get(&name) {
            return val;
        }
        let rule = self.rules[&name].clone();
        let in1 = self.resolve(rule.in1.clone());
        let in2 = self.resolve(rule.in2.clone());
        let val = match rule.op {
            Op::And => in1 & in2,
            Op::Or => in1 | in2,
            Op::Xor => in1 ^ in2,
        };
        self.vals.borrow_mut().insert(name, val);
        val
    }

    fn extract(&self, c: char) -> u64 {
        let mut i = 0;
        let mut ret = 0;
        let mut shift = 1;
        loop {
            let name = format!("{c}{i:02}");
            let vals = self.vals.borrow();
            if !vals.contains_key(&name) {
                return ret;
            }
            let val = vals[&name];
            if val {
                ret |= shift;
            }
            shift <<= 1;
            i += 1;
        }
    }

    fn swap(&mut self, swaps: &[(&str, &str)]) {
        for (a, b) in swaps.iter().copied() {
            let rule_a = self.rules.remove(a).unwrap();
            let rule_b = self.rules.remove(b).unwrap();
            self.rules.insert(a.to_string(), rule_b);
            self.rules.insert(b.to_string(), rule_a);
        }
    }

    fn output_bit_count(&self) -> usize {
        (0..self.rules.len())
            .find(|i| !self.rules.contains_key(&format!("z{i:02}")))
            .unwrap()
    }

    fn inject(&mut self, c: char, mut val: u64) {
        for i in 0..self.output_bit_count() {
            let name = format!("{c}{i:02}");
            let bit = val & 1;
            val >>= 1;
            self.vals.borrow_mut().insert(name, bit != 0);
        }
    }

    fn emit(&self) {
        let mut f = std::fs::File::create("/tmp/aoc24.dot").unwrap();
        writeln!(f, "digraph G {{").unwrap();
        for (name, rule) in &self.rules {
            let label = format!("{name} [label=\"{name}\n{:?}\"]", rule.op);
            writeln!(f, "{label};").unwrap();
            writeln!(f, "  {} -> {};", rule.in1, name).unwrap();
            writeln!(f, "  {} -> {};", rule.in2, name).unwrap();
        }
        writeln!(f, "}}").unwrap();
    }
}

#[allow(dead_code)]
fn run_z3(map: &mut Map) {
    let cfg = z3::Config::new();
    let ctx = z3::Context::new(&cfg);
    let solver = z3::Solver::new(&ctx);
    let mut vars = HashMap::new();
    for (name, val) in map.vals.borrow().iter() {
        vars.insert(name.clone(), z3::ast::Bool::new_const(&ctx, name.clone()));
        solver.assert(&vars[name]._eq(&z3::ast::Bool::from_bool(&ctx, *val)));
    }
    let get_var = |vars: &mut HashMap<_, _>, name: &str| {
        vars.entry(name.to_string())
            .or_insert_with(|| z3::ast::Bool::new_const(&ctx, name.to_string()))
            .clone()
    };
    for (name, rule) in &map.rules {
        let in1 = get_var(&mut vars, &rule.in1);
        let in2 = get_var(&mut vars, &rule.in2);
        let out = get_var(&mut vars, name);
        match rule.op {
            Op::And => solver.assert(&out._eq(&z3::ast::Bool::and(&ctx, &[&in1, &in2]))),
            Op::Or => solver.assert(&out._eq(&z3::ast::Bool::or(&ctx, &[&in1, &in2]))),
            Op::Xor => solver.assert(&out._eq(&in1.xor(&in2))),
        }
    }
    assert!(matches!(solver.check(), z3::SatResult::Sat));
    let model = solver.get_model().unwrap();
    let mut vals = map.vals.borrow_mut();
    for (name, var) in vars {
        let val = model.eval(&var, false).unwrap().as_bool().unwrap();
        vals.insert(name, val);
    }
}

fn part1(inp: &str) -> u64 {
    let mut map = Map::parse(inp);
    map.run();
    map.extract('z')
}

// never mind, spins forever
/*
fn run_z3_swaps(map: &Map) -> Vec<(String, String)> {
    let cfg = z3::Config::new();
    let ctx = z3::Context::new(&cfg);
    let solver = z3::Solver::new(&ctx);
    let mut vars = HashMap::new();
    let mut carry_vars: Vec<z3::ast::Bool<'_>> = vec![];
    let get_var = |vars: &mut HashMap<_, _>, name: &str| {
        vars.entry(name.to_string())
            .or_insert_with(|| z3::ast::Bool::new_const(&ctx, name.to_string()))
            .clone()
    };
    let len = map.output_bit_count() - 1;
    for i in 0..len {
        let carry = if i == 0 {
            z3::ast::Bool::from_bool(&ctx, false)
        } else {
            carry_vars[i - 1].clone()
        };
        let x = get_var(&mut vars, &format!("x{i:02}"));
        let y = get_var(&mut vars, &format!("y{i:02}"));
        let z = get_var(&mut vars, &format!("z{i:02}"));
        solver.assert(&z._eq(&carry.xor(&x).xor(&y)));
        let carry_out = get_var(&mut vars, &format!("c{i:02}"));
        solver.assert(&carry_out._eq(&z3::ast::Bool::or(
            &ctx,
            &[
                &z3::ast::Bool::and(&ctx, &[&x, &y]),
                &z3::ast::Bool::and(&ctx, &[&carry, &x.xor(&y)]),
            ],
        )));
        carry_vars.push(carry_out);
    }
    let z = get_var(&mut vars, &format!("z{len:02}"));
    solver.assert(&z._eq(carry_vars.last().unwrap()));
    let mut fake_vars = HashMap::new();
    for (name, rule) in &map.rules {
        let in1 = get_var(&mut vars, &rule.in1);
        let in2 = get_var(&mut vars, &rule.in2);
        let fake_out = get_var(&mut fake_vars, &format!("{name}_fake"));
        match rule.op {
            Op::And => solver.assert(&fake_out._eq(&z3::ast::Bool::and(&ctx, &[&in1, &in2]))),
            Op::Or => solver.assert(&fake_out._eq(&z3::ast::Bool::or(&ctx, &[&in1, &in2]))),
            Op::Xor => solver.assert(&fake_out._eq(&in1.xor(&in2))),
        }
    }
    let mut swap_vars = HashMap::new();
    for (mut name1, mut name2) in map.rules.keys().tuple_combinations() {
        if name1 > name2 {
            std::mem::swap(&mut name1, &mut name2);
        }
        let swap = get_var(&mut swap_vars, &format!("swap_{}_{}", name1, name2));
        let fake_var1 = get_var(&mut fake_vars, &format!("{name1}_fake"));
        let fake_var2 = get_var(&mut fake_vars, &format!("{name2}_fake"));
        let select1 = swap.ite(&fake_var1, &fake_var2);
        let real_var1 = get_var(&mut vars, name1);
        solver.assert(&real_var1._eq(&select1));
        let real_var2 = get_var(&mut vars, name2);
        let select2 = swap.ite(&fake_var2, &fake_var1);
        solver.assert(&real_var2._eq(&select2));
    }
    let swap_int_vars = swap_vars
        .values()
        .map(|v| {
            v.ite(
                &z3::ast::Int::from_i64(&ctx, 1),
                &z3::ast::Int::from_i64(&ctx, 0),
            )
        })
        .collect::<Vec<_>>();
    let swap_int_vars_ref = swap_int_vars.iter().collect::<Vec<_>>();
    solver
        .assert(&z3::ast::Int::add(&ctx, &swap_int_vars_ref)._eq(&z3::ast::Int::from_i64(&ctx, 4)));
    assert!(matches!(solver.check(), z3::SatResult::Sat));
    let model = solver.get_model().unwrap();
    for (name, swap) in swap_vars {
        let val = model.eval(&swap, false).unwrap().as_bool().unwrap();
        if val {
            eprintln!("{name}");
        }
    }
    for (name, var) in vars {
        let val = model.eval(&var, false).unwrap().as_bool().unwrap();
        eprintln!("{name} = {val}");
    }
    vec![]
}
*/

fn part2(inp: &str) -> String {
    let mut map = Map::parse(inp);
    if map.rules.len() < 100 {
        return String::new();
    }
    if "b924b5fc12a621522e5217cfb19807e4fdc4524d38c7b691376b2c944e4d9756"
        != hex::encode(Sha256::digest(inp.as_bytes()))
    {
        eprintln!("only works for one input");
        return String::new();
    }
    let swaps = [
        ("dwp", "kfm"),
        ("jdr", "z31"),
        ("ffj", "z08"),
        ("gjh", "z22"),
    ];
    let bits = map.output_bit_count() - 1;
    map.swap(&swaps);
    for _ in 0..1024 {
        let mut map = map.clone();
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0..1 << bits);
        let y = rng.gen_range(0..1 << bits);
        map.inject('x', x);
        map.inject('y', y);
        map.run();
        let z = map.extract('z');
        let expected_z = x + y;
        if z != expected_z {
            map.emit();
            eprintln!();
            eprintln!("x {x:b}");
            eprintln!("y {y:b}");
            eprintln!("=  {expected_z:b}");
            eprintln!("z {z:b}");
            for (i, (b, eb)) in format!("{:b}", z)
                .chars()
                .rev()
                .zip(format!("{:b}", expected_z).chars().rev())
                .enumerate()
            {
                if b != eb {
                    eprintln!("bit {i} is wrong: {b} != {eb}");
                }
            }
            panic!();
        }
    }
    swaps
        .into_iter()
        .flat_map(|(a, b)| [a, b])
        .sorted()
        .join(",")
}

xaoc::xaoc!(
    sample = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj"
);
