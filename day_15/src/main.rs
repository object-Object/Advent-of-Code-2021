use priority_queue::PriorityQueue;
use std::{
    cmp::Reverse,
    collections::HashMap,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn read_file(filename: impl AsRef<Path>) -> Vec<Vec<u32>> {
    let file = File::open(filename).expect("File not found");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|s| {
            s.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect()
        })
        .collect()
}

fn adjacent(pos: (usize, usize), map_size: usize) -> Vec<(usize, usize)> {
    let mut output = Vec::new();
    if pos.0 > 0 {
        output.push((pos.0 - 1, pos.1));
    }
    if pos.1 > 0 {
        output.push((pos.0, pos.1 - 1));
    }
    if pos.0 < map_size - 1 {
        output.push((pos.0 + 1, pos.1));
    }
    if pos.1 < map_size - 1 {
        output.push((pos.0, pos.1 + 1));
    }
    output
}

fn dijkstra(map: &[Vec<u32>]) -> u32 {
    let map_size = map.len();
    let mut dist = HashMap::new();
    let mut queue = PriorityQueue::new();

    dist.insert((0, 0), 0);
    for row in 0..map_size {
        for col in 0..map_size {
            let dist = *dist.entry((row, col)).or_insert(u32::MAX);
            queue.push((row, col), Reverse(dist));
        }
    }

    while let Some((best_pos, best_dist)) = queue.pop() {
        for adj_pos in adjacent(best_pos, map_size) {
            if let Some((_, adj_dist)) = queue.get(&adj_pos) {
                let alt_adj_dist = best_dist.0 + map[adj_pos.0][adj_pos.1];
                if alt_adj_dist < adj_dist.0 {
                    dist.insert(adj_pos, alt_adj_dist);
                    queue.change_priority(&adj_pos, Reverse(alt_adj_dist));
                }
            }
        }
    }

    dist[&(map_size - 1, map_size - 1)]
}

fn main() {
    let map = read_file("data/input.txt");

    // part 1
    println!("Part 1: lowest risk = {:?}", dijkstra(&map));

    // part 2
    let original_size = map.len(); // map is a square
    let mut expanded_map = vec![vec![0; original_size * 5]; original_size * 5];
    for row in 0..original_size * 5 {
        for col in 0..original_size * 5 {
            expanded_map[row][col] = (map[row % original_size][col % original_size]
                + (row / original_size) as u32
                + (col / original_size) as u32
                - 1)
                % 9
                + 1;
        }
    }
    println!("Part 2: lowest risk = {:?}", dijkstra(&expanded_map));
}
