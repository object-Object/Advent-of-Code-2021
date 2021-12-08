use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn read_file(filename: impl AsRef<Path>) -> Vec<u8> {
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

fn main() {
    let input = read_file("data/input.txt");

    // part 1
    {
        let mut input = input.clone();
        for _ in 1..=80 {
            let mut num_create = 0;
            input = input
                .iter()
                .map(|fish| match fish {
                    0 => {
                        num_create += 1;
                        6
                    }
                    _ => fish - 1,
                })
                .collect();
            input.append(&mut vec![8_u8; num_create]);
        }
        println!("Part 1: {} fish after 80 days", input.len());
    }

    // part 2
    /*
    9 buckets [0-8]
    each day:
        - move counts in buckets [1-8] left
        - move count in bucket 0 to bucket 8 and also add to bucket 6
    */
    let mut buckets = [0_u64; 9];
    for fish in &input {
        buckets[*fish as usize] += 1;
    }
    for _ in 1..=256 {
        let zero_count = buckets[0];
        for index in 1..9 {
            buckets[index - 1] = buckets[index];
        }
        buckets[8] = zero_count;
        buckets[6] += zero_count;
    }
    println!("Part 2: {} fish after 80 days", buckets.iter().sum::<u64>());
}
