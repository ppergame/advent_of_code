use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
    fmt::Debug,
};

use duplicate::duplicate;
use itertools::Itertools;
use sscanf::scanf;
use z3::ast::Int;

#[derive(Copy, Clone)]
struct Bot {
    pos: Coord3,
    r: i64,
}

impl Bot {
    fn parse(line: &str) -> Self {
        let (x, y, z, r) = scanf!(line, "pos=<{},{},{}>, r={}", i64, i64, i64, i64).unwrap();
        Self {
            pos: (x, y, z).into(),
            r,
        }
    }
}

fn part1(inp: &str) -> i64 {
    let bots = inp.lines().map(Bot::parse).collect_vec();
    let strongest = bots.iter().max_by_key(|bot| bot.r).unwrap();
    let mut ret = 0;
    for bot in &bots {
        if strongest.pos.dist(bot.pos) <= strongest.r {
            ret += 1;
        }
    }
    ret
}

#[derive(Debug, Copy, Clone)]
struct Coord3(i64, i64, i64);

impl From<(i64, i64, i64)> for Coord3 {
    fn from(value: (i64, i64, i64)) -> Self {
        Self(value.0, value.1, value.2)
    }
}

impl From<Coord3> for (i64, i64, i64) {
    fn from(value: Coord3) -> Self {
        (value.0, value.1, value.2)
    }
}

impl From<Coord4> for Coord3 {
    fn from(value: Coord4) -> Self {
        let (t, u, v, w) = value.into();
        assert_eq!(t % 2, u % 2);
        assert_eq!(u % 2, v % 2);
        assert_eq!(v % 2, w % 2);
        Coord3((t + v) / 2, (u - v) / 2, (t - u) / 2)
    }
}

impl Coord3 {
    fn dist(self, other: Self) -> i64 {
        let (x, y, z) = self.into();
        let (ox, oy, oz) = other.into();
        (x - ox).abs() + (y - oy).abs() + (z - oz).abs()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Coord4(i64, i64, i64, i64);

impl From<(i64, i64, i64, i64)> for Coord4 {
    fn from(value: (i64, i64, i64, i64)) -> Self {
        Self(value.0, value.1, value.2, value.3)
    }
}

impl From<Coord4> for (i64, i64, i64, i64) {
    fn from(value: Coord4) -> Self {
        (value.0, value.1, value.2, value.3)
    }
}

impl From<Coord3> for Coord4 {
    fn from(value: Coord3) -> Self {
        let (x, y, z) = value.into();
        Coord4(x + y + z, x + y - z, x - y - z, x - y + z)
    }
}

impl Coord4 {
    #[allow(dead_code)]
    fn dist(self, other: Self) -> i64 {
        let (t, u, v, w) = self.into();
        let (ot, ou, ov, ow) = other.into();
        (t - ot).abs() + (u - ou).abs() + (v - ov).abs() + (w - ow).abs()
    }
}

#[derive(Debug, Copy, Clone)]
struct Region {
    min: Coord4,
    max: Coord4,
}

impl From<Bot> for Region {
    fn from(bot: Bot) -> Self {
        let c4 = Coord4::from(bot.pos);
        let min = Coord4(c4.0 - bot.r, c4.1 - bot.r, c4.2 - bot.r, c4.3 - bot.r);
        let max = Coord4(
            c4.0 + bot.r + 1,
            c4.1 + bot.r + 1,
            c4.2 + bot.r + 1,
            c4.3 + bot.r + 1,
        );
        Self { min, max }
    }
}

impl Region {
    #[allow(dead_code)]
    fn contains(&self, c: Coord4) -> bool {
        (self.min.0..self.max.0).contains(&c.0)
            && (self.min.1..self.max.1).contains(&c.1)
            && (self.min.2..self.max.2).contains(&c.2)
            && (self.min.3..self.max.3).contains(&c.3)
    }

    fn intersects(&self, my_bots: &[usize], bots: &[Region]) -> Vec<usize> {
        let mut ret = vec![];
        for &idx in my_bots {
            let bot = &bots[idx];
            if ((self.min.0..self.max.0).contains(&bot.min.0)
                || (bot.min.0..bot.max.0).contains(&self.min.0))
                && ((self.min.1..self.max.1).contains(&bot.min.1)
                    || (bot.min.1..bot.max.1).contains(&self.min.1))
                && ((self.min.2..self.max.2).contains(&bot.min.2)
                    || (bot.min.2..bot.max.2).contains(&self.min.2))
                && ((self.min.3..self.max.3).contains(&bot.min.3)
                    || (bot.min.3..bot.max.3).contains(&self.min.3))
            {
                ret.push(idx);
            }
        }
        ret
    }

    #[allow(clippy::let_and_return)]
    fn origin_closest_3d(&self) -> i64 {
        let cfg = z3::Config::new();
        let ctx = z3::Context::new(&cfg);
        let opt = z3::Optimize::new(&ctx);
        let x = Int::new_const(&ctx, "x");
        let y = Int::new_const(&ctx, "y");
        let z = Int::new_const(&ctx, "z");
        let t = Int::add(&ctx, &[&x, &y, &z]);
        let u = Int::add(&ctx, &[&x, &y, &z.unary_minus()]);
        let v = Int::add(&ctx, &[&x, &y.unary_minus(), &z.unary_minus()]);
        let w = Int::add(&ctx, &[&x, &y.unary_minus(), &z]);
        let dist = Int::add(
            &ctx,
            &[&z3_abs(&ctx, &x), &z3_abs(&ctx, &y), &z3_abs(&ctx, &z)],
        );

        opt.assert(&t.ge(&Int::from_i64(&ctx, self.min.0)));
        opt.assert(&t.lt(&Int::from_i64(&ctx, self.max.0)));
        opt.assert(&u.ge(&Int::from_i64(&ctx, self.min.1)));
        opt.assert(&u.lt(&Int::from_i64(&ctx, self.max.1)));
        opt.assert(&v.ge(&Int::from_i64(&ctx, self.min.2)));
        opt.assert(&v.lt(&Int::from_i64(&ctx, self.max.2)));
        opt.assert(&w.ge(&Int::from_i64(&ctx, self.min.3)));
        opt.assert(&w.lt(&Int::from_i64(&ctx, self.max.3)));
        opt.minimize(&dist);
        assert!(matches!(opt.check(&[]), z3::SatResult::Sat));
        let model = opt.get_model().unwrap();
        let ret = model.eval(&dist, false).unwrap().as_i64().unwrap();
        ret
    }
}

fn z3_abs<'a>(ctx: &'a z3::Context, x: &Int<'a>) -> Int<'a> {
    let zero = Int::from_i64(ctx, 0);
    x.ge(&zero).ite(x, &x.unary_minus())
}

struct Item {
    reg: Region,
    my_bots: Vec<usize>,
}

impl Debug for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Item")
            .field("reg", &self.reg)
            .field("bots", &self.my_bots.len())
            .finish()
    }
}

impl Item {
    fn initial(bots: &[Region]) -> Vec<ItemOrder> {
        let mut mins = (i64::MAX, i64::MAX, i64::MAX, i64::MAX);
        let mut maxs = (i64::MIN, i64::MIN, i64::MIN, i64::MIN);
        for bot in bots {
            duplicate! { [ dim; [0]; [1]; [2]; [3]; ]
                mins.dim = mins.dim.min(bot.min.dim);
                maxs.dim = maxs.dim.max(bot.max.dim);
            }
        }
        let splits = [Region {
            min: Coord4::from(mins),
            max: Coord4::from(maxs),
        }];
        let all_bots = (0..bots.len()).collect_vec();
        assert_eq!(
            splits[0].intersects(&all_bots, bots),
            (0..bots.len()).collect_vec()
        );
        vec![ItemOrder(Item {
            reg: splits[0],
            my_bots: all_bots,
        })]
        // duplicate! { [ dim; [0]; [1]; [2]; [3]; ]
        //     splits = splits.into_iter().flat_map(|reg| {
        //         if (reg.min.dim + 1..reg.max.dim).contains(&0) {
        //             let mut ret = vec![reg,reg];
        //             ret[0].max.dim = 0;
        //             ret[1].min.dim = 0;
        //             ret
        //         } else {
        //             vec![reg]
        //         }
        //     }).collect();
        // };
        // splits
        //     .into_iter()
        //     .map(|reg| {
        //         let bots = reg.intersects(&all_bots, bots);
        //         ItemOrder(Item { reg, my_bots: bots })
        //     })
        //     .collect()
    }

    #[allow(clippy::vec_init_then_push)]
    fn split(&self, bots: &[Region], notches: &[Vec<i64>]) -> Option<(Self, Self)> {
        let mut my_notches = vec![];
        duplicate! { [ dim; [0]; [1]; [2]; [3]; ]
            my_notches.push(notches[dim].iter().copied().filter(|&n| (self.reg.min.dim + 1..self.reg.max.dim).contains(&n)).collect_vec());
        }
        // eprintln!(
        //     "splitting {self:?} notches {:?}",
        //     my_notches.iter().map(|n| n.len()).collect_vec()
        // );
        // if self.reg.contains(Coord4(500001, 500001, 500001, 500001)) {
        //     eprintln!("  contains 500001");
        // }
        let (axis, notches) = my_notches
            .into_iter()
            .enumerate()
            .max_by_key(|(_, n)| n.len())
            .unwrap();
        if notches.is_empty() {
            // eprintln!("  no notches");
            return None;
        }
        let notch = notches[notches.len() / 2];
        duplicate! { [ dim; [0]; [1]; [2]; [3]; ]
            if axis == dim {
                let mut max = self.reg.max;
                max.dim = notch;
                let reg = { Region { min: self.reg.min, max } };
                let my_bots = reg.intersects(&self.my_bots, bots);
                let item1 = Item { reg, my_bots };
                let mut min = self.reg.min;
                min.dim = notch;
                let reg = { Region { min, max: self.reg.max } };
                let my_bots = reg.intersects(&self.my_bots, bots);
                let item2 =Item { reg, my_bots };
                // eprintln!("  {item1:?}");
                // if item1.reg.contains(Coord4(500001, 500001, 500001, 500001)) {
                //     eprintln!("    contains 500001");
                // }
                // eprintln!("  {item2:?}");
                // if item2.reg.contains(Coord4(500001, 500001, 500001, 500001)) {
                //     eprintln!("    contains 500001");
                // }
                return Some((item1, item2))
            }
        }
        unreachable!();
    }
}

#[derive(Debug)]
struct ItemOrder(Item);

impl ItemOrder {
    fn key(&self) -> usize {
        // (self.0.my_bots.len(), -self.0.reg.max.dist(self.0.reg.min))
        self.0.my_bots.len()
    }
}

impl PartialEq for ItemOrder {
    fn eq(&self, other: &Self) -> bool {
        self.key() == other.key()
    }
}

impl Eq for ItemOrder {}

impl PartialOrd for ItemOrder {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ItemOrder {
    fn cmp(&self, other: &Self) -> Ordering {
        self.key().cmp(&other.key())
    }
}

fn part2(inp: &str) -> i64 {
    let reg = Region {
        min: (500001, 500001, 500001, 500001).into(),
        max: (1499999, 1499999, 1499999, 1499999).into(),
    };
    reg.origin_closest_3d();
    let bots = inp
        .lines()
        .map(|line| Bot::parse(line).into())
        .collect::<Vec<Region>>();
    let mut notches = vec![];
    duplicate! { [ dim; [0]; [1]; [2]; [3]; ]
        let mut acc = HashSet::new();
        for bot in &bots {
            acc.insert(bot.min.dim);
            acc.insert(bot.max.dim);
        }
        notches.push(acc.into_iter().sorted().collect_vec());
    }
    let mut queue = BinaryHeap::from_iter(Item::initial(&bots));
    let mut best = 0;
    let mut bests = vec![];
    // eprintln!();
    while let Some(item) = queue.pop() {
        if item.key() < best {
            break;
        }
        match item.0.split(&bots, &notches) {
            Some((item1, item2)) => {
                let item1 = ItemOrder(item1);
                if item1.key() >= best {
                    queue.push(item1);
                }
                let item2 = ItemOrder(item2);
                if item2.key() >= best {
                    queue.push(item2);
                }
            }
            None => match best.cmp(&item.key()) {
                Ordering::Less => {
                    best = item.key();
                    bests = vec![item.0.reg];
                }
                Ordering::Equal => {
                    bests.push(item.0.reg);
                }
                Ordering::Greater => {}
            },
        }
    }
    // eprintln!("\n len {}, best {:?}", bests.len(), best);
    // for best in &bests {
    //     eprintln!("{} {best:?}", best.origin_closest_3d());
    // }
    bests
        .into_iter()
        .map(|reg| reg.origin_closest_3d())
        .min()
        .unwrap()
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
