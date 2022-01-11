use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use String;

// a bingo board
#[derive(Clone)]
struct Board {
    entries: Vec<Vec<u8>>,
    marked: [[bool; 5]; 5],
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

    let marked_entries = [[false; 5]; 5];
    let (mut boards, last_board): (Vec<Board>, Vec<Vec<u8>>) = iter.fold(
        (Vec::new(), Vec::new()),
        |(boards, curr_board), line| match line.as_str() {
            "" => {
                let mut b_vec = boards.clone();
                let b: Board = Board {
                    entries: curr_board.clone(),
                    marked: marked_entries.clone(),
                };
                b_vec.push(b);
                (b_vec, Vec::new())
            }
            _ => {
                let mut v = curr_board.clone();
                v.push(
                    line.split_whitespace()
                        .map(|e| e.trim().parse::<u8>().unwrap())
                        .collect(),
                );
                (boards, v)
            }
        },
    );

    // push the last board since it would not have been collected in the iterator
    boards.push(Board {
        entries: last_board.clone(),
        marked: marked_entries.clone(),
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
}
