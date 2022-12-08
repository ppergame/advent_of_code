use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

use itertools::Itertools;

fn parse(inp: &str) -> Vec<usize> {
    inp.chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect()
}

fn part1(inp: &str) -> String {
    let nums = parse(inp);
    let mut g = Game::new(&nums, nums.len());
    for _ in 0..100 {
        g.step();
    }
    g.iter_from1()
        .skip(1)
        .take(nums.len() - 1)
        .map(|x| x.to_string())
        .join("")
}

type Link = Rc<RefCell<Node>>;

struct NodeIterator {
    cur: Link,
}

impl Iterator for NodeIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.cur.borrow().val;
        let next = self.cur.borrow().next();
        self.cur = next;
        Some(ret)
    }
}

struct Node {
    val: usize,
    next: Option<Link>,
}

impl Node {
    fn next(&self) -> Rc<RefCell<Node>> {
        self.next.as_ref().unwrap().clone()
    }
}

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node")
            .field("val", &self.val)
            .field(
                "next",
                &match &self.next {
                    Some(rc) => format!("0x{:x}", rc.as_ptr() as u64),
                    None => "null".to_string(),
                },
            )
            .finish()
    }
}

#[derive(Debug)]
struct Game {
    cur: Link,
    idx: Vec<Option<Link>>,
}

impl Game {
    fn iter_from1(&self) -> NodeIterator {
        NodeIterator {
            cur: self.idx[1].as_ref().unwrap().clone(),
        }
    }

    fn new(nums: &[usize], len: usize) -> Game {
        let mut idx = vec![None; len + 1];
        let mut head: Option<Link> = None;
        let mut prev: Option<Link> = None;
        for &num in nums {
            let node = Rc::new(RefCell::new(Node {
                val: num,
                next: None,
            }));
            if head.is_none() {
                head = Some(node.clone());
            }
            if let Some(prev_node) = prev {
                prev_node.borrow_mut().next = Some(node.clone());
            }
            idx[num] = Some(node.clone());
            prev = Some(node);
        }
        for (num, idx_mut) in idx
            .iter_mut()
            .enumerate()
            .take(len + 1)
            .skip(nums.len() + 1)
        {
            let node = Rc::new(RefCell::new(Node {
                val: num,
                next: None,
            }));
            prev.unwrap().borrow_mut().next = Some(node.clone());
            *idx_mut = Some(node.clone());
            prev = Some(node);
        }
        prev.unwrap().borrow_mut().next = Some(head.as_ref().unwrap().clone());
        Game {
            cur: head.unwrap(),
            idx,
        }
    }

    fn step(&mut self) {
        let cur = self.cur.clone();
        let grab_first = cur.borrow().next();
        let mut grab_last = cur.clone();
        let mut grabbed = HashSet::new();
        for _ in 0..3 {
            let next = grab_last.borrow().next();
            grabbed.insert(next.borrow().val);
            grab_last = next;
        }
        //println!("{} {}", cur.borrow().val, grab_last.borrow().val);
        cur.borrow_mut().next = Some(grab_last.borrow().next());
        let mut val = cur.borrow().val;
        let dest;
        loop {
            val = if val == 1 {
                self.idx.len() - 1
            } else {
                val - 1
            };
            if grabbed.contains(&val) {
                continue;
            }
            dest = self.idx[val].as_ref().unwrap().clone();
            break;
        }
        let dest_next = dest.borrow().next();
        dest.borrow_mut().next = Some(grab_first);
        grab_last.borrow_mut().next = Some(dest_next);
        let next = self.cur.borrow().next();
        self.cur = next
    }
}

fn part2(inp: &str) -> usize {
    let nums = parse(inp);
    let mut g = Game::new(&nums, 1000000);
    for _ in 0..10000000 {
        g.step();
    }
    g.iter_from1().skip(1).take(2).product()
}

xaoc::xaoc!();
