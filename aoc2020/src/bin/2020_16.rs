use std::collections::HashMap;

use bitvec::prelude::*;

struct Field {
    name: String,
    ranges: Vec<std::ops::RangeInclusive<usize>>,
}

impl Field {
    fn ok(&self, num: usize) -> bool {
        self.ranges.iter().any(|r| r.contains(&num))
    }
}

struct Problem {
    fields: Vec<Field>,
    my_ticket: Vec<usize>,
    tickets: Vec<Vec<usize>>,
}

impl Problem {
    fn new(inp: &str) -> Problem {
        let mut prob = Problem {
            fields: Vec::new(),
            my_ticket: Vec::new(),
            tickets: Vec::new(),
        };
        let mut lines = inp.lines();
        for line in &mut lines {
            if line.is_empty() {
                break;
            }
            let (name, ranges) = line.split_once(": ").unwrap();
            let ranges = ranges
                .split(" or ")
                .map(|rt| {
                    let (start, end) = rt.split_once('-').unwrap();
                    start.parse().unwrap()..=end.parse().unwrap()
                })
                .collect();
            prob.fields.push(Field {
                name: name.to_string(),
                ranges,
            });
        }
        assert_eq!(lines.next().unwrap(), "your ticket:");
        prob.my_ticket = lines
            .next()
            .unwrap()
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect();
        assert_eq!(lines.next().unwrap(), "");
        assert_eq!(lines.next().unwrap(), "nearby tickets:");
        prob.tickets = lines
            .map(|line| line.split(',').map(|x| x.parse().unwrap()).collect())
            .collect();
        prob
    }

    fn validate(&self) -> usize {
        self.tickets
            .iter()
            .flatten()
            .filter(|&&x| !self.is_valid(x))
            .sum()
    }

    fn clean(&mut self) {
        let mut tickets = std::mem::take(&mut self.tickets);
        tickets.retain(|t| !t.iter().any(|&x| !self.is_valid(x)));
        self.tickets = tickets;
    }

    fn is_valid(&self, x: usize) -> bool {
        for field in &self.fields {
            for r in &field.ranges {
                if r.contains(&x) {
                    return true;
                }
            }
        }
        false
    }
}

fn part1(inp: &str) -> usize {
    let prob = Problem::new(inp);
    prob.validate()
}

fn part2(inp: &str) -> usize {
    let mut prob = Problem::new(inp);
    prob.clean();
    assert_eq!(prob.fields.len(), prob.my_ticket.len());
    assert_eq!(prob.fields.len(), prob.tickets[0].len());
    // maps field index to spots where it's ok to use
    let mut fields_ok = HashMap::new();
    for (i, bv) in std::iter::repeat_with(|| bitvec![1; prob.fields.len()])
        .enumerate()
        .take(prob.fields.len())
    {
        fields_ok.insert(i, bv);
    }
    for ticket in &prob.tickets {
        for (field_i, field) in prob.fields.iter().enumerate() {
            for (i, &num) in ticket.iter().enumerate() {
                if !field.ok(num) {
                    *fields_ok.get_mut(&field_i).unwrap().get_mut(i).unwrap() = false;
                }
            }
        }
    }
    let mut fields = HashMap::new();
    while !fields_ok.is_empty() {
        let mut found = None;
        for (&field_i, slots) in &fields_ok {
            if slots.count_ones() == 1 {
                found = Some(field_i);
                break;
            }
        }
        let field_i = found.unwrap();
        let slots = fields_ok.remove(&field_i).unwrap();
        let slot = slots.into_iter().enumerate().find(|(_, x)| *x).unwrap().0;
        fields.insert(field_i, slot);
        for slots in fields_ok.values_mut() {
            *slots.get_mut(slot).unwrap() = false;
        }
    }
    let mut mul = 1;
    for (field_i, field) in prob.fields.iter().enumerate() {
        if field.name.starts_with("departure") {
            mul *= prob.my_ticket[fields[&field_i]];
        }
    }
    mul
}

xaoc::xaoc!();
