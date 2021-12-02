use std::{
    fs::File,
    io::{prelude::*, BufReader},
    num::ParseIntError,
    path::Path,
    str::FromStr,
};

struct ParseInstructionError;

impl From<ParseIntError> for ParseInstructionError {
    fn from(_: ParseIntError) -> Self {
        ParseInstructionError
    }
}

#[derive(Debug)]
struct Instruction {
    vertical_offset: i64,
    forward_offset: i64,
}

impl FromStr for Instruction {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_whitespace();
        let command = iter.next().unwrap_or("");
        let distance: i64 = iter.next().unwrap_or("").parse()?;
        match command {
            "forward" => Ok(Self {
                vertical_offset: 0,
                forward_offset: distance,
            }),
            "down" => Ok(Self {
                vertical_offset: distance,
                forward_offset: 0,
            }),
            "up" => Ok(Self {
                vertical_offset: -distance,
                forward_offset: 0,
            }),
            _ => Err(ParseInstructionError),
        }
    }
}

fn read_file(filename: impl AsRef<Path>) -> Vec<Instruction> {
    let file = File::open(filename).expect("File not found");
    let buf = BufReader::new(file);
    buf.lines()
        .filter_map(|line| line.expect("Failed to read line").parse().ok())
        .collect()
}

fn main() {
    let instructions = read_file("data/input.txt");

    // part 1
    let mut horizontal_pos = 0;
    let mut depth = 0;
    for instruction in &instructions {
        horizontal_pos += instruction.forward_offset;
        depth += instruction.vertical_offset;
    }
    println!(
        "Part 1:\nHorizontal position: {}\nDepth: {}\nProduct: {}\n",
        horizontal_pos,
        depth,
        horizontal_pos * depth
    );

    // part 2
    horizontal_pos = 0;
    depth = 0;
    let mut aim = 0;
    for instruction in &instructions {
        aim += instruction.vertical_offset;
        horizontal_pos += instruction.forward_offset;
        depth += instruction.forward_offset * aim;
    }
    println!(
        "Part 2:\nHorizontal position: {}\nDepth: {}\nProduct: {}",
        horizontal_pos,
        depth,
        horizontal_pos * depth
    );
}
