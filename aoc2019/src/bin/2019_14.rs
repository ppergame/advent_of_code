use regex::Regex;
use std::collections::HashMap;
use z3::ast::Ast;

#[derive(Debug)]
struct Reaction {
    inputs: Vec<(String, u64)>,
    output: u64,
}

lazy_static::lazy_static! {
    static ref RE_RE: Regex = Regex::new(r"(\d+) ([A-Z]+)").unwrap();
}

fn parse_reactions(inp: &str) -> HashMap<String, Reaction> {
    let mut reactions = HashMap::new();
    for line in inp.lines() {
        let mut inputs = Vec::new();
        let mut iter = RE_RE.captures_iter(line).peekable();
        while let Some(cap) = iter.next() {
            let (count, name) = (cap[1].parse().unwrap(), cap[2].to_string());
            if iter.peek().is_some() {
                inputs.push((name, count));
            } else {
                reactions.insert(
                    name,
                    Reaction {
                        inputs,
                        output: count,
                    },
                );
                break;
            }
        }
    }
    assert_eq!(reactions["FUEL"].output, 1);
    reactions
}

pub struct State<'a> {
    optimize: z3::Optimize<'a>,
    rcounts: HashMap<String, z3::ast::Int<'a>>,
    used_ore: z3::ast::Int<'a>,
}

pub fn build<'a, 'b>(ctx: &'a z3::Context, inp: &str) -> State<'b>
where
    'a: 'b,
{
    let reactions = parse_reactions(inp);

    let optimize = z3::Optimize::new(ctx);

    let mut rcounts = HashMap::<String, z3::ast::Int>::new();
    let mut used_terms = HashMap::<String, Vec<z3::ast::Int>>::new();

    // Tabulate used materials
    for (output_name, reaction) in &reactions {
        let rcount = z3::ast::Int::new_const(ctx, format!("rcount_{}", output_name));
        for (input_name, input_count) in &reaction.inputs {
            let used_vec = used_terms.entry(input_name.to_string()).or_default();
            let term =
                z3::ast::Int::mul(ctx, &[&rcount, &z3::ast::Int::from_u64(ctx, *input_count)]);
            used_vec.push(term);
        }
        optimize.assert(&rcount.ge(&z3::ast::Int::from_u64(ctx, 0)));
        rcounts.insert(output_name.to_string(), rcount);
    }

    let mut used_ore = None;
    // let mut used = HashMap::<String, Vec<z3::ast::Int>>::new();
    // rcount*output_count >= used
    for (name, used_vec) in used_terms {
        let term = z3::ast::Int::add(ctx, &used_vec.iter().collect::<Vec<_>>());
        let used_var = z3::ast::Int::new_const(ctx, format!("used_{}", name));
        optimize.assert(&used_var._eq(&term));
        if name == "ORE" {
            used_ore = Some(used_var);
            continue;
        }
        optimize.assert(&used_var.le(&z3::ast::Int::mul(
            ctx,
            &[
                &rcounts[&name],
                &z3::ast::Int::from_u64(ctx, reactions[&name].output),
            ],
        )));
    }

    let used_ore = used_ore.unwrap();

    State {
        optimize,
        rcounts,
        used_ore,
    }
}

// "let x" is necessary to keep lifetimes straight
#[allow(clippy::let_and_return)]
fn part1(inp: &str) -> u64 {
    let cfg = z3::Config::new();
    let ctx = z3::Context::new(&cfg);
    let state = build(&ctx, inp);
    let optimize = state.optimize;
    optimize.assert(&state.rcounts["FUEL"]._eq(&z3::ast::Int::from_u64(&ctx, 1)));
    optimize.minimize(&state.used_ore);
    assert!(matches!(optimize.check(&[]), z3::SatResult::Sat));
    let model = optimize.get_model().unwrap();
    let x = model.eval(&state.used_ore, true).unwrap().as_u64().unwrap();
    x
}

#[allow(clippy::let_and_return)]
fn part2(inp: &str) -> u64 {
    let cfg = z3::Config::new();
    let ctx = z3::Context::new(&cfg);
    let state = build(&ctx, inp);
    let optimize = state.optimize;
    optimize.assert(
        &state
            .used_ore
            .le(&z3::ast::Int::from_u64(&ctx, 1000000000000)),
    );
    optimize.maximize(&state.rcounts["FUEL"]);
    assert!(matches!(optimize.check(&[]), z3::SatResult::Sat));
    let model = optimize.get_model().unwrap();
    let x = model
        .eval(&state.rcounts["FUEL"], true)
        .unwrap()
        .as_u64()
        .unwrap();
    x
}

xaoc::xaoc!();
