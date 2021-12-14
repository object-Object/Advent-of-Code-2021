use std::{
    collections::HashMap,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

type Rules = HashMap<[char; 2], char>;

fn read_file(filename: impl AsRef<Path>) -> (Vec<char>, Rules) {
    let file = File::open(filename).expect("File not found");
    let mut lines = BufReader::new(file).lines();
    let template = lines.next().unwrap().unwrap().chars().collect();
    lines.next();
    (
        template,
        lines
            .map(|line| {
                let chars = line.unwrap().chars().collect::<Vec<_>>();
                ([chars[0], chars[1]], chars[6])
            })
            .collect(),
    )
}

fn main() {
    let (template, rules) = read_file("data/input.txt");

    // part 1
    {
        let mut polymer = template.clone();
        for _ in 0..10 {
            for i in (0..polymer.len() - 1).rev() {
                if let Some(to_insert) = rules.get(&[polymer[i], polymer[i + 1]]) {
                    polymer.insert(i + 1, *to_insert);
                }
            }
        }
        let mut counts = HashMap::new();
        for c in &polymer {
            *counts.entry(*c).or_insert(0) += 1;
        }
        println!(
            "Part 1: result = {}",
            counts.values().max().unwrap() - counts.values().min().unwrap()
        );
    }

    // part 2
    let mut pair_counts = rules
        .iter()
        .map(|(k, _)| (*k, 0_u64))
        .collect::<HashMap<_, _>>();

    let transforms = rules
        .iter()
        .map(|(k, v)| (*k, [[k[0], *v], [*v, k[1]]]))
        .collect::<HashMap<_, _>>();

    for i in 0..template.len() - 1 {
        pair_counts
            .entry([template[i], template[i + 1]])
            .and_modify(|e| *e += 1);
    }

    for _ in 0..40 {
        let mut to_update = Vec::new();
        for (pair, count) in pair_counts.iter_mut() {
            if *count > 0 {
                if let Some(transform) = transforms.get(pair) {
                    for pair in transform {
                        to_update.push((*pair, *count));
                    }
                }
                *count = 0;
            }
        }
        for (pair, count) in &to_update {
            pair_counts.entry(*pair).and_modify(|e| *e += *count);
        }
    }

    let mut counts = HashMap::new();
    for (pair, count) in &pair_counts {
        for c in pair {
            *counts.entry(c).or_insert(0) += count;
        }
    }
    // i think dividing by 2 is needed because every value appears twice due to overlap
    println!(
        "Part 2: result = {}",
        ((counts.values().max().unwrap() - counts.values().min().unwrap()) as f64 / 2_f64).ceil()
    );
}
