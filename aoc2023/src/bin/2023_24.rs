use itertools::Itertools as _;
use nalgebra::{vector, Vector3};
use z3::ast::Ast as _;

type Coord = Vector3<f64>;

#[derive(Debug)]
struct Stone {
    pos: Coord,
    vel: Coord,
}

impl Stone {
    fn line(&self) -> Line {
        Line {
            a: -self.vel[1],
            b: self.vel[0],
            c: self.vel[1] * self.pos[0] - self.vel[0] * self.pos[1],
        }
    }
}

struct Line {
    a: f64,
    b: f64,
    c: f64,
}

fn intersect(l1: Line, l2: Line) -> Option<(f64, f64)> {
    let det = l2.a * l1.b - l1.a * l2.b;
    if det == 0.0 {
        return None;
    }
    let x = (l2.b * l1.c - l1.b * l2.c) / det;
    let y = (l1.a * l2.c - l2.a * l1.c) / det;
    Some((x, y))
}

struct Stones(Vec<Stone>);

impl Stones {
    fn parse(inp: &str) -> Self {
        Stones(
            inp.lines()
                .map(|l| {
                    let (pos, vel) = l.split_once(" @ ").unwrap();
                    let tt = |s: &str| {
                        let (x, y, z) = s
                            .split(',')
                            .map(|s| s.trim().parse().unwrap())
                            .collect_tuple()
                            .unwrap();
                        vector![x, y, z]
                    };
                    Stone {
                        pos: tt(pos),
                        vel: tt(vel),
                    }
                })
                .collect(),
        )
    }
}

fn part1(inp: &str) -> usize {
    let stones = Stones::parse(inp);
    let min;
    let max;
    if stones.0.len() < 10 {
        min = 7.0;
        max = 27.0;
    } else {
        min = 200000000000000.0;
        max = 400000000000000.0;
    }
    let mut ret = 0;
    for (a, b) in stones.0.iter().tuple_combinations() {
        let l1 = a.line();
        let l2 = b.line();
        let Some((x, y)) = intersect(l1, l2) else {
            continue;
        };
        if x < min || x > max || y < min || y > max {
            continue;
        }
        let t1 = (x - a.pos[0]) / a.vel[0];
        let t2 = (x - b.pos[0]) / b.vel[0];
        if t1 < 0.0 || t2 < 0.0 {
            continue;
        }
        ret += 1;
    }
    // todo!("{ret}");
    ret
}

fn real_from_f64(ctx: &z3::Context, f: f64) -> z3::ast::Real {
    assert_eq!(f.trunc(), f);
    z3::ast::Real::from_int(&z3::ast::Int::from_i64(ctx, f as i64))
}

fn real_to_f64(ctx: &z3::Context, r: &z3::ast::Real) -> f64 {
    unsafe {
        z3_sys::Z3_get_numeral_double(
            *(ctx as *const z3::Context as *const z3_sys::Z3_context),
            r.get_z3_ast(),
        )
    }
}

fn part2(inp: &str) -> i64 {
    let stones = Stones::parse(inp);
    let cfg = z3::Config::new();
    let ctx = z3::Context::new(&cfg);
    let solver = z3::Solver::new(&ctx);
    let intercepts = (0..stones.0.len())
        .map(|i| z3::ast::Real::new_const(&ctx, format!("intercept_{}", i)))
        .collect_vec();
    let x = z3::ast::Real::new_const(&ctx, "x");
    let y = z3::ast::Real::new_const(&ctx, "y");
    let z = z3::ast::Real::new_const(&ctx, "z");
    let vx = z3::ast::Real::new_const(&ctx, "vx");
    let vy = z3::ast::Real::new_const(&ctx, "vy");
    let vz = z3::ast::Real::new_const(&ctx, "vz");
    for (t, s) in intercepts.iter().zip(&stones.0) {
        solver.assert(&t.ge(&real_from_f64(&ctx, 0.0)));
        solver.assert(
            &(t * &vx + &x)
                ._eq(&(t * real_from_f64(&ctx, s.vel[0]) + real_from_f64(&ctx, s.pos[0]))),
        );
        solver.assert(
            &(t * &vy + &y)
                ._eq(&(t * real_from_f64(&ctx, s.vel[1]) + real_from_f64(&ctx, s.pos[1]))),
        );
        solver.assert(
            &(t * &vz + &z)
                ._eq(&(t * real_from_f64(&ctx, s.vel[2]) + real_from_f64(&ctx, s.pos[2]))),
        );
    }
    assert!(matches!(solver.check(), z3::SatResult::Sat));
    let model = solver.get_model().unwrap();
    let x = real_to_f64(&ctx, &model.eval(&x, true).unwrap());
    let y = real_to_f64(&ctx, &model.eval(&y, true).unwrap());
    let z = real_to_f64(&ctx, &model.eval(&z, true).unwrap());
    // let vx = real_to_f64(&ctx, &model.eval(&vx, true).unwrap());
    // let vy = real_to_f64(&ctx, &model.eval(&vy, true).unwrap());
    // let vz = real_to_f64(&ctx, &model.eval(&vz, true).unwrap());
    // eprintln!("{x} {y} {z} {vx} {vy} {vz}");
    (x + y + z) as i64
}

xaoc::xaoc!(
    sample = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3"
);
