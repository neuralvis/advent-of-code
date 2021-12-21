use std::fs::File;
use std::io::{BufRead, BufReader, Read};

fn main() {
    let f: File = File::open("/Users/srinivm/develop/aoc/aoc1/input").unwrap();
    let reader: BufReader<File> = BufReader::new(f);

    let array: Vec<i32> = reader
        .lines()
        .map(|item| item.unwrap().to_string().parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    part_1(&array);
    part_2(&array);
}

fn part_1(array: &Vec<i32>) {
    let num: i32 = array
        .iter()
        .zip(array.iter().skip(1))
        .fold(0, |accum: i32, (&item1, &item2)| {
            accum + (item2 > item1) as i32
        });
    println!("{}", num);
}

fn part_2(array: &Vec<i32>) {
    let num: i32 =
        array
            .windows(3)
            .zip(array.windows(3).skip(1))
            .fold(0, |accum: i32, (item1, item2)| {
                accum + ((item2[0] + item2[1] + item2[2]) > (item1[0] + item1[1] + item1[2])) as i32
            });
    println!("{}", num);
}
