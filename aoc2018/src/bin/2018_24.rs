use itertools::Itertools;
use sscanf::scanf;
use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
};

#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
enum DamageType {
    Bludgeoning,
    Cold,
    Fire,
    Radiation,
    Slashing,
}

impl DamageType {
    fn parse(s: &str) -> Self {
        match s {
            "bludgeoning" => Self::Bludgeoning,
            "cold" => Self::Cold,
            "fire" => Self::Fire,
            "radiation" => Self::Radiation,
            "slashing" => Self::Slashing,
            _ => panic!(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Unit {
    id: usize,
    good: bool,
    count: usize,
    hp: usize,
    weak: HashSet<DamageType>,
    immune: HashSet<DamageType>,
    dam: usize,
    damtype: DamageType,
    init: usize,
}

impl Unit {
    fn parse(s: &str, id: usize, good: bool) -> Self {
        let (count, hp, iw, dam, damtype, init) = scanf!(s,
            "{usize} units each with {usize} hit points{str}with an attack that does {usize} {str} damage at initiative {usize}").unwrap();
        let iw = iw.trim().trim_start_matches('(').trim_end_matches(')');
        let mut weak = HashSet::new();
        let mut immune = HashSet::new();
        if !iw.is_empty() {
            for s in iw.split("; ") {
                if let Some(s) = s.strip_prefix("weak to ") {
                    weak.extend(s.split(", ").map(DamageType::parse));
                } else if let Some(s) = s.strip_prefix("immune to ") {
                    immune.extend(s.split(", ").map(DamageType::parse));
                } else {
                    panic!();
                }
            }
        }
        Unit {
            id,
            good,
            count,
            hp,
            weak,
            immune,
            dam,
            damtype: DamageType::parse(damtype),
            init,
        }
    }

    fn select_target(&self, targets: &HashMap<usize, Unit>) -> Option<usize> {
        targets
            .values()
            .filter(|target| self.good != target.good && !target.immune.contains(&self.damtype))
            .max_by_key(|unit| {
                let crit = unit.weak.contains(&self.damtype);
                // eprintln!(
                //     "{} would deal {} {} damage",
                //     self.id,
                //     unit.id,
                //     self.count * self.dam * if crit { 2 } else { 1 }
                // );
                (crit, unit.count * unit.dam, unit.init)
            })
            .map(|target| target.id)
    }
}

fn parse(inp: &str) -> HashMap<usize, Unit> {
    let mut next_id = 0;
    let mut inp = inp.split("\n\n");
    let mut us = inp.next().unwrap().lines();
    assert_eq!(us.next().unwrap(), "Immune System:");
    let mut ret = HashMap::new();
    ret.extend(us.map(|s| {
        let id = next_id;
        next_id += 1;
        let u = Unit::parse(s, id, true);
        (id, u)
    }));
    let mut them = inp.next().unwrap().lines();
    assert_eq!(them.next().unwrap(), "Infection:");
    ret.extend(them.map(|s| {
        let id = next_id;
        next_id += 1;
        let u = Unit::parse(s, id, false);
        (id, u)
    }));
    ret
}

#[allow(dead_code)]
fn print(units: &HashMap<usize, Unit>) {
    eprintln!();
    let (us, them): (Vec<_>, Vec<_>) = units
        .iter()
        .sorted_by_key(|(id, _)| *id)
        .partition(|(_, u)| u.good);
    eprintln!("Immune System:");
    for (id, u) in us {
        eprintln!("Group {} contains {} units", id, u.count);
    }
    eprintln!("Infection:");
    for (id, u) in them {
        eprintln!("Group {} contains {} units", id, u.count);
    }
}

enum Outcome {
    Good(usize),
    Bad(usize),
    Draw,
}

impl Outcome {
    fn score(&self) -> Option<usize> {
        match self {
            Outcome::Good(x) => Some(*x),
            Outcome::Bad(x) => Some(*x),
            Outcome::Draw => None,
        }
    }
}

fn fight(mut units: HashMap<usize, Unit>) -> Outcome {
    let mut prev = units.clone();
    loop {
        // print(&units);
        let (us, them): (Vec<_>, Vec<_>) = units.iter().partition(|(_, u)| u.good);
        if us.is_empty() {
            // eprintln!("{them:?}");
            return Outcome::Bad(them.iter().map(|(_, u)| u.count).sum());
        }
        if them.is_empty() {
            return Outcome::Good(us.iter().map(|(_, u)| u.count).sum());
        }
        let mut select_order = units
            .keys()
            .copied()
            .sorted_by_key(|id| {
                let unit = &units[id];
                (unit.count * unit.dam, unit.init)
            })
            .rev()
            .collect_vec();
        // eprintln!(
        //     "select order {:?}",
        //     select_order
        //         .iter()
        //         .map(|id| (id, units[id].count * units[id].dam, units[id].init))
        //         .collect_vec()
        // );
        let mut selected_targets = HashMap::new();
        let mut remaining_units = units.clone();
        for &id in &select_order {
            let unit = &units[&id];
            if let Some(target) = unit.select_target(&remaining_units) {
                selected_targets.insert(id, target);
                remaining_units.remove(&target);
            }
        }
        select_order.sort_by_key(|id| Reverse(units[id].init));
        for id in select_order {
            if let Some(target_id) = selected_targets.get(&id) {
                let unit = &units[&id];
                if unit.count == 0 {
                    continue;
                }
                let target = &units[target_id];
                let crit = target.weak.contains(&unit.damtype);
                let dam = unit.count * unit.dam;
                let dam = if crit { dam * 2 } else { dam };
                let units_killed = dam / target.hp;
                let count = units.get_mut(target_id).map(|t| &mut t.count).unwrap();
                *count = count.saturating_sub(units_killed);
                // eprintln!(
                //     "{id} attacks {target_id} for {dam} damage, killing {units_killed} units"
                // );
            }
        }
        units.retain(|_, u| u.count > 0);
        if prev == units {
            return Outcome::Draw;
        }
        prev = units.clone();
    }
}

fn part1(inp: &str) -> usize {
    let units = parse(inp);
    fight(units).score().unwrap()
}

fn part2(inp: &str) -> usize {
    let units = parse(inp);
    for boost in 0..10000 {
        let mut units = units.clone();
        for u in units.values_mut() {
            if u.good {
                u.dam += boost;
            }
        }
        if let Outcome::Good(score) = fight(units) {
            return score;
        }
    }
    unreachable!();
}

xaoc::xaoc!(
    sample = r#"Immune System:
17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3

Infection:
801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4"#
);
