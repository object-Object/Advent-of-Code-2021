use std::{
    cmp::{max, min, Ordering},
    fs::File,
    io::{prelude::*, BufReader},
    num::ParseIntError,
    path::Path,
    str::FromStr,
};

struct ParseErr;

impl From<ParseIntError> for ParseErr {
    fn from(_: ParseIntError) -> Self {
        ParseErr
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: usize,
    y: usize,
}

impl FromStr for Point {
    type Err = ParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(',');
        Ok(Point {
            x: split.next().ok_or(ParseErr)?.parse()?,
            y: split.next().ok_or(ParseErr)?.parse()?,
        })
    }
}

#[derive(Debug, Clone, Copy)]
struct Line {
    a: Point,
    b: Point,
}

impl FromStr for Line {
    type Err = ParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(" -> ");
        let a = split.next().ok_or(ParseErr)?.parse()?;
        let b = split.next().ok_or(ParseErr)?.parse()?;
        Ok(Line {
            a: min(a, b),
            b: max(a, b),
        })
    }
}

fn read_file(filename: impl AsRef<Path>) -> Vec<Line> {
    let file = File::open(filename).expect("File not found");
    let buf = BufReader::new(file);
    buf.lines()
        .filter_map(|line| line.expect("Failed to read line").parse().ok())
        .collect()
}

fn birange(a: usize, b: usize) -> Vec<usize> {
    if a < b {
        (a..=b).collect()
    } else {
        (b..=a).rev().collect()
    }
}

fn get_overlaps(input: &[Line]) -> usize {
    let points: Vec<Point> = input.iter().map(|l| [l.a, l.b]).flatten().collect();

    let mut bottom_right = points[0];
    for point in &points {
        bottom_right.x = max(point.x, bottom_right.x);
        bottom_right.y = max(point.y, bottom_right.y);
    }

    let mut grid = vec![ // grid[x][y]
        vec![0; bottom_right.y + 1];
        bottom_right.x + 1
    ];

    for line in input {
        let mut all_x = birange(line.a.x, line.b.x);
        let mut all_y = birange(line.a.y, line.b.y);
        match all_x.len().cmp(&all_y.len()) {
            Ordering::Less => all_x = vec![line.a.x; all_y.len()],
            Ordering::Greater => all_y = vec![line.a.y; all_x.len()],
            _ => (),
        }
        for (x, y) in all_x.iter().zip(&all_y) {
            grid[*x][*y] += 1;
        }
    }

    grid.iter().flatten().filter(|&n| *n >= 2).count()
}

fn main() {
    let input = read_file("data/input.txt");

    // part 1
    {
        let mut input = input.clone();
        input.retain(|l| l.a.x == l.b.x || l.a.y == l.b.y); // only keep horizontal/vertical
        println!("Part 1: {} overlaps", get_overlaps(&input));
    }

    // part 2
    println!("Part 2: {} overlaps", get_overlaps(&input));
}
