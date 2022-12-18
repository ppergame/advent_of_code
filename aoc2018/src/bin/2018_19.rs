use itertools::Itertools;
use sscanf::scanf;
use std::str::FromStr;
use strum_macros::EnumString;

#[derive(Copy, Clone, Debug, EnumString)]
enum Opcode {
    Addr = 0,
    Addi = 1,
    Mulr = 2,
    Muli = 3,
    Banr = 4,
    Bani = 5,
    Borr = 6,
    Bori = 7,
    Setr = 8,
    Seti = 9,
    Gtir = 10,
    Gtri = 11,
    Gtrr = 12,
    Eqir = 13,
    Eqri = 14,
    Eqrr = 15,
}

impl Opcode {
    fn parse(s: &str) -> Self {
        let mut cc = s.chars();
        let first = cc.next().unwrap().to_ascii_uppercase();
        let name = std::iter::once(first).chain(cc).collect::<String>();
        Opcode::from_str(&name).unwrap()
    }
}

struct Instr {
    cmd: Opcode,
    a: usize,
    b: usize,
    c: usize,
}

struct Machine {
    ip_reg: usize,
    regs: [usize; 6],
    pc: usize,
    prog: Vec<Instr>,
    profi: Vec<usize>,
}

impl Machine {
    fn parse(inp: &str) -> Self {
        let mut li = inp.lines();
        let ip_reg = scanf!(li.next().unwrap(), "#ip {}", usize).unwrap();
        let mut prog = vec![];
        for line in li {
            let sp = line.split_ascii_whitespace().collect_vec();
            let instr = Instr {
                cmd: Opcode::parse(sp[0]),
                a: sp[1].parse().unwrap(),
                b: sp[2].parse().unwrap(),
                c: sp[3].parse().unwrap(),
            };
            prog.push(instr);
        }
        let profi = std::iter::repeat(0).take(prog.len()).collect();
        Self {
            ip_reg,
            regs: Default::default(),
            pc: 0,
            prog,
            profi,
        }
    }

    fn step(&mut self) -> bool {
        let Some(instr) = self.prog.get(self.pc) else { return true };
        self.profi[self.pc] += 1;
        self.regs[self.ip_reg] = self.pc;
        let cmd = instr.cmd;
        let a = instr.a;
        let b = instr.b;
        let c = instr.c;
        // print!("ip={} {:?} {cmd:?} {a} {b} {c}", self.pc, self.regs);
        *self.regs.get_mut(c).unwrap() = match cmd {
            Opcode::Addr => self.regs[a] + self.regs[b],
            Opcode::Addi => self.regs[a] + b,
            Opcode::Mulr => self.regs[a] * self.regs[b],
            Opcode::Muli => self.regs[a] * b,
            Opcode::Banr => self.regs[a] & self.regs[b],
            Opcode::Bani => self.regs[a] & b,
            Opcode::Borr => self.regs[a] | self.regs[b],
            Opcode::Bori => self.regs[a] | b,
            Opcode::Setr => self.regs[a],
            Opcode::Seti => a,
            Opcode::Gtir => (a > self.regs[b]) as usize,
            Opcode::Gtri => (self.regs[a] > b) as usize,
            Opcode::Gtrr => (self.regs[a] > self.regs[b]) as usize,
            Opcode::Eqir => (a == self.regs[b]) as usize,
            Opcode::Eqri => (self.regs[a] == b) as usize,
            Opcode::Eqrr => (self.regs[a] == self.regs[b]) as usize,
        };
        self.pc = self.regs[self.ip_reg];
        self.pc += 1;
        // println!(" {:?}", self.regs);
        false
    }
}

fn part1(inp: &str) -> usize {
    let mut m = Machine::parse(inp);
    while !m.step() {}
    m.regs[0]
}

fn part2(inp: &str) -> usize {
    let mut m = Machine::parse(inp);
    m.regs[0] = 1;
    for _ in 0..1000 {
        m.step();
    }
    let num = *m.regs.iter().max().unwrap();
    (1..=num).filter(|f| num % f == 0).sum()
}

xaoc::xaoc!(sample_idx = 11);
