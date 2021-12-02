use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn read_file(filename: impl AsRef<Path>) -> Vec<u64> {
    let file = File::open(filename).expect("File not found");
    let buf = BufReader::new(file);
    buf.lines()
        .filter_map(|line| line.expect("Failed to read line").parse().ok())
        .collect()
}

fn main() {
    let input = read_file("data/input.txt");

    let mut num_increased = 0;
    for (a, b) in input.iter().zip(input.iter().skip(1)) {
        if b > a {
            num_increased += 1;
        }
    }
    println!(
        "Part 1: There are {} measurements that are larger than the previous measurement.",
        num_increased
    );

    let mut windows = Vec::new();
    for (a, (b, c)) in input
        .iter()
        .zip(input.iter().skip(1).zip(input.iter().skip(2)))
    {
        windows.push(a + b + c);
    }
    num_increased = 0;
    for (a, b) in windows.iter().zip(windows.iter().skip(1)) {
        if b > a {
            num_increased += 1;
        }
    }
    println!(
        "Part 2: There are {} windows that are larger than the previous window.",
        num_increased
    );
}
