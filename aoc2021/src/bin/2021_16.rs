use itertools::Itertools;

#[derive(Debug)]
struct Packet {
    version: i64,
    typ: i64,
    kind: Kind,
}

#[derive(Debug)]
enum Kind {
    Literal(i64),
    Operator(Vec<Packet>),
}

fn parse(inp: &str) -> String {
    inp.chars()
        .map(|c| format!("{:04b}", c.to_digit(16).unwrap()))
        .join("")
}

fn take<I: Iterator<Item = char>>(iter: &mut I, n: usize) -> String {
    iter.take(n).collect()
}

fn take_num<I: Iterator<Item = char>>(iter: &mut I, n: usize) -> i64 {
    i64::from_str_radix(&take(iter, n), 2).unwrap()
}

fn take_packet<I: Iterator<Item = char>>(iter: &mut I) -> Packet {
    let version = take_num(iter, 3);
    let typ = take_num(iter, 3);
    let kind = match typ {
        4 => {
            let mut s = String::new();
            loop {
                let sig = iter.next().unwrap();
                s += &take(iter, 4);
                if sig == '0' {
                    break;
                }
            }
            Kind::Literal(i64::from_str_radix(&s, 2).unwrap())
        }
        _ => {
            let i = iter.next().unwrap();
            let mut subpackets = vec![];
            if i == '0' {
                let len = take_num(iter, 15);
                let sub = take(iter, len as usize);
                let mut subiter = sub.chars().peekable();
                while subiter.peek().is_some() {
                    subpackets.push(take_packet(&mut subiter));
                }
            } else {
                let len = take_num(iter, 11);
                for _ in 0..len {
                    subpackets.push(take_packet(iter));
                }
            }
            Kind::Operator(subpackets)
        }
    };
    Packet { version, typ, kind }
}

fn sum_ver(packet: &Packet) -> i64 {
    packet.version
        + match &packet.kind {
            Kind::Literal(_) => 0,
            Kind::Operator(pp) => pp.iter().map(sum_ver).sum(),
        }
}

fn calc(packet: &Packet) -> i64 {
    match &packet.kind {
        Kind::Literal(i) => *i,
        Kind::Operator(pp) => {
            let mut nn = pp.iter().map(calc);
            match packet.typ {
                0 => nn.sum(),
                1 => nn.product(),
                2 => nn.min().unwrap(),
                3 => nn.max().unwrap(),
                5 => (nn.next().unwrap() > nn.next().unwrap()) as i64,
                6 => (nn.next().unwrap() < nn.next().unwrap()) as i64,
                7 => (nn.next().unwrap() == nn.next().unwrap()) as i64,
                _ => unreachable!(),
            }
        }
    }
}

fn part1(inp: &str) -> i64 {
    let inp = parse(inp);
    let mut iter = inp.chars().peekable();
    let packet = take_packet(&mut iter);
    sum_ver(&packet)
}

fn part2(inp: &str) -> i64 {
    let inp = parse(inp);
    let mut iter = inp.chars().peekable();
    let packet = take_packet(&mut iter);
    calc(&packet)
}

xaoc::xaoc!();
