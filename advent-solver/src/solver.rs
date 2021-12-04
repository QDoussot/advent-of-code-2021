#[derive(Debug)]
pub enum Error {
    UnimplementedSolver,
    WrongLine(usize, FormatError),
    ExpectationUnfulfilled,
    Unexpected,
}

#[derive(Debug)]
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

impl<P, S> Schooler<P, S>
where
    S: Solver + Default,
    P: Parse<ProblemModel = S::ProblemModel> + Default,
{
    pub fn new() -> Box<Self> {
        Box::new(Self {
            parser: P::default(),
            solver: S::default(),
        })
    }
}

impl<S: Solver, P: Parse<ProblemModel = S::ProblemModel>> Exercice for Schooler<P, S> {
    fn solve(&self, lines: &[String]) -> Result<String, Error> {
        let model = self.parser.parse(lines)?;
        self.solver.solve(model).map(|sol| sol.to_string())
    }
}

pub fn new<So: Default>() -> Box<So> {
    Box::new(So::default())
}

#[derive(Default)]
pub struct Unimplemented {}
impl Exercice for Unimplemented {
    fn solve(&self, _lines: &[String]) -> Result<String, Error> {
        Err(Error::UnimplementedSolver)
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
