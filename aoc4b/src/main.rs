use ndarray::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use String;

// a bingo board
#[derive(Clone)]
struct Board {
    entries: ndarray::Array2<u8>,
    marked: ndarray::Array2<u8>,
}

//read input
fn read_lines(input: &Path) -> Vec<String> {
    let f: File = File::open(input).expect("File not found");
    let reader: BufReader<File> = BufReader::new(f);
    reader.lines().map(|line| line.unwrap()).collect()
}

//read game
fn read_game(lines: &Vec<String>) -> (Vec<u8>, Vec<Board>) {
    let bingo_calls: Vec<u8> = lines[0]
        .split(",")
        .map(|e| e.parse::<u8>().unwrap())
        .collect();

    let mut iter = lines.iter();
    let _ = iter.next();
    let _ = iter.next();

    let (mut boards, last_board): (Vec<Board>, Vec<u8>) = iter.fold(
        (Vec::<Board>::new(), Vec::<u8>::new()),
        |(mut boards, mut partial_board), line| match line.as_str() {
            "" => {
                // let mut b_vec = boards.clone();
                let b: Board = Board {
                    entries: Array::from_shape_vec((5, 5), partial_board).unwrap(),
                    marked: Array2::<u8>::from_elem((5, 5), 0),
                };
                boards.push(b);
                (boards, Vec::<u8>::new())
            }
            _ => {
                line.split_whitespace()
                    .for_each(|e| partial_board.push(e.trim().parse::<u8>().unwrap()));
                (boards, partial_board)
            }
        },
    );

    // push the last board since it would not have been collected in the iterator
    boards.push(Board {
        entries: Array::from_shape_vec((5, 5), last_board).unwrap(),
        marked: Array2::<u8>::from_elem((5, 5), 0),
    });

    (bingo_calls, boards)
}

fn declare_bingo(marked: &ndarray::Array2<u8>) -> bool {
    let r_sum = marked.sum_axis(Axis(0));
    let c_sum = marked.sum_axis(Axis(1));

    r_sum.iter().any(|e| e.eq(&(5 as u8))) || c_sum.iter().any(|e| e.eq(&(5 as u8)))
}

fn main() {
    // read input as lines
    let lines: Vec<String> = read_lines(Path::new("/Users/srinivm/develop/aoc/aoc4a/input"));

    // parse lines into bingo calls and boards
    let (bingo_calls, mut boards) = read_game(&lines);

    for x in bingo_calls {
        // mark called out number on each board
        boards.iter_mut().for_each(|mut board| {
            // turn x into a 5x5 array2d
            let x_matrix = Array2::<u8>::from_elem((5, 5), x);
            let mut entries_mask = Array2::<u8>::from_elem((5, 5), 0);
            azip!((a in &mut entries_mask, &b in &x_matrix, &c in &board.entries) *a = if b == c {1} else {0});

            let mut new_marked = Array2::<u8>::from_elem((5, 5), 0);
            azip!((a in &mut new_marked, &b in &entries_mask, &c in &board.marked) *a = if (b + c) == 0 {0} else {1});
            board.marked = new_marked;
        });

        // mark which boards have not won this time
        let mut play_boards: Vec<Board> = Vec::new();
        for (index, board) in boards.iter().enumerate() {
            if !declare_bingo(&board.marked) {
                play_boards.push(board.clone());
            }
        }
        // if this was the last board that won, return the result
        if boards.len() == 1 && play_boards.len() == 0 {
            let result = Array2::<u8>::from((!&boards[0].marked - 254) * &boards[0].entries);
            let r = result.mapv(|e| e as i32);
            println!("{}", r.sum() * (x as i32));
            return;
        }

        boards = play_boards.clone();
    }
}
