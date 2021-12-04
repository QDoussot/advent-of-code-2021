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

pub trait Solver {
    fn solve(&self, lines: &[String]) -> Result<String, Error>;
}

pub fn new<So: Default>() -> Box<So> {
    Box::new(So::default())
}

#[derive(Default)]
pub struct Unimplemented {}
impl Solver for Unimplemented {
    fn solve(&self, _lines: &[String]) -> Result<String, Error> {
        Err(Error::UnimplementedSolver)
    }
}
