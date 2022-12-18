mod emit;

use anyhow::{Context, Result};
use itertools::Itertools;
use sscanf::scanf;
use std::collections::HashMap;
use std::str::FromStr;
use strum_macros::EnumString;
use wasmtime::{Caller, Config, Engine, Func, Instance, Module, Store, Val};

#[derive(Copy, Clone, Debug, EnumString)]
pub enum Opcode {
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

pub struct Instr {
    pub cmd: Opcode,
    pub a: usize,
    pub b: usize,
    pub c: usize,
}

type CbMap<'a> = HashMap<i64, Box<dyn FnMut(i64, [i64; 6]) -> bool + 'a>>;

pub struct Machine<'a> {
    ip_reg: usize,
    pub regs: [i64; 6],
    pub pc: i64,
    pub prog: Vec<Instr>,
    callbacks: CbMap<'a>,
}

impl<'a> Machine<'a> {
    pub fn parse(inp: &str) -> Self {
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
        Self {
            ip_reg,
            regs: Default::default(),
            pc: 0,
            prog,
            callbacks: HashMap::new(),
        }
    }

    pub fn run(&mut self) {
        while !self.step() {}
    }

    pub fn run_wasm(&mut self) -> Result<()> {
        let mut config = Config::new();
        config.cranelift_opt_level(wasmtime::OptLevel::SpeedAndSize);
        let engine = Engine::new(&config).context("Engine::new")?;
        let module = Module::from_binary(
            &engine,
            &emit::emit(self.callbacks.keys().copied(), &self.prog, self.ip_reg),
        )
        .context("Module::from_binary")?;
        let mut store = Store::new(&engine, &mut self.callbacks);
        let callback = Func::wrap(
            &mut store,
            |mut caller: Caller<'_, &mut CbMap>,
             r0: i64,
             r1: i64,
             r2: i64,
             r3: i64,
             r4: i64,
             r5: i64,
             pc: i64|
             -> i32 {
                let regs = [r0, r1, r2, r3, r4, r5];
                let callback = caller.data_mut().get_mut(&pc).unwrap();
                callback(pc, regs) as i32
            },
        );
        let instance =
            Instance::new(&mut store, &module, &[callback.into()]).context("Instance::new")?;
        for i in 0..6 {
            instance
                .get_export(&mut store, &format!("r{i}"))
                .unwrap()
                .into_global()
                .unwrap()
                .set(&mut store, Val::I64(self.regs[i]))
                .context("set global")?;
        }
        instance
            .get_export(&mut store, "pc")
            .unwrap()
            .into_global()
            .unwrap()
            .set(&mut store, Val::I64(self.pc))
            .context("set global")?;
        let run = instance
            .get_func(&mut store, "run")
            .unwrap()
            .typed::<(), (), _>(&store)
            .context("instance.get_func")?;
        run.call(&mut store, ()).context("run.call")?;
        for i in 0..6 {
            self.regs[i] = instance
                .get_export(&mut store, &format!("r{i}"))
                .unwrap()
                .into_global()
                .unwrap()
                .get(&mut store)
                .i64()
                .unwrap();
        }
        self.pc = instance
            .get_export(&mut store, "pc")
            .unwrap()
            .into_global()
            .unwrap()
            .get(&mut store)
            .i64()
            .unwrap();
        Ok(())
    }

    pub fn add_callback(&mut self, pc: i64, cb: impl FnMut(i64, [i64; 6]) -> bool + 'a) {
        self.callbacks.insert(pc, Box::new(cb));
    }

    fn step(&mut self) -> bool {
        let Some(instr) = self.prog.get(self.pc as usize) else { return true };
        if let Some(cb) = self.callbacks.get_mut(&self.pc) {
            if cb(self.pc, self.regs) {
                return true;
            }
        }
        self.regs[self.ip_reg] = self.pc;
        let cmd = instr.cmd;
        let a = instr.a;
        let b = instr.b;
        let c = instr.c;
        // print!("ip={} {:?} {cmd:?} {a} {b} {c}", self.pc, self.regs);
        *self.regs.get_mut(c).unwrap() = match cmd {
            Opcode::Addr => self.regs[a] + self.regs[b],
            Opcode::Addi => self.regs[a] + b as i64,
            Opcode::Mulr => self.regs[a] * self.regs[b],
            Opcode::Muli => self.regs[a] * b as i64,
            Opcode::Banr => self.regs[a] & self.regs[b],
            Opcode::Bani => self.regs[a] & b as i64,
            Opcode::Borr => self.regs[a] | self.regs[b],
            Opcode::Bori => self.regs[a] | b as i64,
            Opcode::Setr => self.regs[a],
            Opcode::Seti => a as i64,
            Opcode::Gtir => (a as i64 > self.regs[b]) as i64,
            Opcode::Gtri => (self.regs[a] > b as i64) as i64,
            Opcode::Gtrr => (self.regs[a] > self.regs[b]) as i64,
            Opcode::Eqir => (a as i64 == self.regs[b]) as i64,
            Opcode::Eqri => (self.regs[a] == b as i64) as i64,
            Opcode::Eqrr => (self.regs[a] == self.regs[b]) as i64,
        };
        self.pc = self.regs[self.ip_reg] + 1;
        // println!(" {:?}", self.regs);
        false
    }
}
