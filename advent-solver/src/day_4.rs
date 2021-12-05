use crate::solver::{self, Parse, Solver};

use ansi_term::Style;
use derive_more::Display;

use ansi_term::ANSIString;
use std::collections::LinkedList;
use std::fmt::Display;
use std::num::ParseIntError;
use std::str::FromStr;

//

#[derive(Debug, Clone)]
struct Element {
    number: usize,
    played: bool,
}

impl Element {
    fn new(number: usize) -> Self {
        Self { number, played: false }
    }

    fn play(&mut self) {
        self.played = true;
    }

    fn to_string(&self) -> ANSIString<'static> {
        if self.played {
            Style::new().bold().paint(self.number.to_string())
        } else {
            self.number.to_string().into()
        }
    }
}

impl AsRef<usize> for Element {
    fn as_ref(&self) -> &usize {
        &self.number
    }
}

impl FromStr for Element {
    type Err = LineParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        usize::from_str(s)
            .map_err(|e| LineParsingError::ElementIsNotANumber(s.to_string(), e))
            .map(Element::new)
    }
}

#[derive(Debug)]
pub struct Board {
    grid: [[Element; 5]; 5],
}

impl Board {
    fn play(&mut self, number: usize) {
        self.grid.iter_mut().for_each(|line| {
            line.iter_mut().for_each(|el| {
                if el.number == number {
                    el.play();
                }
            })
        })
    }

    fn wins(&self) -> Option<usize> {
        if self.grid.iter().any(|line| line.iter().all(|el| el.played))
            || (0..5).any(|col| self.grid.iter().all(|line| line[col].played))
        {
            let as_point = |el: &Element| -> usize {
                if el.played {
                    0
                } else {
                    el.number
                }
            };

            let sum: usize = self
                .grid
                .iter()
                .map(|line| line.iter().map(as_point).sum::<usize>())
                .sum();
            Some(sum)
        } else {
            None
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.grid {
            for el in line {
                write!(f, "{:>5}  ", el.to_string())?
            }
            writeln!(f, "")?
        }
        Ok(())
    }
}

#[derive(Debug, Display)]
pub enum LineParsingError {
    #[display(fmt = "Element {} is not a number: {}", _0, _1)]
    ElementIsNotANumber(String, ParseIntError),

    #[display(fmt = "Wrong length {}", "_0")]
    WrongLength(usize),
}

#[derive(Debug, Display)]
pub enum BoardParsingError {
    #[display(fmt = r#"Line parsing error '{}' for line {}:'{}'"#, "error", "line_number", "line")]
    WrongLine {
        line_number: usize,
        line: String,
        error: LineParsingError,
    },
    UnexpectedEndOfInput,
}

impl Into<solver::Error> for BoardParsingError {
    fn into(self) -> solver::Error {
        solver::Error::ExpectationUnfulfilled(self.to_string())
    }
}

fn parse_line<'a>(line: &'a String) -> Result<[Element; 5], LineParsingError> {
    line.split_ascii_whitespace()
        .map(Element::from_str)
        .collect::<Result<Vec<_>, _>>()?
        .try_into()
        .map_err(|vec: Vec<_>| LineParsingError::WrongLength(vec.len()))
}

impl Board {
    pub fn parse<'a>(lines: &mut impl Iterator<Item = (usize, &'a String)>) -> Result<Self, BoardParsingError> {
        let mut grid: [[Element; 5]; 5] = [
            [
                Element::new(0),
                Element::new(0),
                Element::new(0),
                Element::new(0),
                Element::new(0),
            ],
            [
                Element::new(0),
                Element::new(0),
                Element::new(0),
                Element::new(0),
                Element::new(0),
            ],
            [
                Element::new(0),
                Element::new(0),
                Element::new(0),
                Element::new(0),
                Element::new(0),
            ],
            [
                Element::new(0),
                Element::new(0),
                Element::new(0),
                Element::new(0),
                Element::new(0),
            ],
            [
                Element::new(0),
                Element::new(0),
                Element::new(0),
                Element::new(0),
                Element::new(0),
            ],
        ];
        for i in 0..5 {
            let (number, line) = lines.next().ok_or(BoardParsingError::UnexpectedEndOfInput)?;
            grid[i] = parse_line(line).map_err(|error| BoardParsingError::WrongLine {
                line_number: number + 1,
                line: line.into(),
                error,
            })?;
        }
        Ok(Board { grid })
    }
}

#[derive(Debug)]
pub struct Bingo {
    numbers: LinkedList<usize>,
    boards: Vec<Board>,
}

pub enum Status {
    Finished,
    Playing(Vec<usize>),
}

impl Status {
    fn as_win(self) -> Option<Vec<usize>> {
        use Status::Playing;
        match self {
            Playing(win) if win.is_empty() => None,
            Playing(win) => Some(win),
            _ => None,
        }
    }
}

impl Bingo {
    fn playable_boards(&mut self) -> impl Iterator<Item = &mut Board> {
        self.boards.iter_mut().filter(|b| !b.wins().is_some())
    }
    fn play_next_number(&mut self) -> Status {
        if let Some(number) = self.numbers.pop_front() {
            let wins = self
                .playable_boards()
                .filter_map(|b| {
                    b.play(number);
                    let win = b.wins();
                    if win.is_some() {
                        println!("Board wins at {} * {}\n {}", number, win.unwrap(), b);
                    }
                    win
                })
                .map(|points| (number * points))
                .collect();
            Status::Playing(wins)
        } else {
            Status::Finished
        }
    }
}

impl Iterator for Bingo {
    type Item = Status;

    fn next(&mut self) -> Option<Self::Item> {
        match self.play_next_number() {
            Status::Finished => None,
            s => Some(s),
        }
    }
}

#[derive(Default)]
pub struct Parser {}
impl Parse for Parser {
    type ProblemModel = Bingo;

    fn parse(&self, lines: &[String]) -> Result<Self::ProblemModel, crate::solver::Error> {
        if lines.len() < 7 {
            return Err(solver::Error::ExpectationUnfulfilled("Less than 7 lines".into()));
        }

        let mut first = lines.iter();
        let numbers = first
            .next()
            .unwrap()
            .split(",")
            .map(usize::from_str)
            .collect::<Result<LinkedList<_>, _>>()
            .map_err(|_| solver::Error::ExpectationUnfulfilled("First line has wrong format".into()))?;

        let mut boards = vec![];
        let mut lines = lines.into_iter().enumerate();
        lines.next();
        while let Some(_) = lines.next() {
            boards.push(Board::parse(&mut lines).map_err(Into::into)?)
        }

        Ok(Bingo { numbers, boards })
    }
}

#[derive(Default)]
pub struct First {}

impl Solver for First {
    type ProblemModel = Bingo;

    type Solution = usize;

    fn solve(&self, mut model: Self::ProblemModel) -> Result<Self::Solution, solver::Error> {
        model
            .find_map(|s| s.as_win())
            .map(|win_vec| *win_vec.first().unwrap())
            .ok_or(solver::Error::ExpectationUnfulfilled("No Win".into()))
    }
}

#[derive(Default)]
pub struct Second {}

impl Solver for Second {
    type ProblemModel = Bingo;

    type Solution = usize;

    fn solve(&self, model: Self::ProblemModel) -> Result<Self::Solution, solver::Error> {
        let plays = model.filter_map(|s| s.as_win()).collect::<Vec<_>>();
        println!("{:?}", plays);
        plays
            .last()
            .map(|win_vec| *win_vec.last().unwrap())
            .ok_or(solver::Error::ExpectationUnfulfilled("No Win".into()))
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn it_plays_element() {
        let mut grid = [
            [
                Element::new(0),
                Element::new(0),
                Element::new(0),
                Element::new(0),
                Element::new(0),
            ],
            [
                Element::new(0),
                Element::new(0),
                Element::new(0),
                Element::new(0),
                Element::new(0),
            ],
            [
                Element::new(0),
                Element::new(0),
                Element::new(0),
                Element::new(0),
                Element::new(0),
            ],
            [
                Element::new(0),
                Element::new(0),
                Element::new(0),
                Element::new(0),
                Element::new(0),
            ],
            [
                Element::new(0),
                Element::new(0),
                Element::new(0),
                Element::new(0),
                Element::new(0),
            ],
        ];

        grid[1][3].number = 4;
        for line in &grid {
            for el in line {}
        }
        //grid[1][3].played = true;

        let mut board = Board { grid };
        println!("{}", board);
        board.play(4);
        println!("{}", board);
    }
}
