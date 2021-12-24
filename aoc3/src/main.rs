// https://adventofcode.com/2021/day/3
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let f: File = File::open("/Users/srinivm/develop/aoc/aoc3/input").expect("File not found");
    let reader: BufReader<File> = BufReader::new(f);

    let totals: [i32; 12] = reader.lines().fold([0; 12], |acc: [i32; 12], line| {
        let mut new_state = acc.clone();
        line.unwrap().chars().enumerate().for_each(|(i, bit_val)| {
            new_state[i] = new_state[i]
                + match bit_val {
                    '0' => -1,
                    '1' => 1,
                    _ => 0,
                }
        });
        new_state
    });

    let [gamma, epsilon]: [i32; 2] = totals.iter().fold([0, 0], |acc: [i32; 2], total| {
        // first element is gamma rate, second is epsilon rate
        match total < &0 {
            true => [(acc[0] << 1) | 0b0, (acc[1] << 1) | 0b1], // there are more '0's
            false => [(acc[0] << 1) | 0b1, (acc[1] << 1) | 0b0], // there are more '1's
        }
    });
    println!("{}", gamma * epsilon);
}
