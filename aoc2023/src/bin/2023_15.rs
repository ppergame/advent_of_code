use indexmap::IndexMap;
use sscanf::scanf;

fn hash(s: &str) -> usize {
    let mut cur = 0;
    for c in s.chars() {
        cur += c as u8 as usize;
        cur *= 17;
        cur %= 256;
    }
    cur
}

fn part1(inp: &str) -> usize {
    let mut ret = 0;
    for step in inp.split(',') {
        ret += hash(step);
    }
    ret
}

#[derive(Debug, Default, Clone)]
struct Box(IndexMap<String, usize>);

impl Box {
    fn insert(&mut self, label: &str, focal: usize) {
        match self.0.get_mut(label) {
            Some(old_focal) => *old_focal = focal,
            None => {
                self.0.insert(label.to_owned(), focal);
            }
        }
    }

    fn remove(&mut self, label: &str) {
        self.0.shift_remove(label);
    }

    fn score(&self) -> usize {
        self.0
            .iter()
            .enumerate()
            .map(|(idx, (_, focal))| (idx + 1) * focal)
            .sum()
    }
}

fn part2(inp: &str) -> usize {
    let mut boxes = vec![Box::default(); 256];
    for step in inp.split(',') {
        if let Ok(label) = scanf!(step, "{str}-") {
            boxes.get_mut(hash(label)).unwrap().remove(label);
        } else if let Ok((label, focal)) = scanf!(step, "{str}={usize}") {
            boxes.get_mut(hash(label)).unwrap().insert(label, focal);
        } else {
            unreachable!();
        }
    }
    boxes
        .iter()
        .enumerate()
        .map(|(idx, b)| (idx + 1) * b.score())
        .sum()
}

xaoc::xaoc!(sample = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7");
