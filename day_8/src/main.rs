use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    str::FromStr,
};

struct ParseErr;

#[derive(Debug)]
struct Entry {
    signals: Vec<HashSet<char>>,
    output: Vec<HashSet<char>>,
}

impl FromStr for Entry {
    type Err = ParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(" | ");
        Ok(Entry {
            signals: split
                .next()
                .ok_or(ParseErr)?
                .split(' ')
                .map(|s| s.chars().collect())
                .collect(),
            output: split
                .next()
                .ok_or(ParseErr)?
                .split(' ')
                .map(|s| s.chars().collect())
                .collect(),
        })
    }
}

fn read_file(filename: impl AsRef<Path>) -> Vec<Entry> {
    let file = File::open(filename).expect("File not found");
    let buf = BufReader::new(file);
    buf.lines()
        .filter_map(|line| line.unwrap().parse().ok())
        .collect()
}

fn main() {
    let input = read_file("data/input.txt");

    // part 1
    let valid_lengths = vec![2, 3, 4, 7];
    println!(
        "Part 1: {} times",
        input
            .iter()
            .map(|e| e
                .output
                .iter()
                .filter(|s| valid_lengths.contains(&s.len()))
                .count())
            .sum::<usize>()
    );

    // part 2
    /*
    standard 7 digit 0-9, letters appear this many times:
        a: 8 *
        b: 6
        c: 8 *
        d: 7 *
        e: 4
        f: 9
        g: 7 *
    so only need to tell a/c and d/g apart
    c appears in 1 (len 2) but a doesn't
    d appears in 4 (len 4) but g doesn't
    */
    let mut real_digits: Vec<HashSet<char>> = Vec::new();
    for s in [
        "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
    ]
    .iter()
    {
        real_digits.push(s.chars().collect());
    }
    let real_digits = real_digits; // doesn't need to be mutable anymore

    let mut output_sum = 0;
    for entry in &input {
        let mut letter_counts = HashMap::new();
        let mut pattern_1 = None;
        let mut pattern_4 = None;

        for pattern in &entry.signals {
            for ch in pattern {
                *letter_counts.entry(*ch).or_insert(0) += 1;
            }
            match pattern.len() {
                2 => pattern_1 = Some(pattern.clone()),
                4 => pattern_4 = Some(pattern.clone()),
                _ => (),
            }
        }

        let mut letter_map = HashMap::new();
        let pattern_1 = pattern_1.unwrap();
        let pattern_4 = pattern_4.unwrap();

        for (ch, count) in &letter_counts {
            letter_map.insert(
                *ch,
                match count {
                    8 => match pattern_1.contains(ch) {
                        // a or c
                        true => 'c',
                        false => 'a',
                    },
                    6 => 'b',
                    7 => match pattern_4.contains(ch) {
                        // d or g
                        true => 'd',
                        false => 'g',
                    },
                    4 => 'e',
                    9 => 'f',
                    _ => unreachable!(),
                },
            );
        }

        output_sum += entry
            .output
            .iter()
            .map(|p| {
                real_digits
                    .iter()
                    .position(|rp| {
                        rp == &p
                            .iter()
                            .map(|c| *letter_map.get(c).unwrap())
                            .collect::<HashSet<char>>()
                    })
                    .unwrap()
            })
            .reduce(|acc, n| acc * 10 + n)
            .unwrap();
    }
    println!("Part 2: sum = {}", output_sum);
}
