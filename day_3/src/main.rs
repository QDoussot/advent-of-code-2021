#![feature(array_zip)]

use std::io::{self, BufRead, BufReader};

mod day_3;

fn main() {
    let stdin = BufReader::new(io::stdin());
    let lines = stdin.lines().collect::<Result<Vec<_>, io::Error>>().unwrap();
    println!("{:?}", day_3::first_part(lines));
}
