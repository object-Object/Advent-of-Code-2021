// this one's a mess :)
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
struct Octopus {
    energy: u8,
    has_flashed: bool,
}

impl TryFrom<char> for Octopus {
    type Error = ParseErr;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(Octopus {
            energy: value.to_digit(10).ok_or(ParseErr)?.try_into()?,
            has_flashed: false,
        })
    }
}

fn read_file(filename: impl AsRef<Path>) -> Vec<Vec<Octopus>> {
    let file = File::open(filename).expect("File not found");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|s| s.unwrap().chars().map(|c| c.try_into().unwrap()).collect())
        .collect()
}

fn get_adjacent(input: &[Vec<Octopus>], row_i: usize, col_i: usize) -> Vec<(usize, usize)> {
    let mut output = Vec::new();
    for i in -1..=1 {
        for j in -1..=1 {
            let adj_row_i = row_i as isize + i;
            let adj_col_i = col_i as isize + j;
            if (0..input.len() as isize).contains(&adj_row_i)
                && (0..input[adj_row_i as usize].len() as isize).contains(&adj_col_i)
            {
                output.push((adj_row_i as usize, adj_col_i as usize))
            }
        }
    }
    output
}

fn try_flash(input: &mut [Vec<Octopus>], row_i: usize, col_i: usize) -> u32 {
    let octopus = &mut input[row_i][col_i];
    if octopus.has_flashed || octopus.energy <= 9 {
        return 0;
    }
    octopus.has_flashed = true;
    let mut flashes = 1;
    for (adj_row_i, adj_col_i) in get_adjacent(input, row_i, col_i) {
        input[adj_row_i][adj_col_i].energy += 1;
        flashes += try_flash(input, adj_row_i, adj_col_i);
    }
    flashes
}

fn main() {
    let mut input = read_file("data/input.txt");

    let mut total_flashes = 0;
    let mut has_synced = false;
    let mut step = 1;
    loop {
        for octopus in input.iter_mut().flatten() {
            octopus.energy += 1;
        }
        for row_i in 0..input.len() {
            for col_i in 0..input[row_i].len() {
                total_flashes += try_flash(&mut input, row_i, col_i);
            }
        }
        if step == 100 {
            println!("Part 1: {} flashes", total_flashes);
            if has_synced {
                break;
            }
        }
        let mut is_synced = true;
        for octopus in input.iter_mut().flatten() {
            if octopus.has_flashed {
                octopus.has_flashed = false;
                octopus.energy = 0;
            } else {
                is_synced = false;
            }
        }
        if is_synced && !has_synced {
            println!("Part 2: synced on step {}", step);
            if step >= 100 {
                break;
            }
            has_synced = true;
        }
        step += 1;
    }
}
