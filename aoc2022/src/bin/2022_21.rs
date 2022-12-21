use itertools::Itertools;
use sscanf::scanf;
use std::ops::Index;
use std::{collections::HashMap, fmt::Display};
use z3::ast::{Ast, Int};
use z3::SatResult;

struct Monkeys(HashMap<Id, Op>);

impl Index<Id> for Monkeys {
    type Output = Op;

    fn index(&self, index: Id) -> &Self::Output {
        &self.0[&index]
    }
}

impl Index<&str> for Monkeys {
    type Output = Op;

    fn index(&self, index: &str) -> &Self::Output {
        &self.0[&index.into()]
    }
}

impl Monkeys {
    fn parse(inp: &str) -> Self {
        let mut monkeys = HashMap::<Id, Op>::new();
        for line in inp.lines() {
            let (name, op) = line.split_once(": ").unwrap();
            monkeys.insert(name.into(), Op::parse(op));
        }
        Monkeys(monkeys)
    }

    fn names(&self) -> impl Iterator<Item = Id> + '_ {
        self.0.keys().copied()
    }

    fn propagate(&self, part1: bool) -> HashMap<Id, Val> {
        let mut vals = HashMap::<Id, Val>::new();
        for id in self.names() {
            vals.insert(id, Val::Unknown);
        }
        let mut stack = vec![Id::from("root")];
        while let Some(name) = stack.pop() {
            if let Op::Imm(imm) = self[name] {
                if !part1 && name == "humn".into() {
                    vals.insert(name, Val::DependsOnHuman);
                } else {
                    vals.insert(name, Val::Known(imm));
                }
                continue;
            }
            let Op::Bin(_, a1, a2) = self[name] else { unreachable!() };
            match (vals[&a1], vals[&a2]) {
                (Val::Unknown, Val::Unknown) => {
                    stack.push(name);
                    stack.push(a1);
                    stack.push(a2);
                }
                (Val::Unknown, _) => {
                    stack.push(name);
                    stack.push(a1);
                }
                (_, Val::Unknown) => {
                    stack.push(name);
                    stack.push(a2);
                }
                (Val::DependsOnHuman, Val::DependsOnHuman) => unreachable!(),
                (Val::DependsOnHuman, _) | (_, Val::DependsOnHuman) => {
                    vals.insert(name, Val::DependsOnHuman);
                }
                (Val::Known(v1), Val::Known(v2)) => {
                    vals.insert(name, Val::Known(self[name].compute(v1, v2).unwrap()));
                }
            }
        }
        vals
    }

    fn z3_solve(&self, part1: bool) -> HashMap<Id, Val> {
        let cfg = z3::Config::new();
        let ctx = z3::Context::new(&cfg);
        let opt = z3::Optimize::new(&ctx);
        let mut vars = HashMap::new();
        for id in self.names() {
            vars.insert(id, Int::new_const(&ctx, id.to_string()));
        }
        for id in self.names() {
            let op = self[id];
            let var = &vars[&id];
            match op {
                Op::Imm(imm) => {
                    if !part1 && id == "humn".into() {
                    } else {
                        opt.assert(&var._eq(&Int::from_i64(&ctx, imm)));
                    }
                }
                Op::Bin(op, a1, a2) => {
                    assert_ne!(id, "humn".into());
                    let var1 = &vars[&a1];
                    let var2 = &vars[&a2];
                    if !part1 && id == "root".into() {
                        opt.assert(&var._eq(var1));
                        opt.assert(&var._eq(var2));
                    } else {
                        match op {
                            Binop::Add => opt.assert(&var._eq(&Int::add(&ctx, &[var1, var2]))),
                            Binop::Sub => opt.assert(&var._eq(&Int::sub(&ctx, &[var1, var2]))),
                            Binop::Div => opt.assert(&var._eq(&z3_div(&ctx, var1, var2))),
                            Binop::Mul => opt.assert(&var._eq(&Int::mul(&ctx, &[var1, var2]))),
                        }
                    }
                }
            }
        }
        opt.minimize(&vars[&"humn".into()]);
        assert!(matches!(opt.check(&[]), SatResult::Sat));
        let model = opt.get_model().unwrap();
        let mut vals = HashMap::new();
        for (id, var) in vars {
            vals.insert(
                id,
                Val::Known(model.eval(&var, false).unwrap().as_i64().unwrap()),
            );
        }
        vals
    }

    #[allow(dead_code)]
    fn validate(&self, vals: &HashMap<Id, Val>, part1: bool) {
        for id in self.names().sorted() {
            let op = self[id];
            let val = vals[&id].known().expect("{id} not known");
            match op {
                Op::Imm(imm) => {
                    if !part1 && id == "humn".into() {
                    } else {
                        assert_eq!(val, imm, "{id} != {imm}");
                    }
                }
                Op::Bin(bop, a1, a2) => {
                    let v1 = vals[&a1]
                        .known()
                        .unwrap_or_else(|| panic!("{a1} not known"));
                    let v2 = vals[&a2]
                        .known()
                        .unwrap_or_else(|| panic!("{a2} not known"));
                    if !part1 && id == "root".into() {
                        assert_eq!(val, v1, "root != {a1}");
                        assert_eq!(val, v2, "root != {a2}");
                    } else {
                        let expected = op.compute(v1, v2).unwrap();
                        assert_eq!(
                            val,
                            expected,
                            "{id} != {a1}({v1}) {} {a2}({v2})",
                            bop.as_char()
                        );
                    }
                }
            }
        }
    }
}

fn z3_div<'a>(ctx: &'a z3::Context, a: &Int<'a>, b: &Int<'a>) -> Int<'a> {
    a.ge(&Int::from_i64(ctx, 0))
        .ite(&a.div(b), &a.unary_minus().div(&b.unary_minus()))
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Id([u8; 4]);

impl From<&str> for Id {
    fn from(value: &str) -> Self {
        let b = value.as_bytes();
        assert_eq!(b.len(), 4);
        Id([b[0], b[1], b[2], b[3]])
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", std::str::from_utf8(&self.0).unwrap())
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Val {
    Unknown,
    DependsOnHuman,
    Known(i64),
}

impl Val {
    fn known(&self) -> Option<i64> {
        let Val::Known(imm) = self else { return None };
        Some(*imm)
    }
}

#[derive(Debug, Copy, Clone)]
enum Binop {
    Add,
    Sub,
    Mul,
    Div,
}

impl Binop {
    #[allow(dead_code)]
    fn as_char(&self) -> char {
        match self {
            Binop::Add => '+',
            Binop::Sub => '-',
            Binop::Mul => '*',
            Binop::Div => '/',
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Op {
    Imm(i64),
    Bin(Binop, Id, Id),
}

impl Op {
    fn parse(s: &str) -> Self {
        if let Ok(imm) = s.parse() {
            Op::Imm(imm)
        } else if let Ok((a1, op, a2)) = scanf!(s, "{str} {char} {str}") {
            let a1 = a1.into();
            let a2 = a2.into();
            match op {
                '+' => Op::Bin(Binop::Add, a1, a2),
                '-' => Op::Bin(Binop::Sub, a1, a2),
                '*' => Op::Bin(Binop::Mul, a1, a2),
                '/' => Op::Bin(Binop::Div, a1, a2),
                _ => unreachable!(),
            }
        } else {
            unreachable!();
        }
    }

    fn compute(&self, v1: i64, v2: i64) -> Option<i64> {
        match self {
            Op::Imm(imm) => Some(*imm),
            Op::Bin(Binop::Add, _, _) => Some(v1 + v2),
            Op::Bin(Binop::Sub, _, _) => Some(v1 - v2),
            Op::Bin(Binop::Mul, _, _) => Some(v1 * v2),
            Op::Bin(Binop::Div, _, _) => Some(v1 / v2),
        }
    }
}

fn part1(inp: &str) -> i64 {
    let monkeys = Monkeys::parse(inp);
    let vals = monkeys.propagate(true);
    monkeys.validate(&vals, true);
    let vals2 = monkeys.z3_solve(true);
    monkeys.validate(&vals2, true);
    assert_eq!(vals, vals2);
    vals[&"root".into()].known().unwrap()
}

fn part2(inp: &str) -> i64 {
    let monkeys = Monkeys::parse(inp);
    let mut vals = monkeys.propagate(false);
    let Op::Bin(_, a1, a2) = monkeys["root"] else { unreachable!() };
    let mut name;
    let mut expected;
    match (vals[&a1], vals[&a2]) {
        (Val::DependsOnHuman, Val::Known(v2)) => {
            name = a1;
            expected = v2;
        }
        (Val::Known(v1), Val::DependsOnHuman) => {
            name = a2;
            expected = v1;
        }
        _ => unreachable!(),
    }
    vals.insert("root".into(), Val::Known(expected));
    loop {
        vals.insert(name, Val::Known(expected));
        if name == "humn".into() {
            break;
        }
        let Op::Bin(op, a1, a2) = monkeys[name] else { unreachable!() };
        match (vals[&a1], vals[&a2]) {
            (Val::DependsOnHuman, Val::Known(v2)) => {
                match op {
                    Binop::Add => expected -= v2,
                    Binop::Sub => expected += v2,
                    Binop::Mul => expected /= v2,
                    Binop::Div => expected *= v2,
                }
                name = a1;
            }
            (Val::Known(v1), Val::DependsOnHuman) => {
                match op {
                    Binop::Add => expected -= v1,
                    Binop::Sub => expected = v1 - expected,
                    Binop::Mul => expected /= v1,
                    Binop::Div => expected = v1 / expected,
                }
                name = a2;
            }
            _ => unreachable!(),
        };
    }
    monkeys.validate(&vals, false);

    let vals2 = monkeys.z3_solve(false);
    monkeys.validate(&vals2, false);
    assert_eq!(vals.len(), vals2.len());
    for k in vals.keys().sorted() {
        if vals[k] != vals2[k] {
            eprintln!("{k}: {:?} -> {:?}", vals[k], vals2[k]);
        }
        // assert_eq!(vals[k], vals2[k], "{k}");
    }
    expected
}

xaoc::xaoc!(
    sample = r#"root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32"#
);
