use std::{collections::HashSet, convert::TryFrom};

#[derive(Debug, Clone, Copy)]
enum Inst {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

fn parse_prog(inp: &str) -> Vec<Inst> {
    inp.lines()
        .map(|line| {
            let (inst, arg) = line.split_once(' ').unwrap();
            let arg: i32 = arg.parse().unwrap();
            match inst {
                "nop" => Inst::Nop(arg),
                "acc" => Inst::Acc(arg),
                "jmp" => Inst::Jmp(arg),
                _ => panic!(),
            }
        })
        .collect()
}

struct Machine {
    prog: Vec<Inst>,
    pc: i32,
    acc: i32,
}

enum MachineStatus {
    Loop,
    Done,
}

impl Machine {
    fn new(prog: Vec<Inst>) -> Machine {
        Machine {
            prog,
            pc: 0,
            acc: 0,
        }
    }

    fn step(&mut self) {
        match self.prog[usize::try_from(self.pc).unwrap()] {
            Inst::Nop(_) => (),
            Inst::Acc(arg) => self.acc += arg,
            Inst::Jmp(arg) => self.pc += arg - 1,
        };
        self.pc += 1;
    }

    fn run(&mut self) -> MachineStatus {
        let mut pc_hist = HashSet::<i32>::new();
        loop {
            if pc_hist.contains(&self.pc) {
                return MachineStatus::Loop;
            }
            pc_hist.insert(self.pc);
            self.step();
            if usize::try_from(self.pc).unwrap() == self.prog.len() {
                return MachineStatus::Done;
            }
        }
    }
}

fn part1(inp: &str) -> i32 {
    let prog = parse_prog(inp);
    let mut m = Machine::new(prog);
    assert!(matches!(m.run(), MachineStatus::Loop));
    m.acc
}

fn part2(inp: &str) -> i32 {
    let prog = parse_prog(inp);
    for (i, inst) in prog.iter().enumerate() {
        let new_inst = match inst {
            Inst::Acc(_) => continue,
            Inst::Nop(arg) => Inst::Jmp(*arg),
            Inst::Jmp(arg) => Inst::Nop(*arg),
        };
        let mut dprog = prog.to_vec();
        dprog[i] = new_inst;
        let mut m = Machine::new(dprog);
        match m.run() {
            MachineStatus::Loop => (),
            MachineStatus::Done => return m.acc,
        }
    }
    unreachable!();
}

xaoc::xaoc!();
