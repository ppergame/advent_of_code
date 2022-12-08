use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref BOSS_RE: Regex = Regex::new(r"Hit Points: (\d+)\s*Damage: (\d+)").unwrap();
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
struct State<const HARD: bool> {
    hp: i64,
    mana: i64,
    bhp: i64,
    bdam: i64,

    // number of turns remaining
    shield: i64,
    poison: i64,
    recharge: i64,
}

impl<const HARD: bool> State<HARD> {
    fn parse(inp: &str) -> Self {
        let caps = BOSS_RE.captures(inp).unwrap();
        Self {
            hp: 50,
            mana: 500,
            bhp: caps.get(1).unwrap().as_str().parse().unwrap(),
            bdam: caps.get(2).unwrap().as_str().parse().unwrap(),
            shield: 0,
            poison: 0,
            recharge: 0,
        }
    }

    fn effects(&mut self) {
        self.shield -= 1;
        if self.poison > 0 {
            self.poison -= 1;
            self.bhp -= 3;
        }
        if self.recharge > 0 {
            self.recharge -= 1;
            self.mana += 101;
        }
    }

    fn bossturn(mut self) -> Self {
        self.effects();
        if self.bhp > 0 {
            let dam = if self.shield > 0 {
                (self.bdam - 7).max(1)
            } else {
                self.bdam
            };
            self.hp -= dam;
        }
        self
    }

    fn succ(&self) -> Vec<(Self, i64)> {
        let mut ret = vec![];
        if self.hp <= 0 || self.bhp <= 0 {
            return vec![];
        }
        if self.mana < 53 {
            return vec![];
        }
        let mut newstate = *self;
        if HARD {
            newstate.hp -= 1;
            if newstate.hp <= 0 {
                return vec![];
            }
        }
        newstate.effects();
        // magic missile
        let msp = 53;
        if newstate.mana >= msp {
            let mut state = newstate;
            state.mana -= msp;
            state.bhp -= 4;
            ret.push((state.bossturn(), msp));
        }
        // drain
        let msp = 73;
        if newstate.mana >= msp {
            let mut state = newstate;
            state.mana -= msp;
            state.hp += 2;
            state.bhp -= 2;
            ret.push((state.bossturn(), msp));
        }
        // shield
        let msp = 113;
        if newstate.mana >= msp {
            let mut state = newstate;
            state.mana -= msp;
            state.shield = 6;
            ret.push((state.bossturn(), msp));
        }
        // poison
        let msp = 173;
        if newstate.mana >= msp {
            let mut state = newstate;
            state.mana -= msp;
            state.poison = 6;
            ret.push((state.bossturn(), msp));
        }
        // recharge
        let msp = 229;
        if newstate.mana >= msp {
            let mut state = newstate;
            state.mana -= msp;
            state.recharge = 5;
            ret.push((state.bossturn(), msp));
        }
        ret
    }
}

fn part1(inp: &str) -> i64 {
    let initial = State::<false>::parse(inp);
    pathfinding::directed::dijkstra::dijkstra(
        &initial,
        |state| state.succ(),
        |state| state.bhp <= 0,
    )
    .unwrap()
    .1
}

fn part2(inp: &str) -> i64 {
    let initial = State::<true>::parse(inp);
    pathfinding::directed::dijkstra::dijkstra(
        &initial,
        |state| state.succ(),
        |state| state.bhp <= 0,
    )
    .unwrap()
    .1
}

xaoc::xaoc!();
