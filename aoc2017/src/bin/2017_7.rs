use bimap::BiMap;
use itertools::Itertools;
use sscanf::scanf;
use std::collections::HashSet;
use z3::ast::Ast;

#[derive(Debug, Default)]
struct Names {
    names: BiMap<String, usize>,
    next_idx: usize,
}

impl Names {
    // fn idx(&self, name: &str) -> usize {
    //     *self.names.get_by_left(name).unwrap()
    // }

    fn name(&self, idx: usize) -> &str {
        self.names.get_by_right(&idx).unwrap()
    }

    fn idx_mut(&mut self, name: &str) -> usize {
        match self.names.get_by_left(name) {
            Some(idx) => *idx,
            None => {
                let ret = self.next_idx;
                self.names.insert(name.to_string(), ret);
                self.next_idx += 1;
                ret
            }
        }
    }
}

#[derive(Debug)]
struct Tower {
    names: Names,
    root: usize,
    weights: Vec<i64>,
    holds: Vec<Vec<usize>>,
}

impl Tower {
    fn parse(inp: &str) -> Self {
        let mut names = Names::default();
        let mut weights = vec![];
        let mut holds = vec![];
        let mut is_child = HashSet::new();
        for line in inp.lines() {
            let mut sp = line.split(" -> ");
            let left = sp.next().unwrap();
            let (base, weight) = scanf!(left, "{} ({})", str, i64).unwrap();
            let base = names.idx_mut(base);
            if weights.len() <= base {
                weights.resize_with(base + 1, Default::default);
                holds.resize_with(base + 1, Default::default);
            }
            if let Some(right) = sp.next() {
                let children = right
                    .split(',')
                    .map(|s| names.idx_mut(s.trim()))
                    .collect_vec();
                is_child.extend(children.iter().copied());
                holds[base] = children;
            }
            weights[base] = weight;
        }
        let root = (0..names.next_idx)
            .find(|prog| !is_child.contains(prog))
            .unwrap();
        Tower {
            names,
            root,
            weights,
            holds,
        }
    }
}

fn part1(inp: &str) -> String {
    let tower = Tower::parse(inp);
    tower.names.name(tower.root).to_owned()
}

fn part2(inp: &str) -> i64 {
    let tower = Tower::parse(inp);
    let cfg = z3::Config::new();
    let ctx = z3::Context::new(&cfg);
    let solver = z3::Solver::new(&ctx);
    let mut params = z3::Params::new(&ctx);
    params.set_bool("parallel.enable", true);
    params.set_u32("threads", 32);
    solver.set_params(&params);
    let total_weight_vars = (0..tower.names.next_idx)
        .map(|idx| z3::ast::Int::new_const(&ctx, format!("total_weight_{}", idx)))
        .collect_vec();
    let adj_vars = (0..tower.names.next_idx)
        .map(|idx| z3::ast::Int::new_const(&ctx, format!("adj_{}", idx)))
        .collect_vec();
    let use_adj_vars = (0..tower.names.next_idx)
        .map(|idx| z3::ast::Int::new_const(&ctx, format!("use_adj_{}", idx)))
        .collect_vec();
    for (idx, adj_var) in adj_vars.iter().enumerate() {
        solver.assert(&adj_var.ge(&z3::ast::Int::from_i64(&ctx, -tower.weights[idx])));
    }
    for use_adj_var in &use_adj_vars {
        solver.assert(&z3::ast::Bool::or(
            &ctx,
            &[
                &use_adj_var._eq(&z3::ast::Int::from_i64(&ctx, 0)),
                &use_adj_var._eq(&z3::ast::Int::from_i64(&ctx, 1)),
            ],
        ));
    }
    for idx in 0..tower.names.next_idx {
        let mut sum = z3::ast::Int::add(
            &ctx,
            &[
                &z3::ast::Int::from_i64(&ctx, tower.weights[idx]),
                &z3::ast::Int::mul(&ctx, &[&adj_vars[idx], &use_adj_vars[idx]]),
            ],
        );
        for &child in &tower.holds[idx] {
            sum = z3::ast::Int::add(&ctx, &[&sum, &total_weight_vars[child]]);
        }
        tower.holds[idx]
            .iter()
            .tuple_windows()
            .for_each(|(a, b)| solver.assert(&total_weight_vars[*a]._eq(&total_weight_vars[*b])));
        solver.assert(&z3::ast::Ast::_eq(&total_weight_vars[idx], &sum));
    }
    solver.assert(
        &z3::ast::Int::add(&ctx, &use_adj_vars.iter().collect_vec())
            ._eq(&z3::ast::Int::from_i64(&ctx, 1)),
    );
    assert!(matches!(solver.check(), z3::SatResult::Sat));
    let model = solver.get_model().unwrap();
    for (idx, use_adj_var) in use_adj_vars.iter().enumerate() {
        if model.eval(use_adj_var, false).unwrap().as_i64().unwrap() == 1 {
            return tower.weights[idx]
                + model.eval(&adj_vars[idx], false).unwrap().as_i64().unwrap();
        }
    }
    unreachable!();
}

xaoc::xaoc!();
