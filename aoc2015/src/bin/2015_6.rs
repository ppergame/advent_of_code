use std::collections::HashMap;

struct Cmd {
    instr: String,
    tl: (usize, usize),
    br: (usize, usize),
}

fn parse(inp: &str) -> Vec<Cmd> {
    inp.lines()
        .map(|line| {
            let sp = line.split_whitespace().collect::<Vec<_>>();
            let spp = &sp[sp.len() - 4..sp.len()];
            let (x1, y1) = spp[1].split_once(',').unwrap();
            let (x2, y2) = spp[3].split_once(',').unwrap();
            Cmd {
                instr: spp[0].to_owned(),
                tl: (x1.parse().unwrap(), y1.parse().unwrap()),
                br: (x2.parse().unwrap(), y2.parse().unwrap()),
            }
        })
        .collect()
}

fn part1(inp: &str) -> usize {
    let cmds = parse(inp);
    let mut map = HashMap::new();
    for y in 0..1000 {
        for x in 0..1000 {
            map.insert((x, y), false);
        }
    }
    for cmd in cmds {
        for x in cmd.tl.0..=cmd.br.0 {
            for y in cmd.tl.1..=cmd.br.1 {
                match cmd.instr.as_str() {
                    "on" => map.insert((x, y), true),
                    "off" => map.insert((x, y), false),
                    "toggle" => {
                        let val = map[&(x, y)];
                        map.insert((x, y), !val)
                    }
                    _ => unreachable!(),
                };
            }
        }
    }
    map.values().filter(|&&x| x).count()
}

fn part2(inp: &str) -> usize {
    let cmds = parse(inp);
    let mut map = HashMap::new();
    for y in 0..1000 {
        for x in 0..1000 {
            map.insert((x, y), 0);
        }
    }
    for cmd in cmds {
        for x in cmd.tl.0..=cmd.br.0 {
            for y in cmd.tl.1..=cmd.br.1 {
                match cmd.instr.as_str() {
                    "on" => {
                        let val = map.get_mut(&(x, y)).unwrap();
                        *val += 1;
                    }
                    "off" => {
                        let val = map.get_mut(&(x, y)).unwrap();
                        if *val > 0 {
                            *val -= 1;
                        }
                    }
                    "toggle" => {
                        let val = map.get_mut(&(x, y)).unwrap();
                        *val += 2;
                    }
                    _ => unreachable!(),
                };
            }
        }
    }
    map.values().sum()
}

xaoc::xaoc!();
