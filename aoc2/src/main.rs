use std::array::TryFromSliceError;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufRead, BufReader, Read};

fn main() {
    part_1();
    part_2();
}

fn part_2() {
    let f: File = File::open("/Users/srinivm/develop/aoc/aoc2/input").unwrap();
    let reader: BufReader<File> = BufReader::new(f);

    let sum: [i32; 3] =
        reader.lines().fold(
            [0, 0, 0],
            |acc: [i32; 3], line| match <&[&str; 2]>::try_from(
                line.unwrap().split(' ').collect::<Vec<&str>>().as_slice(),
            )
            .unwrap()
            {
                ["forward", units] => [
                    acc[0] + (units.parse::<i32>().unwrap()),
                    acc[1] + (units.parse::<i32>().unwrap() * acc[2]),
                    acc[2],
                ],
                ["down", units] => [acc[0], acc[1], acc[2] + (units.parse::<i32>().unwrap())],
                ["up", units] => [acc[0], acc[1], acc[2] - (units.parse::<i32>().unwrap())],
                _ => [acc[0], acc[1], acc[2]],
            },
        );

    println!("{}", sum[0] * sum[1]);
}

fn part_1() {
    let f: File = File::open("/Users/srinivm/develop/aoc/aoc2/input").unwrap();
    let reader: BufReader<File> = BufReader::new(f);

    let sum: [i32; 2] =
        reader
            .lines()
            .fold([0, 0], |acc: [i32; 2], line| match <&[&str; 2]>::try_from(
                line.unwrap().split(' ').collect::<Vec<&str>>().as_slice(),
            )
            .unwrap()
            {
                ["forward", units] => [acc[0] + (units.parse::<i32>().unwrap()), acc[1]],
                ["down", units] => [acc[0], acc[1] + (units.parse::<i32>().unwrap())],
                ["up", units] => [acc[0], acc[1] - (units.parse::<i32>().unwrap())],
                _ => [acc[0], acc[1]],
            });

    println!("{}", sum[0] * sum[1]);
}
