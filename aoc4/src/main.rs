use ndarray::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use String;

// a bingo board
#[derive(Clone)]
struct Board {
    entries: ndarray::Array2<u8>,
    marked: ndarray::Array2<bool>,
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

    // let marked_entries = [[false; 5]; 5];
    let (mut boards, mut last_board): (Vec<Board>, Vec<u8>) = iter.fold(
        (Vec::<Board>::new(), Vec::<u8>::new()),
        |(mut boards, mut partial_board), line| match line.as_str() {
            "" => {
                // let mut b_vec = boards.clone();
                let b: Board = Board {
                    entries: Array::from_shape_vec((5, 5), partial_board).unwrap(),
                    marked: Array2::<bool>::from_elem((5, 5), true),
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
        marked: Array2::<bool>::from_elem((5, 5), true),
    });

    (bingo_calls, boards)
}

fn main() {
    // read input as lines
    let lines: Vec<String> = read_lines(Path::new("/Users/srinivm/develop/aoc/aoc4/input"));

    // parse lines into bingo calls and boards
    let (bingo_calls, boards) = read_game(&lines);
    boards.iter().for_each(|b| {
        println!("{:?}", b.entries);
    });
    // let m = array![
    //     [1, 2, 3, 4, 5],
    //     [6, 7, 8, 9, 1],
    //     [1, 2, 3, 4, 5],
    //     [6, 7, 8, 9, 1],
    //     [1, 2, 3, 4, 5]
    // ];
    //
    // let t = array![
    //     [1, 2, 3, 4, 5],
    //     [6, 7, 8, 9, 1],
    //     [1, 2, 5, 4, 5],
    //     [6, 7, 8, 9, 1],
    //     [1, 2, 2, 4, 5]
    // ];
    //
    // type M = Array2<bool>;
    // let mut a = M::from_elem(m.dim(), true);
    //
    // println!("{}", m);
    // println!("{}", t);
    //
    // azip!((a in &mut a, &b in &m, &c in &t) *a = b == c);
    //
    // println!("{}", a);
    //
    // let aa = Array::from_shape_vec((2, 2), vec![1., 2., 3., 4.]).unwrap();
    // println!("{}", aa);
}
