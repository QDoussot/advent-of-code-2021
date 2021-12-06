use crate::solver::{Exercice, Parse, Solver};

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

impl<S, P> Exercice for Schooler<P, S>
where
    S: Solver,

    P: Parse<ProblemModel = S::ProblemModel>,
{
    fn solve(&self, lines: &[String]) -> Result<String, crate::solver::Error> {
        let model = self.parser.parse(lines)?;
        self.solver.solve(model).map(|sol| sol.to_string()).map_err(Into::into)
    }
}
