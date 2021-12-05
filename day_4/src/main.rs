use std::{
    collections::HashMap,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

#[derive(Clone, Copy, Debug)]
struct Number {
    value: u8,
    marked: bool,
}

type Board = [[Number; 5]; 5];

// this should probably be more closely related to Board, but idc, this is AoC
#[derive(Debug)]
struct BoardIndex {
    board_index: usize,
    row_index: usize,
    col_index: usize,
}

impl BoardIndex {
    fn get_ref_mut<'a>(&self, boards: &'a mut [Board]) -> &'a mut Number {
        &mut boards[self.board_index][self.row_index][self.col_index]
    }

    fn won(&self, boards: &[Board]) -> bool {
        boards[self.board_index][self.row_index]
            .iter()
            .all(|n| n.marked)
            || boards[self.board_index]
                .iter()
                .all(|r| r[self.col_index].marked)
    }
}

fn read_file(filename: impl AsRef<Path>) -> (Vec<u8>, Vec<Board>, HashMap<u8, Vec<BoardIndex>>) {
    let file = File::open(filename).expect("File not found");
    let buf = BufReader::new(file);
    let mut iter = buf.lines();

    let draws = iter
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|n| n.parse::<u8>().unwrap())
        .collect();

    let mut boards = Vec::new();
    let mut map = HashMap::new();
    while let Some(_) = iter.next() {
        let mut board = [[Number {
            value: 0,
            marked: false,
        }; 5]; 5];
        for (row_index, row) in board.iter_mut().enumerate() {
            for (col_index, value) in iter
                .next()
                .unwrap()
                .unwrap()
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .enumerate()
            {
                row[col_index].value = value;
                map.entry(value).or_insert_with(Vec::new).push(BoardIndex {
                    board_index: boards.len(),
                    row_index,
                    col_index,
                });
            }
        }
        boards.push(board);
    }

    (draws, boards, map)
}

fn main() {
    let (draws, boards, map) = read_file("data/input.txt");

    // part 1
    let mut boards_p1 = boards.clone();
    let mut winning_board_op = None;
    let mut winning_draw_op = None;
    'outer: for draw in &draws {
        if let Some(board_indexes) = map.get(draw) {
            for board_index in board_indexes {
                board_index.get_ref_mut(&mut boards_p1).marked = true;
                if board_index.won(&boards_p1) {
                    winning_board_op = Some(boards_p1[board_index.board_index]);
                    winning_draw_op = Some(*draw);
                    break 'outer;
                }
            }
        }
    }
    if let Some(winning_board) = winning_board_op {
        let unmarked_sum = winning_board
            .iter()
            .map(|r| {
                r.iter()
                    .filter_map(|n| match n.marked {
                        true => None,
                        false => Some(n.value as u32),
                    })
                    .sum::<u32>()
            })
            .sum::<u32>();
        let winning_draw = winning_draw_op.unwrap();
        println!(
            "Part 1: unmarked_sum = {}, winning_draw = {}, product = {}",
            unmarked_sum,
            winning_draw,
            unmarked_sum * winning_draw as u32
        );
    }

    // part 2
    // just use boards here since we don't need it again after this
    let mut boards = boards;
    winning_board_op = None;
    winning_draw_op = None;
    let mut won_boards = HashMap::new();
    for draw in &draws {
        if let Some(board_indexes) = map.get(draw) {
            for board_index in board_indexes {
                if won_boards.get(&board_index.board_index).is_none() {
                    board_index.get_ref_mut(&mut boards).marked = true;
                    if board_index.won(&boards) {
                        winning_board_op = Some(boards[board_index.board_index]);
                        winning_draw_op = Some(*draw);
                        won_boards.insert(board_index.board_index, true);
                    }
                }
            }
        }
    }
    if let Some(winning_board) = winning_board_op {
        let unmarked_sum = winning_board
            .iter()
            .map(|r| {
                r.iter()
                    .filter_map(|n| match n.marked {
                        true => None,
                        false => Some(n.value as u32),
                    })
                    .sum::<u32>()
            })
            .sum::<u32>();
        let winning_draw = winning_draw_op.unwrap();
        println!(
            "Part 2: unmarked_sum = {}, winning_draw = {}, product = {}",
            unmarked_sum,
            winning_draw,
            unmarked_sum * winning_draw as u32
        );
    }
}
