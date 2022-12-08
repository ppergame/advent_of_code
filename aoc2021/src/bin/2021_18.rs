use itertools::Itertools;

#[derive(Clone)]
pub enum Token {
    Open,
    Close,
    Comma,
    Num(i64),
}

impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Open => write!(f, "["),
            Self::Close => write!(f, "]"),
            Self::Comma => write!(f, ","),
            Self::Num(arg0) => write!(f, "{}", arg0),
        }
    }
}

#[derive(Clone)]
pub enum Pair {
    One(i64),
    Two(Box<Pair>, Box<Pair>),
}

impl std::fmt::Debug for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::One(arg0) => write!(f, "{}", arg0)?,
            Self::Two(arg0, arg1) => write!(f, "[{:?},{:?}]", arg0, arg1)?,
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct Input {
    lines: Vec<Vec<Token>>,
}

fn parse(inp: &str) -> Input {
    let lines = inp
        .lines()
        .map(|line| {
            let mut stack = vec![];
            let mut iter = line.chars().peekable();
            while let Some(c) = iter.next() {
                match c {
                    '0'..='9' => {
                        let mut s = c.to_string();
                        s.extend(iter.next_if(|c| c.is_ascii_digit()));
                        stack.push(Token::Num(s.parse().unwrap()))
                    }
                    '[' => stack.push(Token::Open),
                    ']' => stack.push(Token::Close),
                    ',' => stack.push(Token::Comma),
                    _ => (),
                }
            }
            stack
        })
        .collect();
    Input { lines }
}

fn find_explode(line: &[Token]) -> Option<usize> {
    let mut nest = 0;
    for (idx, t) in line.iter().enumerate() {
        match t {
            Token::Open => {
                nest += 1;
                if nest >= 5 {
                    if let (Token::Num(_), Token::Comma, Token::Num(_)) =
                        (&line[idx + 1], &line[idx + 2], &line[idx + 3])
                    {
                        return Some(idx);
                    }
                }
            }
            Token::Close => nest -= 1,
            Token::Comma => (),
            Token::Num(_) => (),
        }
    }
    None
}

fn squish(line: &mut Vec<Token>) {
    'outer: loop {
        if let Some(idx) = find_explode(line) {
            match (&line[idx + 1], &line[idx + 2], &line[idx + 3]) {
                (Token::Num(n1), Token::Comma, Token::Num(n2)) => {
                    let (n1, n2) = (*n1, *n2);
                    for t in &mut line[..idx].iter_mut().rev() {
                        if let Token::Num(f) = t {
                            *f += n1;
                            break;
                        }
                    }
                    for t in &mut line[idx + 4..].iter_mut() {
                        if let Token::Num(f) = t {
                            *f += n2;
                            break;
                        }
                    }
                    line.splice(idx..=idx + 4, [Token::Num(0)]);
                }
                _ => unreachable!(),
            }
            continue;
        }
        for (idx, t) in line.iter().enumerate() {
            if let Token::Num(n) = t {
                if *n >= 10 {
                    let n1 = n / 2;
                    let n2 = n - n1;
                    line.splice(
                        idx..idx + 1,
                        [
                            Token::Open,
                            Token::Num(n1),
                            Token::Comma,
                            Token::Num(n2),
                            Token::Close,
                        ],
                    );
                    continue 'outer;
                }
            }
        }
        break;
    }
}

#[allow(dead_code)]
fn print_line(line: &[Token]) {
    line.iter().for_each(|t| print!("{:?}", t));
    println!();
}

fn mag(line: &[Token]) -> i64 {
    let mut stack = vec![];
    for t in line {
        match t {
            Token::Num(n) => stack.push(*n),
            Token::Close => {
                let n2 = stack.pop().unwrap();
                let n1 = stack.pop().unwrap();
                stack.push(n1 * 3 + n2 * 2);
            }
            _ => (),
        }
    }
    assert_eq!(stack.len(), 1);
    stack.pop().unwrap()
}

fn sum_lines(l1: &[Token], l2: &[Token]) -> Vec<Token> {
    let mut res = vec![Token::Open];
    res.extend(l1.iter().cloned());
    res.push(Token::Comma);
    res.extend(l2.iter().cloned());
    res.push(Token::Close);
    squish(&mut res);
    res
}

fn part1(inp: &str) -> i64 {
    let inp = parse(inp);
    let sum = inp
        .lines
        .iter()
        .cloned()
        .reduce(|a1, a2| sum_lines(&a1, &a2))
        .unwrap();
    mag(&sum)
}

fn part2(inp: &str) -> i64 {
    let inp = parse(inp);
    let m1 = inp
        .lines
        .iter()
        .combinations(2)
        .map(|cc| mag(&sum_lines(cc[0], cc[1])))
        .max()
        .unwrap();
    let m2 = inp
        .lines
        .iter()
        .combinations(2)
        .map(|cc| mag(&sum_lines(cc[1], cc[0])))
        .max()
        .unwrap();
    std::cmp::max(m1, m2)
}

xaoc::xaoc!();
