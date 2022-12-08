pub struct Intcode {
    pub pc: i64,
    pub cs: Vec<i64>,
    pub input: Option<i64>,
    base: i64,
    pub last_output: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct IntcodeError {
    pub pc: i64,
    pub why: String,
}

#[must_use]
#[derive(Debug)]
pub enum IntcodeStatus {
    Ok,
    Input,
    Output(i64),
    Halt,
}

enum Param {
    Imm(i64),
    Pos(i64),
    Rel(i64),
}

impl Param {
    pub fn get(&self, ic: &mut Intcode) -> i64 {
        match self {
            Param::Imm(i) => *i,
            Param::Pos(pos) => *ic.loc(*pos),
            Param::Rel(pos) => *ic.loc(*pos + ic.base),
        }
    }

    pub fn set(&self, ic: &mut Intcode, val: i64) {
        match self {
            Param::Imm(_) => panic!("can't set immediate"),
            Param::Pos(pos) => *ic.loc(*pos) = val,
            Param::Rel(pos) => *ic.loc(*pos + ic.base) = val,
        }
    }
}

struct Opcode {
    num: i64,
    opcode: i64,
}

impl Opcode {
    pub fn new(ic: &mut Intcode) -> Opcode {
        let num = ic.at();
        Opcode {
            num: num / 100,
            opcode: num % 100,
        }
    }

    pub fn shift(&mut self, ic: &mut Intcode) -> Param {
        let m = self.num % 10;
        self.num /= 10;
        match m {
            0 => Param::Pos(ic.at()),
            1 => Param::Imm(ic.at()),
            2 => Param::Rel(ic.at()),
            _ => panic!("invalid parameter mode"),
        }
    }
}

type Result = std::result::Result<IntcodeStatus, IntcodeError>;

impl Intcode {
    pub fn new(s: &str) -> Intcode {
        Self::new_with_seq(
            &s.split(',')
                .map(|i| i.parse().unwrap())
                .collect::<Vec<i64>>(),
        )
    }

    pub fn new_with_seq<'a>(v: impl IntoIterator<Item = &'a i64>) -> Intcode {
        Intcode {
            pc: 0,
            cs: v.into_iter().cloned().collect::<Vec<i64>>(),
            input: None,
            base: 0,
            last_output: None,
        }
    }

    pub fn at(&mut self) -> i64 {
        let ret = self.cs[self.pc as usize];
        self.pc += 1;
        ret
    }

    pub fn loc(&mut self, pos: i64) -> &mut i64 {
        let must_len = (pos + 1) as usize;
        if self.cs.len() < must_len {
            self.cs.resize(must_len, 0);
        }
        self.cs.get_mut(pos as usize).unwrap()
    }

    pub fn step(&mut self) -> Result {
        let mut opcode = Opcode::new(self);
        match opcode.opcode {
            // add
            1 => {
                let p1 = opcode.shift(self).get(self);
                let p2 = opcode.shift(self).get(self);
                let p3 = opcode.shift(self);
                p3.set(self, p1 + p2);
                Ok(IntcodeStatus::Ok)
            }
            // multiply
            2 => {
                let p1 = opcode.shift(self).get(self);
                let p2 = opcode.shift(self).get(self);
                let p3 = opcode.shift(self);
                p3.set(self, p1 * p2);
                Ok(IntcodeStatus::Ok)
            }
            // input
            3 => match self.input.take() {
                None => {
                    self.pc -= 1;
                    Ok(IntcodeStatus::Input)
                }
                Some(val) => {
                    let p1 = opcode.shift(self);
                    p1.set(self, val);
                    Ok(IntcodeStatus::Ok)
                }
            },
            // output
            4 => {
                let p1 = opcode.shift(self);
                Ok(IntcodeStatus::Output(p1.get(self)))
            }
            // jump-if-true
            5 => {
                let p1 = opcode.shift(self);
                let p2 = opcode.shift(self);
                if p1.get(self) != 0 {
                    self.pc = p2.get(self)
                }
                Ok(IntcodeStatus::Ok)
            }
            // jump-if-false
            6 => {
                let p1 = opcode.shift(self);
                let p2 = opcode.shift(self);
                if p1.get(self) == 0 {
                    self.pc = p2.get(self)
                }
                Ok(IntcodeStatus::Ok)
            }
            // less than
            7 => {
                let p1 = opcode.shift(self);
                let p2 = opcode.shift(self);
                let p3 = opcode.shift(self);
                if p1.get(self) < p2.get(self) {
                    p3.set(self, 1);
                } else {
                    p3.set(self, 0);
                }
                Ok(IntcodeStatus::Ok)
            }
            // equals
            8 => {
                let p1 = opcode.shift(self);
                let p2 = opcode.shift(self);
                let p3 = opcode.shift(self);
                if p1.get(self) == p2.get(self) {
                    p3.set(self, 1);
                } else {
                    p3.set(self, 0);
                }
                Ok(IntcodeStatus::Ok)
            }
            // adjust relbase
            9 => {
                let p1 = opcode.shift(self).get(self);
                self.base += p1;
                Ok(IntcodeStatus::Ok)
            }
            // halt
            99 => Ok(IntcodeStatus::Halt),
            // ???
            _ => Err(IntcodeError {
                pc: self.pc,
                why: format!("invalid opcode {}", opcode.opcode),
            }),
        }
    }

    pub fn run(&mut self) -> Result {
        loop {
            let res = self.step();
            if let Ok(IntcodeStatus::Ok) = res {
                continue;
            }
            return res;
        }
    }

    pub fn collect_output(&mut self) -> (String, IntcodeStatus) {
        let mut prompt = String::from("");
        loop {
            match self.run().unwrap() {
                IntcodeStatus::Output(output) => {
                    if output > 255 {
                        self.last_output = Some(output);
                    } else {
                        prompt.push(output as u8 as char);
                    }
                }
                s => return (prompt, s),
            }
        }
    }
}
