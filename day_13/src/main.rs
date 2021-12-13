#![feature(int_abs_diff)]
use std::{
    collections::HashSet,
    fs::File,
    io::{prelude::*, BufReader},
    num::ParseIntError,
    path::Path,
    str::FromStr,
};

#[derive(Debug, Clone, Copy)]
enum FoldKind {
    X(u16),
    Y(u16),
}

#[derive(Debug)]
struct ParseErr;

impl From<ParseIntError> for ParseErr {
    fn from(_: ParseIntError) -> Self {
        ParseErr
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Dot {
    x: u16,
    y: u16,
}

impl FromStr for Dot {
    type Err = ParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(',');
        Ok(Dot {
            x: split.next().ok_or(ParseErr)?.parse()?,
            y: split.next().ok_or(ParseErr)?.parse()?,
        })
    }
}

fn read_file(filename: impl AsRef<Path>) -> (HashSet<Dot>, Vec<FoldKind>) {
    let file = File::open(filename).expect("File not found");
    let mut lines = BufReader::new(file).lines();

    let mut dots = HashSet::new();
    let mut folds = Vec::new();
    let mut reading_dots = true;
    while let Some(Ok(line)) = lines.next() {
        if line.is_empty() {
            reading_dots = false;
        } else if reading_dots {
            dots.insert(line.parse().unwrap());
        } else {
            let line = line
                .strip_prefix("fold along ")
                .unwrap()
                .split('=')
                .collect::<Vec<_>>();
            folds.push(match line[0] {
                "x" => FoldKind::X(line[1].parse().unwrap()),
                "y" => FoldKind::Y(line[1].parse().unwrap()),
                _ => unreachable!(),
            });
        }
    }
    (dots, folds)
}

fn do_fold(dots: &mut HashSet<Dot>, fold: FoldKind) {
    let (fold_x, fold_y) = match fold {
        FoldKind::X(x) => (x, u16::MAX),
        FoldKind::Y(y) => (u16::MAX, y),
    };
    let mut to_add = Vec::new();
    dots.retain(|p| {
        if p.x > fold_x {
            to_add.push(Dot {
                x: fold_x.abs_diff(p.x - fold_x),
                y: p.y,
            });
            false
        } else if p.y > fold_y {
            to_add.push(Dot {
                x: p.x,
                y: fold_y.abs_diff(p.y - fold_y),
            });
            false
        } else {
            p.x != fold_x && p.y != fold_y
        }
    });
    dots.extend(to_add);
}

fn main() {
    let (mut dots, folds) = read_file("data/input.txt");

    // part 1
    do_fold(&mut dots, folds[0]);
    println!("Part 1: {} dots", dots.len());

    // part 2
    for fold in folds.iter().skip(1) {
        do_fold(&mut dots, *fold);
    }
    let mut max_x = 0;
    let mut max_y = 0;
    for dot in &dots {
        max_x = max_x.max(dot.x);
        max_y = max_y.max(dot.y);
    }
    
    let mut output = vec![vec![false; max_x as usize + 1]; max_y as usize + 1]; // output[y][x]
    for dot in &dots {
        output[dot.y as usize][dot.x as usize] = true;
    }
    println!("Part 2:");
    for row in output.iter() {
        for col in row {
            print!(
                "{}",
                match col {
                    true => "##",
                    false => "  ",
                }
            );
        }
        println!();
    }
}
