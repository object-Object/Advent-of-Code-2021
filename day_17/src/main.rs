use std::{
    fs::File,
    io::{prelude::*, BufReader},
    num::ParseIntError,
    path::Path,
    str::FromStr,
};

#[derive(Debug)]
struct ParseErr;

impl From<ParseIntError> for ParseErr {
    fn from(_: ParseIntError) -> Self {
        ParseErr
    }
}

#[derive(Debug)]
struct Target {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
}

impl FromStr for Target {
    type Err = ParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pairs = s
            .strip_prefix("target area: x=")
            .ok_or(ParseErr)?
            .split(", y=")
            .map(|s1| s1.split("..").map(|s2| s2.parse()).collect())
            .collect::<Result<Vec<Vec<i32>>, ParseIntError>>()?;
        Ok(Target {
            x_min: pairs[0][0],
            x_max: pairs[0][1],
            y_min: pairs[1][0],
            y_max: pairs[1][1],
        })
    }
}

fn read_file(filename: impl AsRef<Path>) -> Target {
    let file = File::open(filename).expect("File not found");
    let buf = BufReader::new(file);
    buf.lines().next().unwrap().unwrap().parse().unwrap()
}

// assuming target.y_min is always less than zero
// returns (hit_target, max_y)
fn fire(target: &Target, mut x_vel: i32, mut y_vel: i32) -> (bool, i32) {
    let mut x = 0_i32;
    let mut y = 0_i32;
    let mut max_y = y;
    while x.abs() <= target.x_min.abs().max(target.x_max.abs()) && y >= target.y_min {
        x += x_vel;
        y += y_vel;
        max_y = max_y.max(y);
        x_vel -= x_vel.signum();
        y_vel -= 1;
        if x >= target.x_min && x <= target.x_max && y >= target.y_min && y <= target.y_max {
            return (true, max_y);
        }
    }
    (false, max_y)
}

// returns (max_max_y, num_velocities)
fn barrage(target: &Target) -> (i32, u32) {
    let mut max_max_y = 0;
    let mut num_velocities = 0;
    // these could probably be more conservative but whatever
    for x_vel in 0.min(target.x_min)..=0.max(target.x_max) {
        for y_vel in -10 * target.y_min.abs()..10 * target.y_min.abs() {
            let (hit_target, max_y) = fire(target, x_vel, y_vel);
            if hit_target {
                max_max_y = max_max_y.max(max_y);
                num_velocities += 1;
            }
        }
    }
    (max_max_y, num_velocities)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_velocity_7_2() {
        let target: Target = "target area: x=20..30, y=-10..-5".parse().unwrap();
        assert!(fire(&target, 7, 2).0);
    }

    #[test]
    fn check_velocity_6_3() {
        let target: Target = "target area: x=20..30, y=-10..-5".parse().unwrap();
        assert!(fire(&target, 6, 3).0);
    }

    #[test]
    fn check_velocity_9_0() {
        let target: Target = "target area: x=20..30, y=-10..-5".parse().unwrap();
        assert!(fire(&target, 9, 0).0);
    }

    #[test]
    fn check_velocity_17_n4() {
        let target: Target = "target area: x=20..30, y=-10..-5".parse().unwrap();
        assert!(!fire(&target, 17, -4).0);
    }

    #[test]
    fn test_barrage() {
        let target: Target = "target area: x=20..30, y=-10..-5".parse().unwrap();
        let (max_max_y, num_velocities) = barrage(&target);
        assert_eq!(max_max_y, 45);
        assert_eq!(num_velocities, 112);
    }
}

fn main() {
    let target: Target = read_file("data/input.txt");

    let (max_max_y, num_velocities) = barrage(&target);
    println!(
        "Part 1: max_max_y = {}\nPart 2: num_velocities = {}",
        max_max_y, num_velocities
    );
}
