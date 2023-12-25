use petgraph::Graph;
use rand::prelude::*;
use rayon::prelude::*;
use std::collections::HashMap;

fn part1(inp: &str) -> usize {
    let mut names = HashMap::new();
    let mut graph = Graph::new_undirected();
    // gather names
    for line in inp.lines() {
        let (name, rest) = line.split_once(": ").unwrap();
        for part in rest.split_ascii_whitespace().chain(std::iter::once(name)) {
            names.entry(part).or_insert_with(|| graph.add_node(1));
        }
    }
    // build graph
    for line in inp.lines() {
        let (name, rest) = line.split_once(": ").unwrap();
        let idx1 = names[name];
        for part in rest.split_ascii_whitespace() {
            let idx2 = names[part];
            graph.add_edge(idx1, idx2, ());
        }
    }

    (0..8192)
        .into_par_iter()
        .find_map_any(|_| {
            let mut graph = graph.clone();
            while graph.node_count() > 2 {
                let num = rand::thread_rng().gen_range(0..graph.edge_count());
                let (idx1, idx2) = graph
                    .edge_indices()
                    .nth(num)
                    .map(|idx| graph.edge_endpoints(idx).unwrap())
                    .unwrap();
                let sum = graph[idx1] + graph[idx2];
                graph[idx1] = sum;
                let mut neighbors = graph.neighbors(idx2).detach();
                while let Some(idx) = neighbors.next_node(&graph) {
                    if idx == idx1 {
                        continue;
                    }
                    graph.add_edge(idx1, idx, ());
                }
                graph.remove_node(idx2);
            }
            if graph.edge_count() == 3 {
                Some(graph.node_weights().product())
            } else {
                None
            }
        })
        .unwrap()
}

fn part2(_inp: &str) -> usize {
    0
}

xaoc::xaoc!(
    sample = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr"
);
