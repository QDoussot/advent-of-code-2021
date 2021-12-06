use std::{collections::HashMap, str::FromStr};

use derive_more::Display;

use crate::solver::{Parse, Solver};

#[derive(Display)]
pub enum ParsingError {
    #[display(fmt = "Not a digit: '{}'", _0)]
    NotADigit(String),
    #[display(fmt = "Mal")]
    MalformedPoint(String),
    MalformedLine,
    StartPointMissing,
    MiddleArrowMissing,
    EndPointMissing,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]

pub struct Point {
    x: i64,
    y: i64,
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

struct Day5 {
    wind_lines: Vec<Line>,
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
        let mut count_map: HashMap<Point, usize> = HashMap::<Point, usize>::new();
        for line in model.into_iter().filter(Line::is_simple) {
            //println!("{:?}", line);
            line.into_iter().for_each(|p| {
                if let Some(count) = count_map.get_mut(&p) {
                    *count += 1;
                } else {
                    count_map.insert(p, 1);
                }
            });
        }
        let inter = count_map.iter().filter(|(p, count)| **count >= 2).collect::<Vec<_>>();
        //println!("{:?}", inter);
        Ok(inter.len())
    }
}
