#[derive(Clone, Copy, Debug)]
enum Op {
    Mul,
    Add,
}

impl Op {
    fn apply(self, op1: u64, op2: u64) -> u64 {
        match self {
            Op::Mul => op1 * op2,
            Op::Add => op1 + op2,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Token {
    Number(u64),
    Op(Op),
    Push,
    Pop,
}

struct Tokenizer<'a> {
    it: Box<dyn Iterator<Item = char> + 'a>,
}

impl Tokenizer<'_> {
    fn new(inp: &str) -> Tokenizer {
        Tokenizer {
            it: Box::new(inp.chars()),
        }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        for c in &mut self.it {
            match c {
                d @ '0'..='9' => return Some(Token::Number(d.to_digit(10).unwrap() as u64)),
                '+' => return Some(Token::Op(Op::Add)),
                '*' => return Some(Token::Op(Op::Mul)),
                '(' => return Some(Token::Push),
                ')' => return Some(Token::Pop),
                ' ' => continue,
                _ => panic!(),
            }
        }
        None
    }
}

struct Eval<'a> {
    prec: bool,
    t: Tokenizer<'a>,
    val_stack: Vec<u64>,
    op_stack: Vec<Token>,
}

impl Eval<'_> {
    fn new(inp: &str, prec: bool) -> Eval {
        Eval {
            prec,
            t: Tokenizer::new(inp),
            val_stack: Vec::new(),
            op_stack: Vec::new(),
        }
    }

    fn process(&mut self) {
        while let Some(Token::Op(op)) = self.op_stack.last() {
            let op2 = self.val_stack.pop().unwrap();
            let op1 = self.val_stack.pop().unwrap();
            self.val_stack.push(op.apply(op1, op2));
            self.op_stack.pop().unwrap();
        }
    }

    fn run(&mut self) -> u64 {
        while let Some(t) = self.t.next() {
            match t {
                Token::Number(val) => self.val_stack.push(val),
                Token::Op(op) => {
                    if !self.prec
                        || (matches!(op, Op::Mul)
                            && matches!(self.op_stack.last(), Some(Token::Op(Op::Add))))
                    {
                        self.process();
                    }
                    self.op_stack.push(t);
                }
                Token::Push => self.op_stack.push(t),
                Token::Pop => {
                    self.process();
                    assert!(matches!(self.op_stack.pop(), Some(Token::Push)));
                }
            }
            //println!("{:?} {:?} {:?}", t, self.val_stack, self.op_stack);
        }
        self.process();
        assert_eq!(self.val_stack.len(), 1);
        self.val_stack.pop().unwrap()
    }
}

fn part1(inp: &str) -> u64 {
    inp.lines()
        .map(|line| {
            let mut e = Eval::new(line, false);
            e.run()
        })
        .sum()
}

fn part2(inp: &str) -> u64 {
    inp.lines()
        .map(|line| {
            let mut e = Eval::new(line, true);
            e.run()
        })
        .sum()
}

xaoc::xaoc!();
