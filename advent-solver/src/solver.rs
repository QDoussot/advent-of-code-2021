#[derive(Debug)]
pub enum Error {
    WrongInput,
}

pub trait Solver {
    fn solve(&self, lines: &[String]) -> Result<String, Error>;
}

pub fn new<So: Default>() -> Box<So> {
    Box::new(So::default())
}
