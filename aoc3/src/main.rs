// https://adventofcode.com/2021/day/3
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use String;

fn get_diagnostics(input: &Path) -> Vec<String> {
    let f: File = File::open(input).expect("File not found");
    let reader: BufReader<File> = BufReader::new(f);
    reader.lines().map(|line| line.unwrap()).collect()
}

fn tally_bits(diagnostics: &Vec<String>) -> [i32; 12] {
    diagnostics.iter().fold([0; 12], |acc: [i32; 12], line| {
        let mut new_state = acc.clone();
        line.chars().enumerate().for_each(|(i, bit_val)| {
            new_state[i] = new_state[i]
                + match bit_val {
                    '0' => -1,
                    '1' => 1,
                    _ => 0,
                }
        });
        new_state
    })
}

fn power_consumption(totals: &[i32; 12]) -> i32 {
    let [gamma, epsilon]: [i32; 2] = totals.iter().fold([0, 0], |acc: [i32; 2], total| {
        // first element is gamma rate, second is epsilon rate
        match total < &0 {
            true => [(acc[0] << 1) | 0b0, (acc[1] << 1) | 0b1], // there are more '0's
            false => [(acc[0] << 1) | 0b1, (acc[1] << 1) | 0b0], // there are more '1's
        }
    });
    gamma * epsilon
}

fn part_1() {
    let diagnostics: Vec<String> =
        get_diagnostics(Path::new("/Users/srinivm/develop/aoc/aoc3/input"));
    let tallies: [i32; 12] = tally_bits(&diagnostics);

    println!("{}", power_consumption(&tallies));
}

fn main() {
    part_1();
    let oxygen = oxygen_generator_rating();
    let co2 = co2_scrubber_rating();
    println!("{}", oxygen * co2);
}
fn oxygen_generator_rating() -> i32 {
    let diagnostics: Vec<String> =
        get_diagnostics(Path::new("/Users/srinivm/develop/aoc/aoc3/input"));

    let oxygen: Vec<String> = (0..12).fold(diagnostics, |working_set, bit_pos: usize| {
        let tallies: [i32; 12] = tally_bits(&working_set);
        let curr_char = match tallies[bit_pos] >= 0 {
            true => '1',
            false => '0',
        };
        match working_set.len() == 1 {
            true => working_set,
            false => working_set
                .iter()
                .cloned()
                .filter(|item| item.as_bytes()[bit_pos] == (curr_char as u8))
                .collect::<Vec<String>>(),
        }
    });

    oxygen[0].chars().fold(0, |acc: i32, c| match c {
        '0' => (acc << 1) | 0b0,
        '1' => (acc << 1) | 0b1,
        _ => acc,
    })
}

fn co2_scrubber_rating() -> i32 {
    let diagnostics: Vec<String> =
        get_diagnostics(Path::new("/Users/srinivm/develop/aoc/aoc3/input"));

    let co2: Vec<String> = (0..12).fold(diagnostics, |working_set, bit_pos: usize| {
        let tallies: [i32; 12] = tally_bits(&working_set);
        let curr_char = match tallies[bit_pos] >= 0 {
            true => '0',
            false => '1',
        };
        match working_set.len() == 1 {
            true => working_set,
            false => working_set
                .iter()
                .cloned()
                .filter(|item| item.as_bytes()[bit_pos] == (curr_char as u8))
                .collect::<Vec<String>>(),
        }
    });
    co2[0].chars().fold(0, |acc: i32, c| match c {
        '0' => (acc << 1) | 0b0,
        '1' => (acc << 1) | 0b1,
        _ => acc,
    })
}
