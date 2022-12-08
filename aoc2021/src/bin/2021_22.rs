use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref STEP_RE: Regex =
        Regex::new(r"(off|on) x=(-?\d+)..(-?\d+),y=(-?\d+)..(-?\d+),z=(-?\d+)..(-?\d+)").unwrap();
}

#[derive(Debug, Clone)]
pub struct Step {
    state: bool,
    xr: (i64, i64),
    yr: (i64, i64),
    zr: (i64, i64),
}

impl Step {
    fn volume(&self) -> i64 {
        (self.xr.1 - self.xr.0) * (self.yr.0 - self.yr.1) * (self.zr.0 - self.zr.1)
    }

    fn get_axis(&self, axis: Axis) -> (i64, i64) {
        match axis {
            Axis::X => self.xr,
            Axis::Y => self.yr,
            Axis::Z => self.zr,
        }
    }

    fn get_axis_mut(&mut self, axis: Axis) -> &mut (i64, i64) {
        match axis {
            Axis::X => &mut self.xr,
            Axis::Y => &mut self.yr,
            Axis::Z => &mut self.zr,
        }
    }

    fn split(self, pos: i64, axis: Axis) -> (Option<Step>, Option<Step>) {
        let axe = self.get_axis(axis);
        if pos <= axe.0 {
            return (None, Some(self));
        }
        if axe.1 <= pos {
            return (Some(self), None);
        }
        assert!((axe.0..axe.1).contains(&pos));
        let mut left = self.clone();
        let mut right = self;
        left.get_axis_mut(axis).1 = pos;
        right.get_axis_mut(axis).0 = pos;
        (Some(left), Some(right))
    }
}

#[derive(Debug, Clone)]
pub struct Input {
    steps: Vec<Step>,
}

fn parse(inp: &str) -> Input {
    let steps = inp
        .lines()
        .map(|line| {
            let caps = STEP_RE.captures(line).unwrap();
            let coo = (2..8)
                .map(|i| caps[i].parse::<i64>().unwrap())
                .collect_vec();
            Step {
                state: match &caps[1] {
                    "off" => false,
                    "on" => true,
                    _ => unreachable!(),
                },
                xr: (coo[0], coo[1] + 1),
                yr: (coo[2], coo[3] + 1),
                zr: (coo[4], coo[5] + 1),
            }
        })
        .collect();
    Input { steps }
}

fn part1(inp: &str) -> i64 {
    let inp = parse(inp);
    let hmm = inp.steps.iter().cloned().fold(vec![], |steps, step| {
        for axis in Axis::all() {
            let axe = step.get_axis(axis);
            if !(-50..50).contains(&axe.0) {
                return steps;
            }
            if !(-50..50).contains(&axe.1) {
                return steps;
            }
        }
        if steps.is_empty() {
            return vec![step];
        }
        add_step(&steps, &step)
    });
    hmm.into_iter()
        .filter_map(|step| {
            if step.state {
                Some(step.volume())
            } else {
                None
            }
        })
        .sum()
}

fn carve_axis(step: Step, new_step: &Step, axis: Axis) -> (Option<Step>, Vec<Step>) {
    // println!("carve_axis {:?} {:?} {:?}", step, new_step, axis);
    let mut inside = None;
    let mut outside = vec![];
    let axe = new_step.get_axis(axis);
    let (left, right) = step.split(axe.0, axis);
    // println!("  split by {:?} -> {:?} {:?}", axe.0, left, right);
    if let Some(left) = left {
        outside.push(left);
    }
    if let Some(right) = right {
        let (left, right) = right.split(axe.1, axis);
        if let Some(right) = right {
            outside.push(right);
        }
        inside = left
    }
    // println!("  -> {:?} {:?}", inside, outside);
    (inside, outside)
}

fn add_step(steps: &[Step], new_step: &Step) -> Vec<Step> {
    let mut ret = vec![];
    for step in steps {
        let mut inside = Some(step.clone());
        for axis in Axis::all() {
            match inside {
                None => break,
                Some(ins) => {
                    let (ins, outside) = carve_axis(ins, new_step, axis);
                    ret.extend(outside);
                    inside = ins;
                }
            }
        }
    }
    ret.push(new_step.clone());
    ret
}

#[derive(Debug, Clone, Copy)]
enum Axis {
    X,
    Y,
    Z,
}

impl Axis {
    fn all() -> Vec<Axis> {
        vec![Axis::X, Axis::Y, Axis::Z]
    }
}

fn part2(inp: &str) -> i64 {
    let inp = parse(inp);
    let hmm = inp.steps.iter().cloned().fold(vec![], |steps, step| {
        if steps.is_empty() {
            return vec![step];
        }
        add_step(&steps, &step)
    });
    hmm.into_iter()
        .filter_map(|step| {
            if step.state {
                Some(step.volume())
            } else {
                None
            }
        })
        .sum()
}

xaoc::xaoc!();
