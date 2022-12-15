use itertools::Itertools;
use sscanf::scanf;
use z3::ast::{Ast, Bool, Int};
use z3::SatResult;

trait Asserter<'ctx> {
    fn assert(&self, ast: &Bool<'ctx>);
}

impl<'ctx> Asserter<'ctx> for z3::Optimize<'ctx> {
    fn assert(&self, ast: &Bool<'ctx>) {
        self.assert(ast);
    }
}

impl<'ctx> Asserter<'ctx> for z3::Solver<'ctx> {
    fn assert(&self, ast: &Bool<'ctx>) {
        self.assert(ast);
    }
}

fn parse_rev<'ctx>(
    ctx: &'ctx z3::Context,
    inp: &str,
    ass: &impl Asserter<'ctx>,
) -> (Int<'ctx>, Int<'ctx>) {
    let in_range_row = Int::new_const(ctx, "in_range_row");
    let in_range_col = Int::new_const(ctx, "in_range_col");
    let mut hits = vec![];
    for line in inp.lines() {
        let (col, row, bcol, brow) = scanf!(
            line,
            "Sensor at x={}, y={}: closest beacon is at x={}, y={}",
            i64,
            i64,
            i64,
            i64
        )
        .unwrap();
        let dist = (row - brow).abs() + (col - bcol).abs();
        hits.push(
            Int::add(
                ctx,
                &[
                    &z3_abs(
                        ctx,
                        &Int::sub(ctx, &[&in_range_row, &Int::from_i64(ctx, row)]),
                    ),
                    &z3_abs(
                        ctx,
                        &Int::sub(ctx, &[&in_range_col, &Int::from_i64(ctx, col)]),
                    ),
                ],
            )
            .le(&Int::from_i64(ctx, dist)),
        );
    }
    ass.assert(&Bool::or(ctx, &hits.iter().collect_vec()));
    (in_range_row, in_range_col)
}

fn parse<'ctx>(
    ctx: &'ctx z3::Context,
    inp: &str,
    ass: &impl Asserter<'ctx>,
) -> (Int<'ctx>, Int<'ctx>) {
    let out_of_range_row = Int::new_const(ctx, "out_of_range_row");
    let out_of_range_col = Int::new_const(ctx, "out_of_range_col");
    for line in inp.lines() {
        let (col, row, bcol, brow) = scanf!(
            line,
            "Sensor at x={}, y={}: closest beacon is at x={}, y={}",
            i64,
            i64,
            i64,
            i64
        )
        .unwrap();
        let dist = (row - brow).abs() + (col - bcol).abs();
        ass.assert(
            &Int::add(
                ctx,
                &[
                    &z3_abs(
                        ctx,
                        &Int::sub(ctx, &[&out_of_range_row, &Int::from_i64(ctx, row)]),
                    ),
                    &z3_abs(
                        ctx,
                        &Int::sub(ctx, &[&out_of_range_col, &Int::from_i64(ctx, col)]),
                    ),
                ],
            )
            .gt(&Int::from_i64(ctx, dist)),
        );
    }
    (out_of_range_row, out_of_range_col)
}

fn z3_abs<'a>(ctx: &'a z3::Context, v: &Int<'a>) -> Int<'a> {
    let positive = v.ge(&Int::from_i64(ctx, 0));
    positive.ite(v, &v.unary_minus())
}

fn part1(inp: &str) -> i64 {
    let is_sample = inp.lines().count() < 15;
    let cfg = z3::Config::new();
    let ctx = z3::Context::new(&cfg);
    let opt = z3::Optimize::new(&ctx);
    let (out_of_range_row, out_of_range_col) = parse(&ctx, inp, &opt);
    let (in_range_row, in_range_col) = parse_rev(&ctx, inp, &opt);
    opt.assert(&out_of_range_row._eq(&Int::from_i64(&ctx, if is_sample { 10 } else { 2000000 })));
    opt.assert(&in_range_row._eq(&Int::from_i64(&ctx, if is_sample { 10 } else { 2000000 })));
    opt.maximize(&in_range_col);
    let mut found_col = i64::MAX;
    let mut count = 0;
    while matches!(
        opt.check(&[in_range_col.lt(&Int::from_i64(&ctx, found_col))]),
        SatResult::Sat
    ) {
        let to = opt
            .get_model()
            .unwrap()
            .eval(&in_range_col, false)
            .unwrap()
            .as_i64()
            .unwrap();
        assert!(matches!(
            opt.check(&[out_of_range_col.lt(&Int::from_i64(&ctx, to))]),
            SatResult::Sat
        ));
        let from = opt
            .get_model()
            .unwrap()
            .eval(&out_of_range_col, false)
            .unwrap()
            .as_i64()
            .unwrap();
        count += to - from - 1;
        found_col = from;
    }
    count
}

fn part2(inp: &str) -> i64 {
    let is_sample = inp.lines().count() < 15;
    let bounds = if is_sample { 20 } else { 4000000 };
    let cfg = z3::Config::new();
    let ctx = z3::Context::new(&cfg);
    let solver = z3::Solver::new(&ctx);
    let (out_of_range_row, out_of_range_col) = parse(&ctx, inp, &solver);
    solver.assert(&out_of_range_row.ge(&Int::from_i64(&ctx, 0)));
    solver.assert(&out_of_range_row.le(&Int::from_i64(&ctx, bounds)));
    solver.assert(&out_of_range_col.ge(&Int::from_i64(&ctx, 0)));
    solver.assert(&out_of_range_col.le(&Int::from_i64(&ctx, bounds)));
    assert!(matches!(solver.check(), SatResult::Sat));
    let model = solver.get_model().unwrap();
    let row = model
        .eval(&out_of_range_row, false)
        .unwrap()
        .as_i64()
        .unwrap();
    let col = model
        .eval(&out_of_range_col, false)
        .unwrap()
        .as_i64()
        .unwrap();
    col * 4000000 + row
}

xaoc::xaoc!();
