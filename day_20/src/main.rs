use std::{
    cmp,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

// (algorithm, image)
fn read_file(filename: impl AsRef<Path>) -> (Vec<bool>, Vec<Vec<bool>>) {
    let file = File::open(filename).expect("File not found");
    let buf = BufReader::new(file);
    let mut lines = buf.lines();
    let algorithm = lines
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .map(|c| c == '#')
        .collect();
    lines.next();
    (
        algorithm,
        lines
            .map(|l| l.unwrap().chars().map(|c| c == '#').collect())
            .collect(),
    )
}

fn expand_image(image: &[Vec<bool>], factor: usize) -> Vec<Vec<bool>> {
    let num_rows = image.len();
    let num_cols = image[0].len();

    let new_num_rows = num_rows * factor;
    let new_num_cols = num_cols * factor;

    let mut new_image = vec![vec![false; new_num_cols]; new_num_rows];

    for (row_index, row) in image.iter().enumerate() {
        for (col_index, pixel) in row.iter().enumerate() {
            new_image[num_rows * (factor / 2) + row_index][num_cols * (factor / 2) + col_index] =
                *pixel;
        }
    }

    new_image
}

fn clamp(num: isize, min: isize, max: usize) -> usize {
    cmp::min(cmp::max(min, num) as usize, max)
}

#[allow(clippy::needless_range_loop)]
fn enhance(algorithm: &[bool], image: &[Vec<bool>]) -> Vec<Vec<bool>> {
    let mut new_image = image.to_owned();
    let num_rows = image.len();
    let num_cols = image[0].len();

    for row_index in 0..num_rows {
        for col_index in 0..num_cols {
            let mut algorithm_index = 0;
            for i in row_index as isize - 1..=row_index as isize + 1 {
                for j in col_index as isize - 1..=col_index as isize + 1 {
                    algorithm_index = (algorithm_index << 1)
                        | (image[clamp(i, 0, num_rows - 1)][clamp(j, 0, num_cols - 1)] as usize);
                }
            }
            new_image[row_index][col_index] = algorithm[algorithm_index];
        }
    }
    new_image.to_owned()
}

fn main() {
    let (algorithm, image) = read_file("data/input.txt");

    // part 1
    let enhanced_image = enhance(&algorithm, &enhance(&algorithm, &expand_image(&image, 3)));
    println!(
        "Part 1: {} pixels are lit",
        enhanced_image
            .iter()
            .map(|r| r.iter().map(|p| *p as u32).sum::<u32>())
            .sum::<u32>()
    );

    // part 2 - run on release mode
    // for example input, factor = 21 is needed
    // for real input, factor = 3 works fine
    let mut enhanced_image = expand_image(&image, 3);
    for _ in 0..50 {
        enhanced_image = enhance(&algorithm, &enhanced_image);
    }
    println!(
        "Part 2: {} pixels are lit",
        enhanced_image
            .iter()
            .map(|r| r.iter().map(|p| *p as u32).sum::<u32>())
            .sum::<u32>()
    );
}
