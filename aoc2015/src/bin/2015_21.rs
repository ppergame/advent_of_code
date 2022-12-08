use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, PartialEq)]
struct Item {
    cost: i64,
    dam: i64,
    arm: i64,
}

impl Item {
    fn new(cost: i64, dam: i64, arm: i64) -> Self {
        Self { cost, dam, arm }
    }
}

#[derive(Debug, Clone, Copy)]
struct Mob {
    hp: i64,
    dam: i64,
    arm: i64,
}

impl Mob {
    fn apply(mut self, item: &Item) -> Self {
        self.dam += item.dam;
        self.arm += item.arm;
        self
    }

    fn fight(&self, m2: &Mob) -> bool {
        let m1 = self;
        let dam1 = (m1.dam - m2.arm).max(1);
        let dam2 = (m2.dam - m1.arm).max(1);
        let mut turns1 = m2.hp / dam1;
        if m2.hp % dam1 > 0 {
            turns1 += 1
        }
        let mut turns2 = m1.hp / dam2;
        if m1.hp % dam2 > 0 {
            turns2 += 1
        }
        turns1 <= turns2
    }
}

fn parse_field(s: &str, inp: &str) -> i64 {
    Regex::new(s)
        .unwrap()
        .captures(inp)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse()
        .unwrap()
}

fn parse(inp: &str) -> Mob {
    Mob {
        hp: parse_field(r"Hit Points: (\d+)", inp),
        dam: parse_field(r"Damage: (\d+)", inp),
        arm: parse_field(r"Armor: (\d+)", inp),
    }
}

lazy_static! {
    static ref WEPS: Vec<Item> = vec![
        Item::new(8, 4, 0),
        Item::new(10, 5, 0),
        Item::new(25, 6, 0),
        Item::new(40, 7, 0),
        Item::new(74, 8, 0),
    ];
    static ref ARMS: Vec<Item> = vec![
        Item::new(0, 0, 0),
        Item::new(13, 0, 1),
        Item::new(31, 0, 2),
        Item::new(53, 0, 3),
        Item::new(75, 0, 4),
        Item::new(102, 0, 5),
    ];
    static ref RINGS: Vec<Item> = vec![
        Item::new(0, 0, 0),
        Item::new(25, 1, 0),
        Item::new(50, 2, 0),
        Item::new(100, 3, 0),
        Item::new(20, 0, 1),
        Item::new(40, 0, 2),
        Item::new(80, 0, 3),
    ];
}

fn part1(inp: &str) -> i64 {
    let mut gold = i64::MAX;
    let boss = parse(inp);
    for wep in WEPS.iter() {
        for arm in ARMS.iter() {
            for ring0 in RINGS.iter() {
                for ring1 in RINGS.iter() {
                    if ring0 == ring1 && ring0.cost != 0 {
                        continue;
                    }
                    let mut player = Mob {
                        hp: 100,
                        dam: 0,
                        arm: 0,
                    };
                    let mut cost = 0;
                    for i in [&wep, &arm, &ring0, &ring1] {
                        player = player.apply(i);
                        cost += i.cost;
                    }
                    if player.fight(&boss) {
                        gold = gold.min(cost);
                    }
                }
            }
        }
    }
    gold
}

fn part2(inp: &str) -> i64 {
    let mut gold = 0;
    let boss = parse(inp);
    for wep in WEPS.iter() {
        for arm in ARMS.iter() {
            for ring0 in RINGS.iter() {
                for ring1 in RINGS.iter() {
                    if ring0 == ring1 && ring0.cost != 0 {
                        continue;
                    }
                    let mut player = Mob {
                        hp: 100,
                        dam: 0,
                        arm: 0,
                    };
                    let mut cost = 0;
                    for i in [&wep, &arm, &ring0, &ring1] {
                        player = player.apply(i);
                        cost += i.cost;
                    }
                    if !player.fight(&boss) {
                        gold = gold.max(cost);
                    }
                }
            }
        }
    }
    gold
}

xaoc::xaoc!();
