use nalgebra::{geometry, Point2};
use ndarray::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use String;

struct VentLines {
    from: nalgebra::Point2<i32>,
    to: nalgebra::Point2<i32>,
}

// read input
fn read_lines(input: &Path) -> Vec<String> {
    let f: File = File::open(input).expect("File not found");
    let reader: BufReader<File> = BufReader::new(f);
    reader.lines().map(|line| line.unwrap()).collect()
}

fn main() {
    // read input as lines
    let lines: Vec<String> = read_lines(Path::new("/Users/srinivm/develop/aoc/aoc5/input"));
    println!("{:?}", lines);
}
