use std::{collections::HashMap, str::FromStr};

use derive_more::Display;
use itertools::Itertools;

use crate::solver::{Parse, Solver};

#[derive(Display)]
pub enum ParsingError {
    #[display(fmt = "Not a digit: '{}'", _0)]
    NotADigit(String),
    #[display(fmt = "Mal")]
    MalformedPoint(String),
    StartPointMissing,
    MiddleArrowMissing,
    EndPointMissing,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]

pub struct Point {
    x: i64,
    y: i64,
}

impl From<(i64, i64)> for Point {
    fn from(p: (i64, i64)) -> Self {
        Point { x: p.0, y: p.1 }
    }
}

impl FromStr for Point {
    type Err = ParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: [i64; 2] = s
            .split(",")
            .map(|coord| i64::from_str(coord).map_err(|_| ParsingError::NotADigit(coord.into())))
            .collect::<Result<Vec<_>, Self::Err>>()?
            .try_into()
            .map_err(|_| ParsingError::MalformedPoint(s.into()))?;
        Ok(Point {
            x: coords[0],
            y: coords[1],
        })
    }
}

#[derive(Debug)]
pub struct Line {
    start: Point,
    end: Point,
}

impl FromStr for Line {
    type Err = ParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split(" ");
        let start = tokens
            .next()
            .ok_or(ParsingError::StartPointMissing)
            .and_then(Point::from_str)?;

        if let Some("->") = tokens.next() {
        } else {
            return Err(ParsingError::MiddleArrowMissing);
        }

        let end = tokens
            .next()
            .ok_or(ParsingError::EndPointMissing)
            .and_then(Point::from_str)?;

        Ok(Line { start, end })
    }
}

impl Line {
    pub fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }

    pub fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }
    pub fn is_simple(&self) -> bool {
        self.is_horizontal() || self.is_vertical()
    }

    pub fn is_diagonal(&self) -> bool {
        self.start.x - self.end.x == self.start.y - self.end.y
            || self.start.x - self.end.x == -(self.start.y - self.end.y)
    }
}

impl From<(usize, usize, usize, usize)> for Line {
    fn from(line: (usize, usize, usize, usize)) -> Self {
        Line {
            start: Point {
                x: line.0 as i64,
                y: line.1 as i64,
            },
            end: Point {
                x: line.2 as i64,
                y: line.3 as i64,
            },
        }
    }
}

impl IntoIterator for Line {
    type Item = Point;

    type IntoIter = LineIterator;

    fn into_iter(self) -> Self::IntoIter {
        LineIterator {
            line: self,
            started: false,
            finished: false,
        }
    }
}

pub struct LineIterator {
    line: Line,
    started: bool,
    finished: bool,
}

fn orientation(start: i64, end: i64) -> i64 {
    match end - start {
        delta if delta < 0 => -1i64,
        delta if delta > 0 => 1,
        _ => 0,
    }
}

impl Iterator for LineIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.started {
            self.started = true;
            return Some(self.line.start);
        }
        if self.finished {
            return None;
        }
        if self.line.is_simple() || self.line.is_diagonal() {
            if self.line.end == self.line.start {
                self.finished = true;
                None
            } else {
                let delta_x = orientation(self.line.start.x, self.line.end.x);
                let delta_y = orientation(self.line.start.y, self.line.end.y);
                self.line.start.x += delta_x;
                self.line.start.y += delta_y;
                Some(self.line.start)
            }
        } else {
            unimplemented!()
        }
    }
}

trait Exercice {
    type Error;
    type Solution;
    fn parse() -> Result<(), Self::Solution>;
    fn solve_part_one() -> Result<usize, Self::Error>;
}

#[derive(Debug, Default)]
pub struct First {}
impl Parse for First {
    type ProblemModel = Vec<Line>;

    fn parse(&self, lines: &[String]) -> Result<Self::ProblemModel, crate::solver::Error> {
        lines
            .iter()
            .map(AsRef::as_ref)
            .enumerate()
            .map(|(n, line)| {
                Line::from_str(line).map_err(|e| crate::solver::Error::WrongLine {
                    description: e.to_string(),
                    line: "".into(),
                    line_number: n,
                })
            })
            .collect::<Result<Vec<_>, _>>()
    }
}

impl Solver for First {
    type ProblemModel = Vec<Line>;

    type Solution = usize;

    fn solve(&self, model: Self::ProblemModel) -> Result<Self::Solution, crate::solver::Error> {
        let model = model;
        let count_map: HashMap<Point, usize> = model
            .into_iter()
            .filter(Line::is_simple)
            .flat_map(IntoIterator::into_iter)
            .into_group_map_by(|p| *p)
            .into_iter()
            .map(|occ| (occ.0, occ.1.len()))
            .collect();

        let inter = count_map.iter().filter(|(_p, count)| **count >= 2).collect::<Vec<_>>();
        Ok(inter.len())
    }
}

#[derive(Debug, Default)]
pub struct Second {}

impl Solver for Second {
    type ProblemModel = Vec<Line>;

    type Solution = usize;

    fn solve(&self, model: Self::ProblemModel) -> Result<Self::Solution, crate::solver::Error> {
        let is_easy_geo = |line: &Line| -> bool { line.is_simple() || line.is_diagonal() };
        let model = model;
        let count_map: HashMap<Point, usize> = model
            .into_iter()
            .filter(is_easy_geo)
            .flat_map(IntoIterator::into_iter)
            .into_group_map_by(|p| *p)
            .into_iter()
            .map(|occ| (occ.0, occ.1.len()))
            .collect();

        let inter = count_map.iter().filter(|(_p, count)| **count >= 2).collect::<Vec<_>>();
        Ok(inter.len())
    }
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn it_iterates_diagonals() {
        let line: Line = (1, 1, 4, 4).into();
        let expected: Vec<Point> = [(1, 1), (2, 2), (3, 3), (4, 4)].map(Into::into).into();
        let result = line.into_iter().collect::<Vec<_>>();
        assert_eq!(result, expected);
    }

    #[test]
    fn it_knows_which_lines_are_diagonals() {
        let diagonals: Vec<Line> = [(5, 5, 0, 0), (5, 5, 10, 0), (5, 5, 10, 10), (5, 5, 0, 10)]
            .map(Into::into)
            .into();
        for line in diagonals {
            assert!(line.is_diagonal(), "{:?} supposed to be diagonal", line)
        }
    }
}
