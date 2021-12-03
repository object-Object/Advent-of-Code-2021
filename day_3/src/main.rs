use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn read_file(filename: impl AsRef<Path>) -> Vec<Vec<u8>> {
    let file = File::open(filename).expect("File not found");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|line| {
            line.expect("Failed to read line")
                .chars()
                .filter_map(|c| c.to_digit(2).map(|u| u as u8))
                .collect()
        })
        .collect()
}

fn filter<F>(input: &Vec<Vec<u8>>, f: F) -> Option<u32>
where
    F: Fn(u32, u32) -> u8, // take number of zeros and number of ones, return 0 or 1
{
    let mut working = input.clone();
    let len = working[0].len();
    let mut counts = vec![0, 0];
    for bit_num in 0..len {
        for line in &working {
            counts[line[bit_num] as usize] += 1;
        }
        let keep_bit = f(counts[0], counts[1]);
        working = working
            .iter()
            .filter(|l| l[bit_num] == keep_bit)
            .cloned()
            .collect();
        if working.len() == 1 {
            return Some(working[0].iter().fold(0, |acc, &b| (acc << 1) | b as u32));
        }
        counts.clear();
        counts.resize(2, 0);
    }
    None
}

fn main() {
    let input = read_file("data/input.txt");

    // part 1
    let mut counts = vec![[0, 0]; input[0].len()];
    for line in &input {
        for (i, bit) in line.iter().enumerate() {
            counts[i][*bit as usize] += 1;
        }
    }

    let mut gamma: u32 = 0;
    let mut epsilon: u32 = 0;
    for count in &counts {
        let (most_common, least_common) = if count[0] <= count[1] {
            (1, 0) // 1 is most common, 0 is least
        } else {
            (0, 1)
        };
        gamma = (gamma << 1) | most_common;
        epsilon = (epsilon << 1) | least_common;
    }
    println!(
        "Part 1: gamma = {}, epsilon = {}, product = {}",
        gamma,
        epsilon,
        gamma * epsilon
    );

    // part 2
    let o2_rating = filter(&input, |num_0, num_1| if num_1 >= num_0 { 1 } else { 0 })
        .expect("Failed to find a value for o2");
    let co2_rating = filter(&input, |num_0, num_1| if num_0 <= num_1 { 0 } else { 1 })
        .expect("Failed to find a value for co2");
    println!(
        "Part 2: o2_rating = {}, co2_rating = {}, product = {}",
        o2_rating,
        co2_rating,
        o2_rating * co2_rating
    );
}
