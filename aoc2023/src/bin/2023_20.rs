use sscanf::scanf;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
enum ModuleKind {
    Flip(bool),
    And(HashMap<String, bool>),
    Bro,
}

#[derive(Debug)]
struct Module {
    kind: ModuleKind,
    outputs: Vec<String>,
}

#[derive(Debug)]
struct Modules {
    mods: HashMap<String, Module>,
    pre_rx: String,
}

impl Modules {
    fn parse(inp: &str) -> Modules {
        let mut mods = inp
            .lines()
            .map(|line| {
                let (from, outputs) = scanf!(line, "{str} -> {str}").unwrap();
                let (kind, name) = if from == "broadcaster" {
                    (ModuleKind::Bro, from.to_owned())
                } else {
                    let mut from = from.chars();
                    (
                        match from.next().unwrap() {
                            '%' => ModuleKind::Flip(false),
                            '&' => ModuleKind::And(HashMap::new()),
                            _ => unreachable!(),
                        },
                        from.collect::<String>(),
                    )
                };
                let outputs = outputs.split(',').map(|s| s.trim().to_owned()).collect();
                (name.clone(), Module { kind, outputs })
            })
            .collect::<HashMap<_, _>>();
        let mut inputs = HashMap::new();
        let mut pre_rx = None;
        for (name, module) in mods.iter_mut() {
            for output in module.outputs.iter() {
                if output == "rx" {
                    pre_rx = Some(name.clone());
                }
                inputs
                    .entry(output.clone())
                    .or_insert_with(Vec::new)
                    .push(name.clone());
            }
        }
        for (name, inputs) in inputs {
            if let Some(module) = mods.get_mut(&name) {
                if let ModuleKind::And(map) = &mut module.kind {
                    for input in inputs {
                        map.insert(input, false);
                    }
                }
            }
        }
        Modules {
            mods,
            pre_rx: pre_rx.unwrap(),
        }
    }

    // returns low count, high count, names of rx's input's inputs that received high pulse
    fn pulse(&mut self) -> (usize, usize, Vec<String>) {
        let mut low = 0;
        let mut high = 0;
        let mut rx_inputs = vec![];
        let mut todo = VecDeque::new();
        todo.push_back((false, "button".to_owned(), "broadcaster".to_owned()));
        while let Some((pulse, from, modname)) = todo.pop_front() {
            if pulse {
                high += 1;
            } else {
                low += 1;
            }
            let Some(module) = self.mods.get_mut(&modname) else {
                continue;
            };
            if modname == self.pre_rx && pulse {
                rx_inputs.push(from.clone());
            }
            match &mut module.kind {
                ModuleKind::Flip(state) => {
                    if !pulse {
                        *state = !*state;
                        for output in &module.outputs {
                            todo.push_back((*state, modname.clone(), output.clone()));
                        }
                    }
                }
                ModuleKind::And(map) => {
                    *map.get_mut(&from).unwrap() = pulse;
                    let res = !map.values().all(|&v| v);
                    for output in &module.outputs {
                        todo.push_back((res, modname.clone(), output.clone()));
                    }
                }
                ModuleKind::Bro => {
                    for output in &module.outputs {
                        todo.push_back((pulse, modname.clone(), output.clone()));
                    }
                }
            }
            // todo!();
        }
        // todo!();
        (low, high, rx_inputs)
    }
}

fn part1(inp: &str) -> usize {
    let mut mods = Modules::parse(inp);
    let mut slow = 0;
    let mut shigh = 0;
    for _ in 0..1000 {
        let (low, high, _) = mods.pulse();
        slow += low;
        shigh += high;
    }
    // todo!("{slow} {shigh}");
    slow * shigh
}

fn part2(inp: &str) -> usize {
    let mut mods = Modules::parse(inp);
    let mut prevs = HashMap::<String, usize>::new();
    let mut cycles = HashMap::<String, usize>::new();
    for i in 0..10000 {
        let (_, _, rx_inputs) = mods.pulse();
        for input in rx_inputs {
            match (prevs.entry(input.clone()), cycles.entry(input.clone())) {
                (Occupied(mut po), Occupied(oc)) => {
                    let old = *oc.get();
                    let new = i - po.get();
                    if old != new {
                        panic!("irregular cycle for {input} @ {i}: {old} != {new}");
                    }
                    po.insert(i);
                }
                (Occupied(mut po), Vacant(vc)) => {
                    vc.insert(i - po.get());
                    po.insert(i);
                }
                (Vacant(pv), _) => {
                    pv.insert(i);
                }
            }
        }
    }
    cycles.values().product()
}

xaoc::xaoc!(no_sample = true);
