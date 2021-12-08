use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn read_file(filename: impl AsRef<Path>) -> Vec<i32> {
    let file = File::open(filename).expect("File not found");
    let buf = BufReader::new(file);
    buf.lines()
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect()
}

fn get_min_fuel<F>(input: &[i32], f: F) -> i32
where
    F: Fn(i32, i32) -> i32,
{
    let min = *input.iter().min().unwrap();
    let max = *input.iter().max().unwrap();
    let mut min_fuel: Option<i32> = None;

    for position in min..=max {
        let mut fuel = 0;
        for crab in input {
            fuel += f(position, *crab);
        }
        min_fuel = match min_fuel {
            Some(n) => Some(n.min(fuel)),
            None => Some(fuel),
        };
    }

    min_fuel.unwrap()
}

fn main() {
    let input = read_file("data/input.txt");

    // part 1
    println!(
        "Part 1: {} fuel",
        get_min_fuel(&input, |a, b| (a - b).abs())
    );

    // part 2
    println!(
        "Part 1: {} fuel",
        get_min_fuel(&input, |a, b| {
            let dist = (a - b).abs();
            dist * (dist + 1) / 2
        })
    );
}
