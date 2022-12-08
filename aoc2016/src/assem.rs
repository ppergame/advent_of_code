use anyhow::{anyhow, Error, Result};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
enum Oper {
    Imm(i64),
    Reg(Reg),
}

impl FromStr for Oper {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let me = s
            .parse::<Reg>()
            .map(Self::Reg)
            .or_else(|_| s.parse().map(Self::Imm))?;
        Ok(me)
    }
}

#[derive(Debug, Clone, Copy)]
enum Reg {
    A,
    B,
    C,
    D,
}

impl FromStr for Reg {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "a" => Ok(Reg::A),
            "b" => Ok(Reg::B),
            "c" => Ok(Reg::C),
            "d" => Ok(Reg::D),
            _ => Err(anyhow!("unknown register")),
        }
    }
}

pub enum BRes {
    Ok,
    Done,
    Out(i64),
}

#[derive(Debug, Clone, Copy)]
enum Instr {
    Cpy(Oper, Oper),
    Inc(Oper),
    Dec(Oper),
    Jnz(Oper, Oper),
    Tgl(Oper),
    Out(Oper),
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub struct State(i64, i64, i64, i64, i64);

pub struct Bunny {
    pub a: i64,
    pub b: i64,
    pub c: i64,
    pub d: i64,
    prog: Vec<Instr>,
    pub pc: i64,
    pub cp: HashMap<usize, usize>,
}

impl Bunny {
    pub fn state(&self) -> State {
        State(self.a, self.b, self.c, self.d, self.pc)
    }

    pub fn parse(inp: &str) -> Self {
        let prog = inp
            .lines()
            .map(|line| {
                let (inst, rest) = line.split_once(' ').unwrap();
                match inst {
                    "cpy" => {
                        let (src, dst) = rest.split_once(' ').unwrap();
                        Instr::Cpy(src.parse().unwrap(), dst.parse().unwrap())
                    }
                    "inc" => Instr::Inc(rest.parse().unwrap()),
                    "dec" => Instr::Dec(rest.parse().unwrap()),
                    "jnz" => {
                        let (e, shift) = rest.split_once(' ').unwrap();
                        Instr::Jnz(e.parse().unwrap(), shift.parse().unwrap())
                    }
                    "tgl" => Instr::Tgl(rest.parse().unwrap()),
                    "out" => Instr::Out(rest.parse().unwrap()),
                    _ => panic!("unknown instruction {}", line),
                }
            })
            .collect::<Vec<_>>();
        Self {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            prog,
            pc: 0,
            cp: HashMap::new(),
        }
    }

    fn setr(&mut self, reg: Reg, val: i64) {
        match reg {
            Reg::A => self.a = val,
            Reg::B => self.b = val,
            Reg::C => self.c = val,
            Reg::D => self.d = val,
        }
    }

    fn getr(&self, reg: Reg) -> i64 {
        match reg {
            Reg::A => self.a,
            Reg::B => self.b,
            Reg::C => self.c,
            Reg::D => self.d,
        }
    }

    fn get(&self, src: Oper) -> i64 {
        match src {
            Oper::Imm(imm) => imm,
            Oper::Reg(reg) => self.getr(reg),
        }
    }

    pub fn step(&mut self) -> BRes {
        let instr = self.prog[self.pc as usize];
        match instr {
            Instr::Cpy(src, dst) => {
                let val = self.get(src);
                if let Oper::Reg(reg) = dst {
                    self.setr(reg, val);
                } else {
                    unreachable!();
                }
                self.pc += 1;
            }
            Instr::Inc(dst) => {
                if let Oper::Reg(reg) = dst {
                    let val = self.getr(reg);
                    self.setr(reg, val + 1);
                } else {
                    unreachable!();
                }
                self.pc += 1;
            }
            Instr::Dec(dst) => {
                if let Oper::Reg(reg) = dst {
                    let val = self.getr(reg);
                    self.setr(reg, val - 1);
                } else {
                    unreachable!();
                }
                self.pc += 1;
            }
            Instr::Jnz(oper, shift) => {
                let val = self.get(oper);
                if val != 0 {
                    self.pc += self.get(shift);
                } else {
                    self.pc += 1;
                }
            }
            Instr::Tgl(oper) => {
                let val = self.get(oper);
                let idx = self.pc + val;
                if 0 <= idx && (idx as usize) < self.prog.len() {
                    let idx = idx as usize;
                    let instr = match self.prog[idx] {
                        Instr::Cpy(op1, op2) => Instr::Jnz(op1, op2),
                        Instr::Inc(oper) => Instr::Dec(oper),
                        Instr::Dec(oper) => Instr::Inc(oper),
                        Instr::Jnz(op1, op2) => Instr::Cpy(op1, op2),
                        Instr::Tgl(oper) => Instr::Inc(oper),
                        Instr::Out(_) => unreachable!(),
                    };
                    self.prog[idx] = instr;
                }
                self.pc += 1;
            }
            Instr::Out(oper) => {
                let val = self.get(oper);
                self.pc += 1;
                return BRes::Out(val);
            }
        }
        if 0 < self.pc && (self.pc as usize) < self.prog.len() {
            BRes::Ok
        } else {
            BRes::Done
        }
    }

    pub fn run(&mut self) {
        loop {
            let pc = self.pc as usize;
            let res = if let Some(
                [Instr::Cpy(Oper::Reg(Reg::B), Oper::Reg(Reg::C)), Instr::Inc(Oper::Reg(Reg::A)), Instr::Dec(Oper::Reg(Reg::C)), Instr::Jnz(Oper::Reg(Reg::C), Oper::Imm(-2)), Instr::Dec(Oper::Reg(Reg::D)), Instr::Jnz(Oper::Reg(Reg::D), Oper::Imm(-5))],
            ) = self.prog.get(pc..pc + 6)
            {
                self.p1()
            } else {
                self.step()
            };
            match res {
                BRes::Ok => (),
                BRes::Done => break,
                BRes::Out(_) => unreachable!(),
            }
        }
    }

    fn p1(&mut self) -> BRes {
        *self.cp.entry(1).or_default() += 1;
        assert!(self.b > 0);
        assert!(self.d > 0);
        self.a += self.b * self.d;
        self.pc += 6;
        BRes::Ok
    }
}
