use std::io::{self, BufRead, BufReader};
use structopt::StructOpt;

mod day_1;
mod day_2;

mod solver;
use solver::Solver;

struct Fake {}

impl Solver for Fake {
    fn solve(&self, _: &[String]) -> Result<String, solver::Error> {
        Ok("Fake solution".into())
    }
}

struct True {}
impl Solver for True {
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
    SolverFailed,
}

fn main() -> Result<(), Error> {
    let opt = Opt::from_args();

    let solvers: [[Box<dyn Solver>; 2]; 2] = [
        [solver::new::<day_1::First>(), solver::new::<day_1::Second>()],
        [solver::new::<day_2::First>(), solver::new::<day_2::Second>()],
    ];
    if opt.day > solvers.len() || !((1..=2).contains(&opt.part)) {
        Err(Error::NoCorrespondingSolver)?
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
    println!("{}", solution.map_err(|_| Error::SolverFailed)?);

    Ok(())
}
