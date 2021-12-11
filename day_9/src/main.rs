use std::{
    fs::File,
    io::{prelude::*, BufReader},
    num::TryFromIntError,
    path::Path,
};

#[derive(Debug)]
struct ParseErr;

impl From<TryFromIntError> for ParseErr {
    fn from(_: TryFromIntError) -> Self {
        ParseErr
    }
}

#[derive(Debug, Clone, Copy)]
struct Point {
    height: u8,
    visited: bool,
}

impl TryFrom<char> for Point {
    type Error = ParseErr;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(Point {
            height: value.to_digit(10).ok_or(ParseErr)?.try_into()?,
            visited: false,
        })
    }
}

fn read_file(filename: impl AsRef<Path>) -> Vec<Vec<Point>> {
    let file = File::open(filename).expect("File not found");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|s| s.unwrap().chars().map(|c| c.try_into().unwrap()).collect())
        .collect()
}

fn check_low(input: &[Vec<Point>], row_i: usize, col_i: usize) -> bool {
    let height = input[row_i][col_i].height;
    (row_i == 0 || input[row_i - 1][col_i].height > height)
        && (row_i == input.len() - 1 || input[row_i + 1][col_i].height > height)
        && (col_i == 0 || input[row_i][col_i - 1].height > height)
        && (col_i == input[0].len() - 1 || input[row_i][col_i + 1].height > height)
}

fn get_adjacent(input: &[Vec<Point>], row_i: usize, col_i: usize) -> Vec<(usize, usize)> {
    let mut output = Vec::new();
    if row_i > 0 {
        output.push((row_i - 1, col_i));
    }
    if col_i > 0 {
        output.push((row_i, col_i - 1));
    }
    if row_i < input.len() - 1 {
        output.push((row_i + 1, col_i));
    }
    if col_i < input[0].len() - 1 {
        output.push((row_i, col_i + 1));
    }
    output
}

fn get_basin_size(input: &mut [Vec<Point>], row_i: usize, col_i: usize) -> u32 {
    if input[row_i][col_i].height == 9 {
        return 0;
    }
    let mut basin_size = 0;
    for (adj_row_i, adj_col_i) in &get_adjacent(input, row_i, col_i) {
        let point = &mut input[*adj_row_i][*adj_col_i];
        if !point.visited && point.height < 9 {
            point.visited = true;
            basin_size += get_basin_size(input, *adj_row_i, *adj_col_i) + 1;
        }
    }
    basin_size
}

fn main() {
    let input = read_file("data/input.txt");

    // part 1
    let mut risk_sum = 0;
    for (row_i, row) in input.iter().enumerate() {
        for (col_i, point) in row.iter().enumerate() {
            if check_low(&input, row_i, col_i) {
                risk_sum += point.height as u32 + 1;
            }
        }
    }
    println!("Part 1: risk_sum = {}", risk_sum);

    // part 2
    let mut input = input;
    let mut basins = Vec::new();
    for row_i in 0..input.len() {
        for col_i in 0..input[row_i].len() {
            let point = input[row_i][col_i];
            if point.height != 9 && !point.visited {
                basins.push(get_basin_size(&mut input, row_i, col_i));
            }
        }
    }
    basins.sort_unstable();
    println!(
        "Part 2: product = {}",
        basins.iter().rev().take(3).product::<u32>()
    );
}
