use std::fs::File;
use std::io::{BufRead, BufReader};

use day01::*;

mod day01;

fn main() -> Result<(), std::io::Error> {
    let first_env_argument = std::env::args_os().nth(1).unwrap();
    let problem_to_solve = first_env_argument.to_str().unwrap();
    match problem_to_solve {
        "day01a" => println!("{}", day01a(read_input("input/day01")?)),
        "day01b" => println!("{}", day01b(read_input("input/day01")?)),
        p => println!("Problem not found : {}", p),
    }
    Ok(())
}

fn read_input(file_name: &str) -> Result<Vec<i32>, std::io::Error> {
    let file_path = file_name;
    let file = File::open(file_path)?;
    let buffered_reader = BufReader::new(file);
    let input = buffered_reader
        .lines()
        .map(|s| s.unwrap().parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    Ok(input)
}
