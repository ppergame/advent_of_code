use sscanf::scanf;
use std::collections::{HashMap, VecDeque};

type Reg = char;

#[derive(sscanf::FromScanf, Copy, Clone, Debug)]
enum Op {
    #[sscanf(format = "{}")]
    Imm(i64),
    #[sscanf(format = "{}")]
    Reg(Reg),
}

#[derive(sscanf::FromScanf, Copy, Clone, Debug)]
enum Cmd {
    #[sscanf(format = "snd {}")]
    Snd(Op),
    #[sscanf(format = "set {} {}")]
    Set(Reg, Op),
    #[sscanf(format = "add {} {}")]
    Add(Reg, Op),
    #[sscanf(format = "mul {} {}")]
    Mul(Reg, Op),
    #[sscanf(format = "mod {} {}")]
    Mod(Reg, Op),
    #[sscanf(format = "rcv {}")]
    Rcv(Reg),
    #[sscanf(format = "jgz {} {}")]
    Jgz(Op, Op),
}

enum StepResult {
    Ok,
    Snd(i64),
    Rcv,
    Recover(i64),
    Done,
}

struct Machine {
    regs: HashMap<char, i64>,
    pc: i64,
    prog: Vec<Cmd>,
    last_sound: i64,
    part2: bool,
    send_count: i64,
}

impl Machine {
    fn parse(inp: &str, part2: bool) -> Self {
        let prog = inp
            .lines()
            .map(|line| scanf!(line, "{}", Cmd).unwrap())
            .collect();
        Machine {
            regs: HashMap::new(),
            pc: 0,
            prog,
            last_sound: 0,
            part2,
            send_count: 0,
        }
    }

    fn val(&self, val: Op) -> i64 {
        match val {
            Op::Reg(r) => self.regs.get(&r).copied().unwrap_or(0),
            Op::Imm(i) => i,
        }
    }

    fn step(&mut self, rcv: Option<i64>) -> StepResult {
        let mut ret = StepResult::Ok;
        let pc = self.pc;
        self.pc += 1;
        let cmd = self.prog[pc as usize];
        if rcv.is_some() && !matches!(cmd, Cmd::Rcv(_)) {
            panic!("rcv but not rcv");
        }
        match cmd {
            Cmd::Snd(x) => {
                let v = self.val(x);
                if self.part2 {
                    ret = StepResult::Snd(v);
                    self.send_count += 1;
                } else {
                    self.last_sound = v;
                }
            }
            Cmd::Set(x, y) => {
                self.regs.insert(x, self.val(y));
            }
            Cmd::Add(x, y) => {
                self.regs.insert(x, self.val(Op::Reg(x)) + self.val(y));
            }
            Cmd::Mul(x, y) => {
                self.regs.insert(x, self.val(Op::Reg(x)) * self.val(y));
            }
            Cmd::Mod(x, y) => {
                self.regs.insert(x, self.val(Op::Reg(x)) % self.val(y));
            }
            Cmd::Rcv(x) => {
                if self.part2 {
                    match rcv {
                        Some(v) => {
                            self.regs.insert(x, v);
                        }
                        None => {
                            self.pc = pc;
                            return StepResult::Rcv;
                        }
                    }
                } else if self.val(Op::Reg(x)) > 0 {
                    ret = StepResult::Recover(self.last_sound);
                }
            }
            Cmd::Jgz(x, y) => {
                if self.val(x) > 0 {
                    self.pc = pc + self.val(y);
                }
            }
        }
        if pc < 0 || pc >= self.prog.len() as i64 {
            return StepResult::Done;
        }
        ret
    }
}

fn part1(inp: &str) -> i64 {
    let mut m = Machine::parse(inp, false);
    loop {
        let res = m.step(None);
        match res {
            StepResult::Ok => (),
            StepResult::Snd(_) | StepResult::Rcv => unreachable!(),
            StepResult::Recover(r) => {
                if r != 0 {
                    return r;
                }
            }
            StepResult::Done => unreachable!(),
        }
    }
}

struct System {
    m: Vec<Machine>,
    q: Vec<VecDeque<i64>>,
}

impl System {
    // returns true if done, false to transfer control
    fn run_until(&mut self, idx: usize) -> bool {
        let mut rcv = None;
        loop {
            match self.m[idx].step(rcv.take()) {
                StepResult::Ok => (),
                StepResult::Snd(x) => self.q[1 - idx].push_back(x),
                StepResult::Rcv => match self.q[idx].pop_front() {
                    Some(v) => rcv = Some(v),
                    None => return false,
                },
                StepResult::Recover(_) => unreachable!(),
                StepResult::Done => return true,
            }
        }
    }
}

fn part2(inp: &str) -> i64 {
    let m0 = Machine::parse(inp, true);
    let mut m1 = Machine::parse(inp, true);
    m1.regs.insert('p', 1);
    let to0 = VecDeque::new();
    let to1 = VecDeque::new();
    let mut system = System {
        m: vec![m0, m1],
        q: vec![to0, to1],
    };
    let mut done0 = false;
    let mut done1 = false;
    loop {
        if !done0 && system.run_until(0) {
            done0 = true;
        }
        if !done1 && system.run_until(1) {
            done1 = true;
        }
        if (done0 || system.q[0].is_empty()) && (done1 || system.q[0].is_empty()) {
            break;
        }
    }
    system.m[1].send_count
}

xaoc::xaoc!(sample_idx = 27);
