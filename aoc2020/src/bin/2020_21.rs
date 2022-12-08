use bimap::BiHashMap;
use itertools::Itertools;
use std::collections::{BTreeSet, HashSet};
use z3::ast::Ast;

struct Food {
    ingr: BTreeSet<String>,
    allerg: BTreeSet<String>,
}

struct Foods {
    f: Vec<Food>,
}

impl Foods {
    fn parse(inp: &str) -> Foods {
        let mut f = Vec::new();
        for line in inp.lines() {
            let (ii, ll) = line.split_once(" (contains ").unwrap();
            let ingr = ii.split(' ').map(|x| x.to_string()).collect();
            let allerg = ll
                .trim_end_matches(')')
                .split(", ")
                .map(|x| x.to_string())
                .collect();
            f.push(Food { ingr, allerg });
        }
        Foods { f }
    }

    fn bimaps(&self) -> (BiHashMap<usize, String>, BiHashMap<usize, String>) {
        let mut temp = HashSet::new();
        for food in &self.f {
            for s in &food.ingr {
                temp.insert(s);
            }
        }
        let ingrs = temp
            .into_iter()
            .sorted()
            .cloned()
            .enumerate()
            .collect::<BiHashMap<_, _>>();
        let mut temp = HashSet::new();
        for food in &self.f {
            for s in &food.allerg {
                temp.insert(s);
            }
        }
        let allergs = temp
            .into_iter()
            .sorted()
            .cloned()
            .enumerate()
            .collect::<BiHashMap<_, _>>();
        (ingrs, allergs)
    }

    fn solve(&self) -> BiHashMap<String, String> {
        let cfg = z3::Config::new();
        let ctx = z3::Context::new(&cfg);
        let solver = z3::Solver::new(&ctx);

        let (ingrs, allergs) = self.bimaps();
        let allerg_vars = allergs
            .right_values()
            .map(|s| z3::ast::Int::new_const(&ctx, format!("allerg_{}", s)))
            .collect::<Vec<_>>();
        solver.assert(&z3::ast::Ast::distinct(
            &ctx,
            &allerg_vars.iter().collect::<Vec<_>>(),
        ));
        for food in &self.f {
            for allerg in &food.allerg {
                let var = &allerg_vars[*allergs.get_by_right(allerg).unwrap()];
                let checks = &food
                    .ingr
                    .iter()
                    .map(|s| {
                        var._eq(&z3::ast::Int::from_u64(
                            &ctx,
                            *ingrs.get_by_right(s).unwrap() as u64,
                        ))
                    })
                    .collect::<Vec<_>>();
                solver.assert(&z3::ast::Bool::or(&ctx, &checks.iter().collect::<Vec<_>>()));
            }
        }
        assert!(matches!(solver.check(), z3::SatResult::Sat));
        let model = solver.get_model().unwrap();
        let mut ret = BiHashMap::new();
        for (allerg_i, allerg_var) in allerg_vars.iter().enumerate() {
            let ingr_i = model.eval(allerg_var, true).unwrap().as_u64().unwrap() as usize;
            ret.insert(
                allergs.get_by_left(&allerg_i).unwrap().to_owned(),
                ingrs.get_by_left(&ingr_i).unwrap().to_owned(),
            );
        }
        ret
    }
}

fn part1(inp: &str) -> usize {
    let f = Foods::parse(inp);
    let allerg_to_ingr = f.solve();
    let mut count = 0;
    for food in &f.f {
        for ingr in &food.ingr {
            if !allerg_to_ingr.contains_right(ingr) {
                count += 1;
            }
        }
    }
    count
}

fn part2(inp: &str) -> String {
    let f = Foods::parse(inp);
    let allerg_to_ingr = f.solve();
    allerg_to_ingr
        .iter()
        .sorted_by_key(|(allerg, _)| allerg.to_owned())
        .map(|(_, ingr)| ingr)
        .join(",")
}

xaoc::xaoc!();
