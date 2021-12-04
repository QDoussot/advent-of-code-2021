#[derive(Debug)]

pub enum Error {}

pub trait Solver {
    fn solve(&self, lines: &[String]) -> Result<String, Error>;
}

pub fn new<So: Default>() -> Box<So> {
    Box::new(So::default())
}
