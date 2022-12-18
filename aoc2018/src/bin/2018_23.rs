use itertools::Itertools;
use sscanf::scanf;
use z3::ast::{Ast, Bool, Int};

struct Bot {
    pos: (i64, i64, i64),
    r: i64,
}

impl Bot {
    fn parse(line: &str) -> Self {
        let (x, y, z, r) = scanf!(line, "pos=<{},{},{}>, r={}", i64, i64, i64, i64).unwrap();
        Self { pos: (x, y, z), r }
    }

    fn dist(&self, other: &Bot) -> i64 {
        let (x, y, z) = self.pos;
        let (ox, oy, oz) = other.pos;
        (x - ox).abs() + (y - oy).abs() + (z - oz).abs()
    }
}

fn part1(inp: &str) -> i64 {
    let bots = inp.lines().map(Bot::parse).collect::<Vec<_>>();
    let strongest = bots.iter().max_by_key(|bot| bot.r).unwrap();
    let mut ret = 0;
    for bot in &bots {
        if strongest.dist(bot) <= strongest.r {
            ret += 1;
        }
    }
    ret
}

fn part2(inp: &str) -> i64 {
    let bots = inp.lines().map(Bot::parse).collect::<Vec<_>>();
    let cfg = z3::Config::new();
    let ctx = z3::Context::new(&cfg);
    let opt = z3::Optimize::new(&ctx);
    let (t, u, v, w) = (
        Int::new_const(&ctx, "t"),
        Int::new_const(&ctx, "u"),
        Int::new_const(&ctx, "v"),
        Int::new_const(&ctx, "w"),
    );
    let mut in_ranges = vec![];
    for bot in bots {
        let (x, y, z) = bot.pos;
        let (bt, bu, bv, bw) = (-x + y + z, x - y + z, x + y - z, x + y + z);
        let mut bot_in_range = vec![];
        for (b, var) in [
            (bt, t.clone()),
            (bu, u.clone()),
            (bv, v.clone()),
            (bw, w.clone()),
        ] {
            bot_in_range.push(Bool::and(
                &ctx,
                &[
                    &var.ge(&Int::from_i64(&ctx, b - bot.r)),
                    &var.le(&Int::from_i64(&ctx, b + bot.r)),
                ],
            ));
        }
        let in_range = Bool::and(&ctx, &bot_in_range.iter().collect::<Vec<_>>())
            .ite(&Int::from_i64(&ctx, 1), &Int::from_i64(&ctx, 0));
        in_ranges.push(in_range);
    }
    let in_range = Int::new_const(&ctx, "in_range");
    opt.assert(&in_range._eq(&Int::add(&ctx, &in_ranges.iter().collect_vec())));
    opt.maximize(&in_range);
    assert!(matches!(opt.check(&[]), z3::SatResult::Sat));
    let (t, u, v) = (
        opt.get_model()
            .unwrap()
            .eval(&t, false)
            .unwrap()
            .as_i64()
            .unwrap(),
        opt.get_model()
            .unwrap()
            .eval(&u, false)
            .unwrap()
            .as_i64()
            .unwrap(),
        opt.get_model()
            .unwrap()
            .eval(&v, false)
            .unwrap()
            .as_i64()
            .unwrap(),
    );
    ((t + u).abs() + (t + v).abs() + (u + v).abs()) / 2
}

xaoc::xaoc!(
    sample_idx = 3,
    sample2 = r#"pos=<10,12,12>, r=2
pos=<12,14,12>, r=2
pos=<16,12,12>, r=4
pos=<14,14,14>, r=6
pos=<50,50,50>, r=200
pos=<10,10,10>, r=5"#
);
