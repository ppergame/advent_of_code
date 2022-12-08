use aoc2019::intcode::*;
use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet};

lazy_static::lazy_static! {
    static ref WIN_RE: Regex =
        Regex::new(r"You should be able to get in by typing (\d+) on the keypad").unwrap();
}

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
enum Dir {
    N,
    S,
    E,
    W,
}

impl Dir {
    fn opp(self) -> Dir {
        match self {
            Dir::N => Dir::S,
            Dir::S => Dir::N,
            Dir::E => Dir::W,
            Dir::W => Dir::E,
        }
    }

    fn to_command(self) -> String {
        match self {
            Dir::N => "north".to_string(),
            Dir::S => "south".to_string(),
            Dir::E => "east".to_string(),
            Dir::W => "west".to_string(),
        }
    }
}

struct RoomDesc {
    title: String,
    #[allow(dead_code)]
    desc: String,
    exits: HashSet<Dir>,
    items: HashSet<String>,
}

impl RoomDesc {
    fn new(output: &str) -> RoomDesc {
        let mut lines = output.lines();
        let title = lines
            .find(|l| l.starts_with("== "))
            .unwrap()
            .trim_start_matches("== ")
            .trim_end_matches(" ==")
            .to_string();
        let desc = lines.next().unwrap().to_string();
        lines.find(|l| l == &"Doors here lead:");
        let mut exits = HashSet::<Dir>::new();
        for l in &mut lines {
            match l {
                "" => break,
                "- north" => exits.insert(Dir::N),
                "- south" => exits.insert(Dir::S),
                "- east" => exits.insert(Dir::E),
                "- west" => exits.insert(Dir::W),
                _ => panic!("unexpected line {}", l),
            };
        }
        let mut section = lines.next().unwrap();
        let mut items = HashSet::<String>::new();
        if section == "Items here:" {
            for l in &mut lines {
                if l.is_empty() {
                    break;
                }
                if !l.starts_with("- ") {
                    panic!("unexpected item line {}", l);
                }
                items.insert(l[2..].to_string());
            }
            section = lines.next().unwrap();
        }
        assert_eq!(section, "Command?");
        RoomDesc {
            title,
            desc,
            exits,
            items,
        }
    }
}

struct Room {
    // if titles duplicate, might need unique room id
    // somehow merge dupe maze rooms?
    rdesc: RoomDesc,
    // direction to neighbor title. Unexplored exits are in rdesc, but not exits.
    exits: HashMap<Dir, String>,
}

struct Map {
    prog: Vec<i64>,
    rooms: HashMap<String, Room>,
}

impl Map {
    fn new(prog: Vec<i64>) -> Map {
        Map {
            prog,
            rooms: HashMap::new(),
        }
    }

    fn add_or_get_room(&mut self, rdesc: RoomDesc) -> &mut Room {
        let title = rdesc.title.to_string();
        if !self.rooms.contains_key(&title) {
            let room = Room {
                rdesc,
                exits: HashMap::new(),
            };
            self.rooms.insert(title.to_string(), room);
        }
        self.rooms.get_mut(&title).unwrap()
    }

    fn command(ic: &mut Intcode, cmd: &str) -> Result<String, usize> {
        for c in cmd.chars() {
            ic.input = Some(c as i64);
            assert!(matches!(ic.run().unwrap(), IntcodeStatus::Input));
        }
        ic.input = Some('\n' as i64);
        let (output, status) = ic.collect_output();
        //println!("{}", output);
        if let Some(captures) = WIN_RE.captures(&output) {
            return Err(captures[1].parse().unwrap());
        }
        assert!(matches!(status, IntcodeStatus::Input));
        assert!(output.ends_with("Command?\n"));
        Ok(output)
    }

    fn go(ic: &mut Intcode, dir: Dir) -> Result<RoomDesc, usize> {
        let output = Self::command(ic, &dir.to_command())?;
        Ok(RoomDesc::new(&output))
    }

    fn explore(&mut self) -> Result<(), usize> {
        // backtrack stack of (dir, room title) pairs, last item is current room
        // for current room
        //   unexplored exits?
        //     take exit
        //     parse next room
        //     add link from next to current
        //     put next room into rooms if needed
        //     push next room
        //   no unexplored exits? pop room, go to previous
        //   no rooms to pop? done, assert in "Hull Breach"
        let mut stack = Vec::<(Option<Dir>, String)>::new();
        let mut ic = Intcode::new_with_seq(&self.prog);
        let (output, status) = ic.collect_output();
        assert!(matches!(status, IntcodeStatus::Input));
        let rdesc = RoomDesc::new(&output);
        //println!("{}", output);
        stack.push((None, rdesc.title.to_string()));
        self.add_or_get_room(rdesc);

        let mut to_checkpoint = Vec::<Dir>::new();
        let mut items = HashSet::<String>::new();

        while !stack.is_empty() {
            let (last_dir, last_title) = stack.last().unwrap();

            if last_title == "Security Checkpoint" {
                to_checkpoint = stack.iter().filter_map(|(d, _)| *d).collect::<Vec<_>>();
            }

            let room = self.rooms.get_mut(last_title).unwrap();

            match room
                .rdesc
                .exits
                .iter()
                .find(|e| !room.exits.contains_key(e))
            {
                Some(&exit) if room.rdesc.title != "Security Checkpoint" => {
                    let mut rdesc = Self::go(&mut ic, exit)?;
                    for item in rdesc.items.drain() {
                        if item == "infinite loop"
                            || item == "giant electromagnet"
                            || item == "escape pod"
                            || item == "photons"
                            || item == "molten lava"
                        {
                            continue;
                        }
                        Self::command(&mut ic, &format!("take {}", item))?;
                        items.insert(item);
                    }
                    let new_title = rdesc.title.to_string();
                    let room = self.rooms.get_mut(last_title).unwrap();
                    room.exits.insert(exit, new_title.to_string());
                    let new_room = self.add_or_get_room(rdesc);
                    new_room.exits.insert(exit.opp(), last_title.to_string());
                    stack.push((Some(exit), new_title));
                }
                _ => {
                    if let Some(dir) = last_dir {
                        Self::go(&mut ic, dir.opp())?;
                    }
                    stack.pop();
                    continue;
                }
            }
        }

        assert!(!to_checkpoint.is_empty());
        for dir in to_checkpoint {
            Self::go(&mut ic, dir)?;
        }
        let room = self.rooms.get("Security Checkpoint").unwrap();
        let &dir = room
            .rdesc
            .exits
            .iter()
            .find(|e| !room.exits.contains_key(e))
            .unwrap();
        let items = items.into_iter().collect::<Vec<_>>();
        for i in 1..items.len() {
            for wanted_indexes in (0..items.len()).permutations(i) {
                let wanted = wanted_indexes.into_iter().collect::<HashSet<_>>();
                for (idx, item) in items.iter().enumerate() {
                    if wanted.contains(&idx) {
                        Self::command(&mut ic, &format!("take {}", item))?;
                    } else {
                        Self::command(&mut ic, &format!("drop {}", item))?;
                    }
                }
                Self::command(&mut ic, &dir.to_command())?;
            }
        }
        Ok(())
    }
}

fn part1(inp: &str) -> usize {
    let prog = inp
        .split(',')
        .map(|i| i.parse().unwrap())
        .collect::<Vec<i64>>();
    let mut map = Map::new(prog);
    map.explore().unwrap_err()
}

fn part2(_inp: &str) -> i64 {
    0
}

xaoc::xaoc!();
