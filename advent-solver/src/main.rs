#![feature(array_zip)]
#![feature(int_abs_diff)]
#![feature(bool_to_option)]

mod application;
mod bin_seq;
mod pop_array;

use solver::ParsingError;
use solver::SolvingError;
use std::io::{self, BufRead, BufReader};
use std::str::FromStr;
use structopt::StructOpt;

mod day_1;
use day_1::First as Day1First;
use day_1::Parser as Day1Parser;
use day_1::Second as Day1Second;

mod day_2;
use day_2::First as Day2First;
use day_2::Parser as Day2Parser;
use day_2::Second as Day2Second;

mod day_3;
use day_3::First as Day3First;
use day_3::Parser as Day3Parser;
use day_3::Second as Day3Second;

mod day_4;
use day_4::First as Day4First;
use day_4::Parser as Day4Parser;
use day_4::Second as Day4Second;

mod day_5;
use day_5::First as Day5First;

mod day_6;
use day_6::LanterfishCrew as Day6First;

mod day_7;
use day_7::First;

mod day_8;
use day_8::SevenSegmentSearch;

mod solver;
use solver::Exercice;

mod schooler;
use schooler::Schooler;

enum Part {
    One,
    Two,
}

#[derive(StructOpt)]
struct Opt {
    day: usize,
    part: usize,
    #[structopt(long)]
    input: Option<String>,
    #[structopt(long, conflicts_with = "input")]
    example: bool,
}

#[derive(Debug)]
enum Error {
    CantOpenInputFile(String),
    ParsingFailed(solver::ParsingError),
    NoCorrespondingSolver,
    SolverFailed(solver::Error),
}

use solver::Problem;
struct Example {}

impl Problem for Example {
    fn parse(lines: &[String]) -> Result<Self, ParsingError> {
        todo!()
    }

    fn part_one(&self) -> Result<usize, SolvingError> {
        todo!()
    }

    fn part_two(&self) -> Result<usize, SolvingError> {
        todo!()
    }
    //
}

fn solve<T: Problem>(lines: &[String], part: usize) -> Result<usize, Error> {
    let problem = T::parse(&lines).map_err(|e| Error::ParsingFailed(e))?;
    if part == 1 {
        problem.part_one().map_err(|_| Error::NoCorrespondingSolver)
    } else {
        problem.part_two().map_err(|_| Error::NoCorrespondingSolver)
    }
}

fn main() -> Result<(), Error> {
    let opt = Opt::from_args();

    let file_name = match opt.input {
        None => {
            let ext = match opt.example {
                false => "",
                true => ".example",
            };
            format!("inputs/{}{}", opt.day, ext)
        }
        Some(file_name) => file_name,
    };
    let file = std::fs::File::open(file_name).map_err(|e| Error::CantOpenInputFile(e.to_string()))?;
    let lines = BufReader::new(file)
        .lines()
        .collect::<Result<Vec<_>, io::Error>>()
        .unwrap();

    let solution = match opt.day {
        8 => solve::<SevenSegmentSearch>(&lines, opt.part)?,
        _ => return Err(Error::NoCorrespondingSolver),
    };
    println!("{}", solution);

    Ok(())
}
