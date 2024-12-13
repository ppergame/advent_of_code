use itertools::Itertools as _;
use num_rational::Rational64;
use num_traits::Signed as _;
use num_traits::Zero as _;
use sscanf::scanf;
use z3::ast::Ast as _;

struct Machine {
    ax: i64,
    ay: i64,
    bx: i64,
    by: i64,
    x: i64,
    y: i64,
}

impl Machine {
    fn parse(inp: &str) -> Vec<Self> {
        let mut ret = vec![];
        let mut it = inp.lines();
        loop {
            let (ax, ay) = scanf!(it.next().unwrap(), "Button A: X+{}, Y+{}", i64, i64).unwrap();
            let (bx, by) = scanf!(it.next().unwrap(), "Button B: X+{}, Y+{}", i64, i64).unwrap();
            let (x, y) = scanf!(it.next().unwrap(), "Prize: X={}, Y={}", i64, i64).unwrap();
            ret.push(Self {
                ax,
                ay,
                bx,
                by,
                x,
                y,
            });
            if it.next().is_none() {
                return ret;
            }
        }
    }

    fn scale(&mut self) {
        self.x += 10000000000000;
        self.y += 10000000000000;
    }

    #[allow(dead_code)]
    fn solve_z3(&self) -> Option<i64> {
        let cfg = z3::Config::new();
        let ctx = z3::Context::new(&cfg);
        let opt = z3::Optimize::new(&ctx);
        let a = z3::ast::Int::new_const(&ctx, "a");
        let b = z3::ast::Int::new_const(&ctx, "b");
        let x = z3::ast::Int::from_i64(&ctx, self.x);
        let y = z3::ast::Int::from_i64(&ctx, self.y);
        opt.assert(&(&a * self.ax + &b * self.bx)._eq(&x));
        opt.assert(&(&a * self.ay + &b * self.by)._eq(&y));
        let zero = z3::ast::Int::from_i64(&ctx, 0);
        opt.assert(&a.gt(&zero));
        opt.assert(&b.gt(&zero));
        let tt = &a * 3i64 + &b;
        opt.minimize(&tt);
        opt.check(&[]);
        let model = opt.get_model()?;
        let tokens = model.eval(&tt, false)?.as_i64();
        #[allow(clippy::let_and_return)]
        tokens
    }

    #[allow(dead_code)]
    fn solve_det(&self) -> Option<i64> {
        let ax = Rational64::from_integer(self.ax);
        let bx = Rational64::from_integer(self.bx);
        let ay = Rational64::from_integer(self.ay);
        let by = Rational64::from_integer(self.by);
        let x = Rational64::from_integer(self.x);
        let y = Rational64::from_integer(self.y);
        let det = ax * by - ay * bx;
        if det.is_zero() {
            return None;
        }
        let a = (by * x - bx * y) / det;
        let b = (ax * y - ay * x) / det;
        if a.is_integer() && b.is_integer() && a.is_positive() && b.is_positive() {
            Some(a.to_integer() * 3 + b.to_integer())
        } else {
            None
        }
    }
}

fn part1(inp: &str) -> i64 {
    let mm = Machine::parse(inp);
    let mut tokens = 0;
    for m in mm {
        tokens += (0..=100)
            .combinations_with_replacement(2)
            .flat_map(|v| [(v[0], v[1]), (v[1], v[0])])
            .filter_map(|(a, b)| {
                if a * m.ax + b * m.bx == m.x && a * m.ay + b * m.by == m.y {
                    Some(a * 3 + b)
                } else {
                    None
                }
            })
            .min()
            .unwrap_or(0);
    }
    tokens
}

fn part2(inp: &str) -> i64 {
    let mm = Machine::parse(inp);
    mm.into_iter()
        .filter_map(|mut m| {
            m.scale();
            m.solve_det()
        })
        .sum()
}

xaoc::xaoc!(
    sample = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"
);
