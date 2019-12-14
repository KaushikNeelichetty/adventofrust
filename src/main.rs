use std::fs::File;
use std::io::{BufRead, BufReader};

use day01::*;

mod day01;

fn main() -> Result<(), std::io::Error> {
    let input = read_input("input/day01");
    println!("Day 01, Part 1: {}", day01a(input));
    Ok(())
}

fn read_input(file_name: &str) -> Vec<i32> {
    let file_path = file_name;
    let file = File::open(file_path)?;
    let buffered_reader = BufReader::new(file);
    let input = buffered_reader
        .lines()
        .map(|s| s.unwrap().parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    input
}
