use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

// I could use a struct and/or enum here, but honestly I don't care
fn read_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("File not found");
    let buf = BufReader::new(file);
    buf.lines().map(|l| l.unwrap()).collect()
}

fn get_illegal_score(c: char) -> u64 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("unexpected character '{}'", c),
    }
}

fn get_ac_points(c: char) -> u64 {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => panic!("unexpected character '{}'", c),
    }
}

fn get_closing(c: char) -> Option<char> {
    match c {
        '(' => Some(')'),
        '[' => Some(']'),
        '{' => Some('}'),
        '<' => Some('>'),
        _ => None,
    }
}

fn is_opening(c: char) -> bool {
    get_closing(c).is_some()
}

fn main() {
    let mut input = read_file("data/input.txt");

    // part 1
    let mut stack = Vec::new();
    let mut error_score = 0;
    input.retain(|line| {
        stack.clear();
        for c in line.chars() {
            if is_opening(c) {
                stack.push(c);
            } else if let Some(pop_c) = stack.pop() {
                if get_closing(pop_c).unwrap() != c {
                    error_score += get_illegal_score(c);
                    return false;
                }
            }
        }
        true
    });
    println!("Part 1: error_score = {} points", error_score);

    // part 2
    let mut ac_scores = Vec::new();
    for line in &input {
        stack.clear();
        for c in line.chars() {
            if is_opening(c) {
                stack.push(c);
            } else {
                stack.pop();
            }
        }
        let mut line_score = 0;
        for c in stack.iter().rev() {
            line_score = line_score * 5 + get_ac_points(get_closing(*c).unwrap());
        }
        ac_scores.push(line_score);
    }
    ac_scores.sort_unstable();
    println!("Part 2: middle_score = {}", ac_scores[ac_scores.len() / 2]);
}
