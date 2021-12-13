use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
enum NodeKind {
    Start,
    End,
    Small(String),
    Large(String),
}

fn read_file(filename: impl AsRef<Path>) -> (HashSet<NodeKind>, HashMap<NodeKind, Vec<NodeKind>>) {
    let file = File::open(filename).expect("File not found");
    let mut lines = BufReader::new(file).lines();
    let mut nodes = HashSet::new();
    let mut edges = HashMap::new();
    while let Some(Ok(line)) = lines.next() {
        let line_nodes = line
            .split('-')
            .map(|s| match s {
                "start" => NodeKind::Start,
                "end" => NodeKind::End,
                _ => match s.chars().all(char::is_lowercase) {
                    true => NodeKind::Small(s.to_string()),
                    false => NodeKind::Large(s.to_string()),
                },
            })
            .collect::<Vec<_>>();
        nodes.insert(line_nodes[0].clone());
        nodes.insert(line_nodes[1].clone());
        edges
            .entry(line_nodes[0].clone())
            .or_insert_with(Vec::new)
            .push(line_nodes[1].clone());
        edges
            .entry(line_nodes[1].clone())
            .or_insert_with(Vec::new)
            .push(line_nodes[0].clone());
    }
    (nodes, edges)
}

fn traverse(
    nodes: &HashSet<NodeKind>,
    edges: &HashMap<NodeKind, Vec<NodeKind>>,
    current: &NodeKind,
    visited_small: HashSet<NodeKind>,
    has_double_visited: bool,
) -> u32 {
    let mut paths = 0;
    if let Some(adjacent_nodes) = edges.get(current) {
        for node in adjacent_nodes {
            let mut has_double_visited = has_double_visited;
            if visited_small.contains(node) {
                if has_double_visited {
                    continue;
                } else {
                    has_double_visited = true;
                }
            }
            paths += match node {
                NodeKind::Start => 0,
                NodeKind::End => 1,
                _ => {
                    let mut visited_small = visited_small.clone();
                    if let NodeKind::Small(_) = node {
                        visited_small.insert(node.clone());
                    }
                    traverse(nodes, edges, node, visited_small, has_double_visited)
                }
            };
        }
    }
    paths
}

fn solve(filename: &str) {
    println!("{}:", filename);
    let (nodes, edges) = read_file(filename);

    // part 1
    println!(
        "\tPart 1: {} paths",
        traverse(&nodes, &edges, &NodeKind::Start, HashSet::new(), true)
    );

    // part 2
    println!(
        "\tPart 2: {} paths",
        traverse(&nodes, &edges, &NodeKind::Start, HashSet::new(), false)
    );
}

fn main() {
    solve("data/example1.txt");
    solve("data/example2.txt");
    solve("data/example3.txt");
    solve("data/input.txt");
}
