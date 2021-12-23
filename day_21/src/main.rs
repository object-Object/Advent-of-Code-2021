use std::{
    collections::HashMap,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn read_file(filename: impl AsRef<Path>) -> (u64, u64) {
    let file = File::open(filename).expect("File not found");
    let buf = BufReader::new(file);
    let mut lines = buf.lines();
    (
        lines
            .next()
            .unwrap()
            .unwrap()
            .strip_prefix("Player 1 starting position: ")
            .unwrap()
            .parse()
            .unwrap(),
        lines
            .next()
            .unwrap()
            .unwrap()
            .strip_prefix("Player 2 starting position: ")
            .unwrap()
            .parse()
            .unwrap(),
    )
}

fn deterministic_roll(last_roll: &mut u64) -> u64 {
    let mut sum = 0;
    for _ in 0..3 {
        *last_roll = *last_roll % 100 + 1;
        sum += *last_roll;
    }
    sum
}

fn find_pos(pos: u64, sum: u64) -> u64 {
    (pos + sum - 1) % 10 + 1
}

fn deterministic_move(pos: &mut u64, roll: &mut u64, score: &mut u64) -> bool {
    *pos = find_pos(*pos, deterministic_roll(roll));
    *score += *pos;
    *score >= 1000
}

fn add_pairs(lhs: (u64, u64), rhs: (u64, u64)) -> (u64, u64) {
    (lhs.0 + rhs.0, lhs.1 + rhs.1)
}

#[allow(clippy::type_complexity)]
fn quantum_play(
    player1_pos: u64,
    player2_pos: u64,
    player1_score: u64,
    player2_score: u64,
    turn: bool, // true if player 1, false if player 2
    data: &mut HashMap<(u64, u64, u64, u64, bool), (u64, u64)>,
) -> (u64, u64) {
    if let Some(wins) = data.get(&(player1_pos, player2_pos, player1_score, player2_score, turn)) {
        return *wins;
    } else if player1_score >= 21 {
        return (1, 0);
    } else if player2_score >= 21 {
        return (0, 1);
    }

    let mut wins = (0, 0);
    for roll1 in 1..=3 {
        for roll2 in 1..=3 {
            for roll3 in 1..=3 {
                wins = add_pairs(
                    wins,
                    if turn {
                        let player1_pos = find_pos(player1_pos, roll1 + roll2 + roll3);
                        quantum_play(
                            player1_pos,
                            player2_pos,
                            player1_score + player1_pos,
                            player2_score,
                            !turn,
                            data,
                        )
                    } else {
                        let player2_pos = find_pos(player2_pos, roll1 + roll2 + roll3);
                        quantum_play(
                            player1_pos,
                            player2_pos,
                            player1_score,
                            player2_score + player2_pos,
                            !turn,
                            data,
                        )
                    },
                );
            }
        }
    }
    data.insert(
        (player1_pos, player2_pos, player1_score, player2_score, turn),
        wins,
    );
    wins
}

fn main() {
    let (player1_pos, player2_pos) = read_file("data/input.txt");

    // part 1
    {
        let mut roll_count = 0;
        let mut roll = 0;
        let mut player1_pos = player1_pos;
        let mut player2_pos = player2_pos;
        let mut player1_score = 0;
        let mut player2_score = 0;
        let losing_score = loop {
            roll_count += 3;
            if deterministic_move(&mut player1_pos, &mut roll, &mut player1_score) {
                break player2_score;
            }
            roll_count += 3;
            if deterministic_move(&mut player2_pos, &mut roll, &mut player2_score) {
                break player1_score;
            }
        };
        println!(
            "Part 1: losing score = {}, roll count = {}, product = {}",
            losing_score,
            roll_count,
            losing_score * roll_count
        );
    }

    // part 2
    let wins = quantum_play(player1_pos, player2_pos, 0, 0, true, &mut HashMap::new());
    println!(
        "Part 2: player 1 wins in {} universes, player 2 wins in {} universes, {} wins more",
        wins.0,
        wins.1,
        if wins.0 > wins.1 {
            "player 1"
        } else {
            "player 2"
        }
    );
}
