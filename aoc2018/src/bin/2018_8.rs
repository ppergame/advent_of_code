use itertools::Itertools;

struct Node {
    metadata: Vec<usize>,
    value: usize,
}

impl Node {
    fn new(nodes: &[Node], children: Vec<usize>, metadata: Vec<usize>) -> Self {
        let value = if children.is_empty() {
            metadata.iter().sum()
        } else {
            metadata
                .iter()
                .map(|&m| {
                    if m == 0 {
                        return 0;
                    }
                    match children.get(m - 1) {
                        Some(&c) => nodes[c].value,
                        None => 0,
                    }
                })
                .sum()
        };
        Self { metadata, value }
    }
}

#[derive(Default)]
struct Nodes(Vec<Node>);

impl Nodes {
    fn read_node(&mut self, iter: &mut impl Iterator<Item = usize>) -> usize {
        let child_count = iter.next().unwrap();
        let metadata_count = iter.next().unwrap();
        let children = (0..child_count).map(|_| self.read_node(iter)).collect_vec();
        let metadata = (0..metadata_count)
            .map(|_| iter.next().unwrap())
            .collect_vec();
        let ret = self.0.len();
        self.0.push(Node::new(&self.0, children, metadata));
        ret
    }
}

fn part1(inp: &str) -> usize {
    let mut nodes = Nodes::default();
    let mut iter = inp
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .peekable();
    while iter.peek().is_some() {
        nodes.read_node(&mut iter);
    }
    nodes
        .0
        .into_iter()
        .map(|n| n.metadata.into_iter().sum::<usize>())
        .sum()
}

fn part2(inp: &str) -> usize {
    let mut nodes = Nodes::default();
    let mut iter = inp
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .peekable();
    while iter.peek().is_some() {
        nodes.read_node(&mut iter);
    }
    nodes.0.last().unwrap().value
}

xaoc::xaoc!(sample = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2");
