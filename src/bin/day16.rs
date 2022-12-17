use std::{cell::Cell, collections::VecDeque};

use ahash::{AHashMap, AHashSet};

#[derive(Debug, PartialEq, Clone)]
struct Node {
    id: usize,
    name: String,
    flow: i64,
    open: Cell<bool>,
    adjacent: Vec<String>,
    adjacent_id: Vec<usize>,
}

struct NodeCache {
    data: Vec<i64>,
}

impl NodeCache {
    fn new() -> Self {
        NodeCache { data: vec![] }
    }

    fn idx(from: usize, to: usize) -> usize {
        assert!(from < 64);
        assert!(to < 64);

        if from < to {
            ((from & 0x3F) << 6) | (to & 0x3F)
        } else {
            ((to & 0x3F) << 6) | (from & 0x3F)
        }
    }

    fn insert(&mut self, from: usize, to: usize, value: i64) {
        let idx = Self::idx(from, to);
        if idx >= self.data.len() {
            self.data.resize(idx + 1, -1);
        }
        self.data[idx] = value;
    }

    fn lookup(&self, from: usize, to: usize) -> Option<i64> {
        let idx = Self::idx(from, to);
        if idx >= self.data.len() {
            None
        } else {
            let value = self.data[idx];
            if value == -1 {
                None
            } else {
                Some(value)
            }
        }
    }
}

fn parse(input: &str) -> AHashMap<usize, Node> {
    let mut nodes = AHashMap::new();
    let mut adj = AHashMap::new();

    for (id, line) in input.lines().enumerate() {
        let mut words = line.split_ascii_whitespace();
        words.next();
        let name = words.next().unwrap().to_owned();
        words.next();
        words.next();
        let flow = words.next().unwrap();
        let flow: i64 = flow
            .trim_start_matches("rate=")
            .trim_end_matches(';')
            .parse()
            .unwrap();
        words.next();
        words.next();
        words.next();
        words.next();

        let adjacent: Vec<_> =
            words.map(|w| w.trim_end_matches(',').to_owned()).collect();

        adj.insert(name.clone(), id);

        nodes.insert(
            id,
            Node {
                id,
                name,
                flow,
                open: Cell::new(false),
                adjacent,
                adjacent_id: vec![],
            },
        );
    }

    for node in nodes.values_mut() {
        for adj_name in &node.adjacent {
            node.adjacent_id.push(*adj.get(adj_name).unwrap());
        }
    }

    nodes
}

fn optimize(
    nodes: AHashMap<usize, Node>,
    cache: &mut NodeCache,
) -> AHashMap<usize, Node> {
    // warm the cache 🔥
    for (from, _) in &nodes {
        for (to, _) in &nodes {
            dist_valve(*from, *to, &nodes, cache);
        }
    }

    let mut trimmed_nodes = AHashMap::default();

    // The cache is perfect, so we don't need dist_value now.
    for (name, node) in nodes {
        if node.flow != 0 {
            trimmed_nodes.insert(name, node);
        }
    }

    trimmed_nodes
}

fn dist_valve(
    current_node: usize,
    dest: usize,
    nodes: &AHashMap<usize, Node>,
    cache: &mut NodeCache,
) -> i64 {
    if let Some(dist) = cache.lookup(current_node, dest) {
        return dist;
    }

    let mut expanded = AHashSet::new();
    let mut unexpanded = VecDeque::<(usize, i64)>::from([(current_node, 0)]);
    let total_dist;

    loop {
        let (node_id, dist) = unexpanded.pop_back().unwrap();
        if node_id == dest {
            total_dist = dist;
            break;
        }
        if expanded.contains(&node_id) {
            continue;
        }

        if let Some(node) = nodes.get(&node_id) {
            for adj in &node.adjacent_id {
                unexpanded.push_front((*adj, dist + 1));
            }

            expanded.insert(node.id);
        }
    }

    cache.insert(current_node, dest, total_dist);
    total_dist
}

fn part1_solve(
    start: &Node,
    minute: i64,
    nodes: &AHashMap<usize, Node>,
    cache: &mut NodeCache,
) -> i64 {
    let mut pressure = 0;

    if minute > 30 {
        return pressure;
    }

    for (_, node) in nodes {
        if node.open.get() || node.flow == 0 {
            continue;
        }

        let dist = dist_valve(start.id, node.id, nodes, cache);
        node.open.set(true);
        pressure = i64::max(
            pressure,
            (30 - minute - dist - 1) * node.flow
                + part1_solve(node, minute + dist + 1, nodes, cache),
        );
        node.open.set(false);
    }

    pressure
}

fn part1(nodes: &AHashMap<usize, Node>) -> i64 {
    let (id, _) = nodes.iter().find(|(_, node)| node.name == "AA").unwrap();
    let start_node = nodes.get(id).unwrap();
    let mut dist_cache = NodeCache::new();
    part1_solve(start_node, 0, nodes, &mut dist_cache)
}

fn part2_solve(
    start1: &Node,
    start2: &Node,
    minute1: i64,
    minute2: i64,
    nodes: &AHashMap<usize, Node>,
    cache: &mut NodeCache,
) -> i64 {
    let mut pressure = 0;

    if minute1 > 26 || minute2 > 26 {
        return pressure;
    }

    for (_, n1) in nodes {
        for (_, n2) in nodes {
            if n1.flow == 0
                || n2.flow == 0
                || n1.open.get()
                || n2.open.get()
                || n1.id == n2.id
            {
                continue;
            }

            let dist1 = dist_valve(start1.id, n1.id, nodes, cache);
            let dist2 = dist_valve(start2.id, n2.id, nodes, cache);

            n1.open.set(true);
            n2.open.set(true);

            pressure = i64::max(
                pressure,
                (26 - minute1 - dist1 - 1) * n1.flow
                    + (26 - minute2 - dist2 - 1) * n2.flow
                    + part2_solve(
                        n1,
                        n2,
                        minute1 + dist1 + 1,
                        minute2 + dist2 + 1,
                        nodes,
                        cache,
                    ),
            );
            n1.open.set(false);
            n2.open.set(false);
        }
    }

    pressure
}

fn part2(nodes: &AHashMap<usize, Node>) -> i64 {
    let (id, _) = nodes.iter().find(|(_, node)| node.name == "AA").unwrap();
    let start_node = nodes.get(id).unwrap();
    let mut dist_cache = NodeCache::new();

    let nodes = optimize(nodes.clone(), &mut dist_cache);
    part2_solve(start_node, start_node, 0, 0, &nodes, &mut dist_cache)
}

fn main() {
    let input = include_str!("../../input/input16.txt");
    let input = parse(input);
    println!("part1 = {}", part1(&input));
    println!("part2 = {}", part2(&input));
}

#[test]
fn test_day16() {
    let input = "\
Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";
    let input = parse(input);

    assert_eq!(part1(&input), 1651);
    assert_eq!(part2(&input), 1707);
}
