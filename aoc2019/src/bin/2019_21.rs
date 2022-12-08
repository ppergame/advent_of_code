use aoc2019::intcode::*;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use z3::ast::Ast;

// Partial truthtable, absent entries are Don't Care
type TruthTable = HashMap<Vec<bool>, bool>;

struct Springbot {
    icprog: Vec<i64>,
    run_mode: bool,
    debug: bool,
    very_debug: bool,
    testcases: HashSet<String>,
    tt_cache: HashMap<String, Vec<TruthTable>>,
    term_letters: Vec<char>,
    term_offsets: Vec<usize>,
}

enum SpringbotStatus {
    Failure(String),
    Success(i64),
}

impl Springbot {
    fn new(inp: &str, run_mode: bool) -> Springbot {
        let prog = inp
            .split(',')
            .map(|i| i.parse().unwrap())
            .collect::<Vec<i64>>();

        Springbot {
            icprog: prog,
            run_mode,
            debug: false,
            very_debug: false,
            testcases: HashSet::new(),
            tt_cache: HashMap::new(),
            term_letters: Vec::new(),
            term_offsets: Vec::new(),
        }
    }

    fn search(&mut self) -> i64 {
        if !self.run_mode {
            self.term_letters = vec!['A', 'B', 'C', 'D'];
            self.term_offsets = vec![1, 2, 3, 4];
            return self.search_one().unwrap();
        }
        for pairs in ('E'..='I').enumerate().combinations(2) {
            self.term_letters = vec!['A', 'B', 'C', 'D'];
            self.term_offsets = vec![1, 2, 3, 4];
            for (offset, letter) in pairs {
                self.term_letters.push(letter);
                self.term_offsets.push(offset + 5);
            }
            if self.debug {
                println!("trying {:?} {:?}", self.term_letters, self.term_offsets);
            }
            if let Ok(i) = self.search_one() {
                return i;
            }
        }
        panic!("search failed");
    }

    fn search_one(&mut self) -> Result<i64, &'static str> {
        self.tt_cache.clear();
        if self.testcases.is_empty() {
            if let SpringbotStatus::Failure(case) = self.run("") {
                if self.debug {
                    println!("adding case {}", case);
                }
                self.testcases.insert(case);
            } else {
                unreachable!();
            }
        }
        loop {
            self.update_cache();
            let prog = self.make_prog()?;
            if self.very_debug {
                println!("{}", prog);
            }
            let case = match self.run(&prog) {
                SpringbotStatus::Success(i) => {
                    if self.debug {
                        println!("{}", prog);
                        let mut tts = self.testcases.iter().collect::<Vec<_>>();
                        tts.sort();
                        for tt in tts {
                            println!("{}", tt);
                        }
                    }
                    return Ok(i);
                }
                SpringbotStatus::Failure(case) => case,
            };
            if self.testcases.contains(&case) {
                panic!("got an existing test case");
            }
            if self.debug {
                println!("adding case {}", case);
            }
            self.testcases.insert(case);
        }
    }

    fn update_cache(&mut self) {
        //println!("{}", case);

        for case in &self.testcases {
            if self.tt_cache.contains_key(case) {
                continue;
            }

            if self.debug {
                println!("updating cache for {}", case);
            }

            let last_gap = case.rfind('.').unwrap();
            if self.debug {
                println!("last_gap {}", last_gap);
            }
            let is_hull = case
                .chars()
                .map(|x| match x {
                    '#' => true,
                    '.' => false,
                    _ => panic!("unknown case char"),
                })
                .collect::<Vec<bool>>();
            let mut res = Vec::<TruthTable>::new();

            let mut stack = vec![(0, TruthTable::new())];

            let grab_hull = |pos| -> Vec<bool> {
                self.term_offsets
                    .iter()
                    .map(|offset| *is_hull.get(pos + offset).unwrap_or(&true))
                    .collect()
            };

            while let Some((pos, tt)) = stack.pop() {
                // do not jump
                (|| {
                    // next step is a gap
                    if !is_hull[pos + 1] {
                        return;
                    }
                    let t = grab_hull(pos);
                    // truth table says jump, can't not jump
                    if let Some(true) = tt.get(&t) {
                        return;
                    }
                    let mut new_tt = tt.clone();
                    new_tt.insert(t, false);
                    stack.push((pos + 1, new_tt));
                })();
                // jump
                (|| {
                    // jump destination is a gap
                    if !is_hull[pos + 4] {
                        return;
                    }
                    let t = grab_hull(pos);
                    // truth table says do not jump, can't jump
                    if let Some(false) = tt.get(&t) {
                        return;
                    }
                    let mut new_tt = tt.clone();
                    new_tt.insert(t, true);
                    if pos + 4 > last_gap {
                        res.push(new_tt);
                        return;
                    }
                    stack.push((pos + 4, new_tt));
                })();
            }

            if res.is_empty() {
                panic!("could not find path");
            }
            if self.very_debug {
                for v in &res {
                    println!("{:?}", v);
                }
            }

            res.sort_by_key(|x| x.len());
            self.tt_cache.insert(case.to_string(), res);
        }
    }

    fn make_prog(&self) -> Result<String, &'static str> {
        'outer: for (i, tts) in self.tt_cache.values().multi_cartesian_product().enumerate() {
            if self.debug {
                println!("make_prog trying combo {}", i);
            }
            let mut combo = TruthTable::new();
            for tt in tts {
                let mut bad = false;
                for (row, jump) in tt {
                    match combo.get(row) {
                        Some(v) if v != jump => {
                            bad = true;
                            break;
                        }
                        _ => (),
                    }
                }
                if bad {
                    if self.debug {
                        println!("prog conflict");
                    }
                    continue 'outer;
                }
                combo.extend(tt.clone());
            }
            /*
            let mut rows = combo.iter().collect::<Vec<(&[bool; 4], &bool)>>();
            rows.sort();
            for (row, jump) in rows {
                for bit in row {
                    print!("{:06}", bit);
                }
                println!("{}", jump);
            }
            */
            if self.very_debug {
                println!("trying combo:");
                let mut kv = combo
                    .clone()
                    .into_iter()
                    .collect::<Vec<(Vec<bool>, bool)>>();
                kv.sort();
                for (row, jump) in kv {
                    if !jump {
                        continue;
                    }
                    for bit in row {
                        print!("{:06}", bit);
                    }
                    println!("=> {}", jump);
                }
            }
            let start = std::time::Instant::now();
            if let Ok(s) = self.tt_to_prog(&combo) {
                if self.debug {
                    println!("tt_to_prog took {}s", start.elapsed().as_secs());
                }
                return Ok(s);
            }
        }
        Err("make_prog failed to find combo")
    }

    /*
     * datatypes:
     *   instruction AND/OR/NOT
     *   input reg A..=D/T/J or A..=I/T/J
     *   output reg T/J
     * program variables:
     *   15x (instruction, input reg, output reg)
     * per tt entry variables:
     *   scanner inputs A..=D or A..=I
     *   16x T and J values after each instruction (index 0 -> initial t and j)
     * per tt entry constraints
     *   T0 == false, J0 == false
     *   for each instruction
     *     op1 = ite(s_op1 == A, scanner_A, ite(s_op1 == B, scanner_B, ite... ite(s_op1 == T, prev_T, prev_J)...))
     *     op2 = ite(s_op2 == T, prev_T, prev_J)
     *     res = ite(s_inst == AND, op1 AND op2, ite(s_inst == OR, op1 OR op2, NOT op1)
     *     next_T = ite(s_op2 == T, res, prev_T)
     *     next_J = ite(s_op2 == J, res, prev_J)
     *     J16 == tt_jump
     */
    fn tt_to_prog(&self, tt: &TruthTable) -> Result<String, &'static str> {
        let cfg = z3::Config::new();
        let ctx = z3::Context::new(&cfg);
        let solver = z3::Solver::new(&ctx);

        let (inst_sort, inst_const, inst_test) = z3::Sort::enumeration(
            &ctx,
            "Instruction".into(),
            &["AND".into(), "OR".into(), "NOT".into()],
        );
        let [_and_const, _or_const, _not_const] = <[_; 3]>::try_from(inst_const).ok().unwrap();
        let [and_test, or_test, not_test] = <[_; 3]>::try_from(inst_test).ok().unwrap();

        let (oreg_sort, oreg_const, oreg_test) =
            z3::Sort::enumeration(&ctx, "OReg".into(), &["OREG_T".into(), "OREG_J".into()]);
        let [_oreg_t_const, _oreg_j_const] = <[_; 2]>::try_from(oreg_const).ok().unwrap();
        let [oreg_t_test, oreg_j_test] = <[_; 2]>::try_from(oreg_test).ok().unwrap();
        let mut iregs = (0..self.term_letters.len())
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        iregs.push("T".to_string());
        iregs.push("J".to_string());
        let iregs = iregs.into_iter().map(|x| x.into()).collect::<Vec<_>>();
        let (ireg_sort, _ireg_const, mut ireg_test) =
            z3::Sort::enumeration(&ctx, "IReg".into(), &iregs);
        let ireg_tj_test = ireg_test.split_off(self.term_letters.len());
        let [ireg_t_test, ireg_j_test] = <[_; 2]>::try_from(ireg_tj_test).ok().unwrap();

        let inst_vars = (0..15)
            .map(|i| z3::ast::Datatype::new_const(&ctx, format!("inst_{}", i), &inst_sort))
            .collect::<Vec<_>>();
        let ireg_vars = (0..15)
            .map(|i| z3::ast::Datatype::new_const(&ctx, format!("ireg_{}", i), &ireg_sort))
            .collect::<Vec<_>>();
        let oreg_vars = (0..15)
            .map(|i| z3::ast::Datatype::new_const(&ctx, format!("oreg_{}", i), &oreg_sort))
            .collect::<Vec<_>>();

        fn test_enum<'a>(
            test: &z3::FuncDecl<'a>,
            var: &z3::ast::Datatype<'a>,
        ) -> z3::ast::Bool<'a> {
            test.apply(&[var]).as_bool().unwrap()
        }

        for (row, &jump) in tt.iter() {
            let scanner_vars = row
                .iter()
                .map(|&x| z3::ast::Bool::from_bool(&ctx, x))
                .collect::<Vec<_>>();
            let mut t_vals = Vec::<z3::ast::Bool>::new();
            let mut j_vals = Vec::<z3::ast::Bool>::new();
            t_vals.push(z3::ast::Bool::from_bool(&ctx, false));
            j_vals.push(z3::ast::Bool::from_bool(&ctx, false));
            for i in 0..15 {
                let next_t;
                let next_j;
                {
                    let prev_t = t_vals.last().unwrap();
                    let prev_j = j_vals.last().unwrap();
                    let mut op1 = test_enum(&ireg_t_test, &ireg_vars[i]).ite(prev_t, prev_j);
                    for j in 0..self.term_letters.len() {
                        op1 = test_enum(&ireg_test[j], &ireg_vars[i]).ite(&scanner_vars[j], &op1);
                    }
                    let op2 = test_enum(&oreg_t_test, &oreg_vars[i]).ite(prev_t, prev_j);
                    let mut res = test_enum(&or_test, &inst_vars[i])
                        .ite(&z3::ast::Bool::or(&ctx, &[&op1, &op2]), &op1.not());
                    res = test_enum(&and_test, &inst_vars[i])
                        .ite(&z3::ast::Bool::and(&ctx, &[&op1, &op2]), &res);
                    next_t = test_enum(&oreg_t_test, &oreg_vars[i]).ite(&res, prev_t);
                    next_j = test_enum(&oreg_j_test, &oreg_vars[i]).ite(&res, prev_j);
                }
                t_vals.push(next_t);
                j_vals.push(next_j);
            }
            solver.assert(
                &j_vals
                    .last()
                    .unwrap()
                    ._eq(&z3::ast::Bool::from_bool(&ctx, jump)),
            );
        }

        if !matches!(solver.check(), z3::SatResult::Sat) {
            if self.debug {
                println!("solver not sat");
            }
            return Err("not sat");
        }

        fn model_enum(model: &z3::Model, test: &z3::FuncDecl, var: &z3::ast::Datatype) -> bool {
            model
                .eval(&test_enum(test, var), true)
                .unwrap()
                .as_bool()
                .unwrap()
        }

        let inst_to_str = |model: &z3::Model, inst_var: &z3::ast::Datatype| -> &'static str {
            if model_enum(model, &and_test, inst_var) {
                return "AND";
            }
            if model_enum(model, &or_test, inst_var) {
                return "OR";
            }
            if model_enum(model, &not_test, inst_var) {
                return "NOT";
            }
            unreachable!();
        };

        let ireg_to_str = |model: &z3::Model, ireg_var: &z3::ast::Datatype| -> String {
            for (ireg_tester, term_letter) in std::iter::zip(&ireg_test, &self.term_letters) {
                if model_enum(model, ireg_tester, ireg_var) {
                    return term_letter.to_string();
                }
            }
            if model_enum(model, &ireg_t_test, ireg_var) {
                return "T".to_string();
            }
            if model_enum(model, &ireg_j_test, ireg_var) {
                return "J".to_string();
            }
            unreachable!();
        };

        let oreg_to_str = |model: &z3::Model, oreg_var: &z3::ast::Datatype| -> &'static str {
            if model_enum(model, &oreg_t_test, oreg_var) {
                return "T";
            }
            if model_enum(model, &oreg_j_test, oreg_var) {
                return "J";
            }
            unreachable!();
        };

        let model = solver.get_model().unwrap();
        let mut ret = itertools::join(
            (0..15).map(|i| {
                format!(
                    "{} {} {}",
                    inst_to_str(&model, &inst_vars[i]),
                    ireg_to_str(&model, &ireg_vars[i]),
                    oreg_to_str(&model, &oreg_vars[i]),
                )
            }),
            "\n",
        );
        ret.push('\n');
        Ok(ret)
    }

    // Returns failed test case "#####..#.########"
    fn run(&self, prog: &str) -> SpringbotStatus {
        let mut ic = Intcode::new_with_seq(&self.icprog);
        {
            let (output, status) = ic.collect_output();
            assert!(output.contains("Input instructions:"));
            assert!(matches!(status, IntcodeStatus::Input));
        }
        for c in prog
            .chars()
            .chain((if self.run_mode { "RUN" } else { "WALK" }).chars())
        {
            ic.input = Some(c as i64);
            let (_, status) = ic.collect_output();
            assert!(matches!(status, IntcodeStatus::Input));
        }
        ic.input = Some('\n' as i64);
        {
            let (output, status) = ic.collect_output();
            if self.very_debug {
                println!("{}", output);
            }
            assert!(matches!(status, IntcodeStatus::Halt));

            for (i, piece) in output.split("\n\n").enumerate() {
                match i {
                    0 => assert!(piece.contains("Walking...") || piece.contains("Running...")),
                    1 => {
                        if !piece.contains("Didn't make it across:") {
                            let res = ic.last_output.unwrap();
                            return SpringbotStatus::Success(res);
                        }
                    }
                    2 => {
                        let lines = piece.split_ascii_whitespace().collect::<Vec<&str>>();
                        assert!(lines[0] == ".................");
                        assert!(lines[1] == ".................");
                        assert!(lines[2] == "@................");
                        return SpringbotStatus::Failure(lines[3].to_string());
                    }
                    _ => break,
                }
            }
        }
        panic!();
    }
}

fn part1(inp: &str) -> i64 {
    let mut bot = Springbot::new(inp, false);
    bot.search()
}

fn part2(inp: &str) -> i64 {
    let mut bot = Springbot::new(inp, true);
    //bot.debug = true;
    //bot.very_debug = true;
    bot.search()
}

xaoc::xaoc!();
