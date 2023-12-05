use aoc2019::intcode::*;
use itertools::Itertools;
use std::cmp;

fn part1(inp: &str) -> i64 {
    let mut max = 0;
    for phases in (0..5).permutations(5) {
        let mut val = 0;
        for phase in phases {
            let mut ic = Intcode::new(inp);
            assert!(matches!(ic.run().unwrap(), IntcodeStatus::Input));
            ic.input = Some(phase);
            assert!(matches!(ic.run().unwrap(), IntcodeStatus::Input));
            ic.input = Some(val);
            match ic.run().unwrap() {
                IntcodeStatus::Output(output) => val = output,
                _ => panic!("no output"),
            }
        }
        max = cmp::max(max, val);
    }
    max
}

fn part2(inp: &str) -> i64 {
    let mut max = 0;
    for phases in (5..10).permutations(5) {
        let mut ics = phases
            .iter()
            .map(|phase| {
                let mut ic = Intcode::new(inp);
                ic.input = Some(*phase);
                ic
            })
            .collect::<Vec<Intcode>>();
        let mut val = 0;
        let mut last_thrust = 0;
        'outer: loop {
            for (idx, ic) in ics.iter_mut().enumerate() {
                match ic.run().unwrap() {
                    IntcodeStatus::Input => (),
                    IntcodeStatus::Halt => break 'outer,
                    _ => panic!("bad status"),
                }
                ic.input = Some(val);
                match ic.run().unwrap() {
                    IntcodeStatus::Output(output) => val = output,
                    _ => panic!("bad status"),
                }
                if idx == 4 {
                    last_thrust = val;
                }
            }
        }
        max = cmp::max(max, last_thrust);
    }
    max
}

xaoc::xaoc!();
