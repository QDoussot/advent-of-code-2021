use derive_more::Display;
use std::fmt::Debug;
use std::{fmt::Display, marker::PhantomData};

#[derive(Debug)]
pub enum Error {
    UnimplementedSolver,
    EmptyFile,
    WrongLine {
        description: String,
        line_number: usize,
        line: String,
    },
    ExpectationUnfulfilled(String),
    Unexpected,
}

#[derive(Display, Debug)]
pub enum FormatError {
    UnexpectedCharacter,
    WrongLenght(usize),
}

pub trait Exercice {
    fn solve(&self, lines: &[String]) -> Result<String, Error>;
}

pub struct Schooler<P, S>
where
    P: Parse<ProblemModel = S::ProblemModel>,
    S: Solver,
{
    parser: P,
    solver: S,
}

impl<S: Solver, P: Parse<ProblemModel = S::ProblemModel>> Exercice for Schooler<P, S> {
    fn solve(&self, lines: &[String]) -> Result<String, Error> {
        let model = self.parser.parse(lines)?;
        self.solver.solve(model).map(|sol| sol.to_string())
    }
}

pub struct Unimplemented<P, S> {
    //
    p: PhantomData<P>,
    s: PhantomData<S>,
}
impl<P, S> Default for Unimplemented<P, S> {
    fn default() -> Self {
        Self {
            p: PhantomData::<P>::default(),
            s: PhantomData::<S>::default(),
        }
    }
}

pub trait Parse {
    type ProblemModel;
    fn parse(&self, lines: &[String]) -> Result<Self::ProblemModel, crate::solver::Error>;
}

pub trait Solver {
    type ProblemModel;
    type Solution: ToString;
    fn solve(&self, model: Self::ProblemModel) -> Result<Self::Solution, Error>;
}

impl<P, S> Solver for Unimplemented<P, S>
where
    P: Debug,
    S: Display,
{
    type ProblemModel = P;

    type Solution = S;

    fn solve(&self, _model: Self::ProblemModel) -> Result<Self::Solution, Error> {
        Err(Error::UnimplementedSolver)
    }
}
