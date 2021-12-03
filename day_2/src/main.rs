#![feature(array_zip)]

use std::io::{self, BufRead, BufReader};

mod day_2;
use day_2::Move;

use crate::day_2::Error;

fn main() {
    let stdin = BufReader::new(io::stdin());
    let lines = stdin.lines().collect::<Result<Vec<_>, io::Error>>().unwrap();
    let moves = lines
        .into_iter()
        .map(Move::try_from)
        .collect::<Result<Vec<_>, Error>>()
        .unwrap();
    println!("{:?}", day_2::part_2(moves));
}
