use std::fs::File;
use std::io::{BufReader,BufRead, Read};


fn main() {
    let f: File = File::open("/Users/srinivm/develop/aoc/aoc1/input").unwrap();
    let reader: BufReader<File> = BufReader::new(f);

    let array:Vec<i32> = reader.lines().map(|item| item.unwrap().to_string().parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let num:i32 = array.iter().zip(array.iter().skip(1)).fold(0,
    |accum:i32, (&item1, &item2)| {
        accum + (item2 > item1) as i32
    });
    println!("{}", num);
}
