#![feature(array_zip)]
#![feature(int_abs_diff)]

use std::io::{self, BufRead, BufReader};
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

mod solver;
use solver::Exercice;

mod schooler;
use schooler::Schooler;

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
    NoCorrespondingSolver,
    SolverFailed(solver::Error),
}

fn main() -> Result<(), Error> {
    let opt = Opt::from_args();

    let solvers: [[Box<dyn Exercice>; 2]; 7] = {
        [
            [
                Schooler::<Day1Parser, Day1First>::new(),
                Schooler::<Day1Parser, Day1Second>::new(),
            ],
            [
                Schooler::<Day2Parser, Day2First>::new(),
                Schooler::<Day2Parser, Day2Second>::new(),
            ],
            [
                Schooler::<Day3Parser, Day3First>::new(),
                Schooler::<Day3Parser, Day3Second>::new(),
            ],
            [
                Schooler::<Day4Parser, Day4First>::new(),
                Schooler::<Day4Parser, Day4Second>::new(),
            ],
            [
                Schooler::<Day5First, Day5First>::new(),
                Schooler::<Day5First, day_5::Second>::new(),
            ],
            [
                Schooler::<Day6First, Day6First>::new(),
                Schooler::<Day6First, day_6::Second>::new(),
            ],
            [
                Schooler::<day_7::CrabCrew, day_7::First>::new(),
                Schooler::<day_7::CrabCrew, day_7::Second>::new(),
            ],
        ]
    };

    if opt.day > solvers.len() || !((1..=2).contains(&opt.part)) {
        return Err(Error::NoCorrespondingSolver);
    }

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

    let solution = solvers[opt.day - 1][opt.part - 1].solve(&lines);
    println!("{}", solution.map_err(Error::SolverFailed)?);

    Ok(())
}
