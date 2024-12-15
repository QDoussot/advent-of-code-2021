#![feature(array_zip)]

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

mod solver;
use solver::{Exercice, Schooler};

struct Fake {}

impl Exercice for Fake {
    fn solve(&self, _: &[String]) -> Result<String, solver::Error> {
        Ok("Fake solution".into())
    }
}

struct True {}
impl Exercice for True {
    fn solve(&self, _: &[String]) -> Result<String, solver::Error> {
        Ok("True solution".into())
    }
}

//struct BoundedInteger<const MIN: usize, const MAX: usize> {
//value: usize,
//}

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

    let solvers: [[Box<dyn Exercice>; 2]; 3] = {
        use solver::{new, Unimplemented};
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
        ]
    };

    if opt.day > solvers.len() || !((1..=2).contains(&opt.part)) {
        return Err(Error::NoCorrespondingSolver);
    }

    let lines = match opt.input {
        None => {
            let ext = match opt.example {
                false => "",
                true => ".example",
            };
            let file = std::fs::File::open(format!("inputs/{}{}", opt.day, ext))
                .map_err(|e| Error::CantOpenInputFile(e.to_string()))?;
            BufReader::new(file)
                .lines()
                .collect::<Result<Vec<_>, io::Error>>()
                .unwrap()
        }
        Some(_) => {
            //let lines = stdin.lines().collect::<Result<Vec<_>, io::Error>>().unwrap();
            todo!()
        }
    };

    let solution = solvers[opt.day - 1][opt.part - 1].solve(&lines);
    println!("{}", solution.map_err(Error::SolverFailed)?);

    Ok(())
}
