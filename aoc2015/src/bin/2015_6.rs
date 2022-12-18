use array2d::Array2D;
use sscanf::scanf;

enum Instr {
    On,
    Off,
    Toggle,
}

impl Instr {
    fn parse(s: &str) -> Self {
        match s {
            "turn on" => Instr::On,
            "turn off" => Instr::Off,
            "toggle" => Instr::Toggle,
            _ => unreachable!(),
        }
    }
}

struct Cmd {
    instr: Instr,
    tl: (usize, usize),
    br: (usize, usize),
}

fn parse(inp: &str) -> Vec<Cmd> {
    inp.lines()
        .map(|line| {
            let (instr, x1, y1, x2, y2) =
                scanf!(line, "{str} {usize},{usize} through {usize},{usize}").unwrap();
            Cmd {
                instr: Instr::parse(instr),
                tl: (x1, y1),
                br: (x2, y2),
            }
        })
        .collect()
}

fn part1(inp: &str) -> usize {
    let cmds = parse(inp);
    let mut map = Array2D::filled_with(false, 1000, 1000);
    for cmd in cmds {
        for x in cmd.tl.0..=cmd.br.0 {
            for y in cmd.tl.1..=cmd.br.1 {
                match cmd.instr {
                    Instr::On => map[(x, y)] = true,
                    Instr::Off => map[(x, y)] = false,
                    Instr::Toggle => map[(x, y)] = !map[(x, y)],
                }
            }
        }
    }
    map.elements_row_major_iter().filter(|&&x| x).count()
}

fn part2(inp: &str) -> u64 {
    let cmds = parse(inp);
    let mut map = Array2D::filled_with(0u64, 1000, 1000);
    for cmd in cmds {
        for x in cmd.tl.0..=cmd.br.0 {
            for y in cmd.tl.1..=cmd.br.1 {
                match cmd.instr {
                    Instr::On => map.get_mut(x, y).map(|val| *val += 1),
                    Instr::Off => map.get_mut(x, y).map(|val| *val = val.saturating_sub(1)),
                    Instr::Toggle => map.get_mut(x, y).map(|val| *val += 2),
                };
            }
        }
    }
    map.elements_row_major_iter().sum()
}

xaoc::xaoc!(no_sample = true);
