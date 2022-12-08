use std::collections::HashMap;

enum Oper<'a> {
    Imm(u64),
    Wire(&'a str),
}

enum Formula<'a> {
    Id(&'a str),
    Imm(u64),
    And(Oper<'a>, &'a str),
    Or(&'a str, &'a str),
    LShift(&'a str, u64),
    RShift(&'a str, u64),
    Not(&'a str),
}

enum Res<'a> {
    Need(Vec<&'a str>),
    Val(u64),
}

impl<'a> Formula<'a> {
    fn deps(&self) -> Vec<&'a str> {
        match self {
            Formula::Id(id) => vec![id],
            Formula::Imm(_) => vec![],
            Formula::And(op1, id2) => {
                let mut res = vec![];
                if let Oper::Wire(id1) = op1 {
                    res.push(*id1);
                }
                res.push(*id2);
                res
            }
            Formula::Or(id1, id2) => vec![id1, id2],
            Formula::LShift(id, _) => vec![id],
            Formula::RShift(id, _) => vec![id],
            Formula::Not(id) => vec![id],
        }
    }

    fn calc(&self, vals: &HashMap<&'a str, u64>) -> Res {
        let need = self
            .deps()
            .into_iter()
            .filter(|wire| !vals.contains_key(wire))
            .collect::<Vec<_>>();
        if !need.is_empty() {
            return Res::Need(need);
        }
        Res::Val(match self {
            Formula::Id(id) => vals[id],
            Formula::Imm(val) => *val,
            Formula::And(op1, id2) => {
                (match op1 {
                    Oper::Imm(imm) => *imm,
                    Oper::Wire(id1) => vals[id1],
                }) & vals[id2]
            }
            Formula::Or(id1, id2) => vals[id1] | vals[id2],
            Formula::LShift(id, val) => vals[id] << val,
            Formula::RShift(id, val) => vals[id] >> val,
            Formula::Not(id) => !vals[id],
        })
    }
}

fn parse(inp: &str) -> HashMap<&str, Formula> {
    let mut map = HashMap::new();
    for line in inp.lines() {
        let (ff, wire) = line.split_once(" -> ").unwrap();
        let sp = ff.split_whitespace().collect::<Vec<_>>();
        let formula = if sp.contains(&"AND") {
            Formula::And(
                sp[0]
                    .parse()
                    .map(Oper::Imm)
                    .unwrap_or_else(|_| Oper::Wire(sp[0])),
                sp[2],
            )
        } else if sp.contains(&"OR") {
            Formula::Or(sp[0], sp[2])
        } else if sp.contains(&"LSHIFT") {
            Formula::LShift(sp[0], sp[2].parse().unwrap())
        } else if sp.contains(&"RSHIFT") {
            Formula::RShift(sp[0], sp[2].parse().unwrap())
        } else if sp.contains(&"NOT") {
            Formula::Not(sp[1])
        } else {
            ff.parse()
                .map(Formula::Imm)
                .unwrap_or_else(|_| Formula::Id(ff))
        };
        map.insert(wire, formula);
    }
    map
}

fn compute(formulas: &HashMap<&str, Formula>) -> u64 {
    let mut vals = HashMap::new();
    let mut work = vec!["a"];
    while let Some(wire) = work.pop() {
        match formulas[wire].calc(&vals) {
            Res::Need(need) => {
                work.push(wire);
                work.extend(need);
            }
            Res::Val(val) => {
                vals.insert(wire, val);
            }
        }
    }
    vals["a"]
}

fn part1(inp: &str) -> u64 {
    let formulas = parse(inp);
    compute(&formulas)
}

fn part2(inp: &str) -> u64 {
    let mut formulas = parse(inp);
    let b = compute(&formulas);
    formulas.insert("b", Formula::Imm(b));
    compute(&formulas)
}

xaoc::xaoc!();
