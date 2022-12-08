use itertools::Itertools;
use sscanf::scanf;
use std::collections::HashMap;

struct Node {
    parent: usize,
    typ: Type,
}

enum Type {
    Dir(HashMap<String, usize>),
    File(i64),
}

struct Tree {
    nodes: Vec<Node>,
}

impl Tree {
    fn parse(inp: &str) -> Self {
        let node = Node {
            parent: 0,
            typ: Type::Dir(HashMap::new()),
        };
        let mut nodes = vec![node];
        let mut cur_dir = 0;
        let mut iter = inp.lines().peekable();
        assert_eq!(iter.next(), Some("$ cd /"));
        while let Some(line) = iter.next() {
            if line == "$ ls" {
                while !iter.peek().map_or(true, |l| l.starts_with("$ ")) {
                    let line = iter.next().unwrap();
                    let next_inode = nodes.len();
                    let node = &mut nodes[cur_dir];
                    let Type::Dir(ref mut dir) = &mut node.typ else { unreachable!() };
                    if let Ok(name) = scanf!(line, "dir {}", str) {
                        dir.insert(name.to_string(), next_inode);
                        nodes.push(Node {
                            parent: cur_dir,
                            typ: Type::Dir(HashMap::new()),
                        });
                    } else if let Ok((size, name)) = scanf!(line, "{} {}", i64, str) {
                        dir.insert(name.to_string(), next_inode);
                        nodes.push(Node {
                            parent: cur_dir,
                            typ: Type::File(size),
                        });
                    } else {
                        unreachable!();
                    }
                }
            } else if let Ok(name) = scanf!(line, "$ cd {}", str) {
                let node = &mut nodes[cur_dir];
                match name {
                    "/" => cur_dir = 0,
                    ".." => cur_dir = node.parent,
                    x => {
                        let Type::Dir(dir) = &node.typ else { unreachable!() };
                        cur_dir = dir[x];
                    }
                }
            } else {
                unreachable!();
            }
        }
        Tree { nodes }
    }

    fn calc_size(&self, inode: usize, total_sizes: &mut HashMap<usize, i64>) -> i64 {
        let node = &self.nodes[inode];
        match &node.typ {
            Type::Dir(d) => {
                let mut total = 0;
                for child in d.values() {
                    total += self.calc_size(*child, total_sizes);
                }
                total_sizes.insert(inode, total);
                total
            }
            Type::File(s) => *s,
        }
    }
}

fn part1(inp: &str) -> i64 {
    let tree = Tree::parse(inp);
    let mut total_sizes = HashMap::<usize, i64>::new();
    tree.calc_size(0, &mut total_sizes);
    let mut total = 0;
    for &size in total_sizes.values() {
        if size <= 100000 {
            total += size;
        }
    }
    total
}

fn part2(inp: &str) -> i64 {
    let tree = Tree::parse(inp);
    let mut total_sizes = HashMap::<usize, i64>::new();
    tree.calc_size(0, &mut total_sizes);
    let free = 70000000 - total_sizes[&0];
    let need = 30000000 - free;
    *total_sizes.values().sorted().find(|s| **s > need).unwrap()
}

xaoc::xaoc!(sample_idx = 1);
