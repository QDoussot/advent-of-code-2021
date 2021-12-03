use std::{num::ParseIntError, str::FromStr};

pub(crate) enum Move {
    Forward(usize),
    Down(usize),
    Up(usize),
}

#[derive(Debug)]
pub enum Error {
    UnexpectedFormat,
    ParseIntError(ParseIntError),
}

impl std::str::FromStr for Move {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, amplitude) = s.split_once(" ").ok_or_else(|| Error::UnexpectedFormat)?;
        let amplitude = usize::from_str(amplitude).map_err(Error::ParseIntError)?;
        match direction {
            "forward" => Ok(Move::Forward(amplitude)),
            "down" => Ok(Move::Down(amplitude)),
            "up" => Ok(Move::Up(amplitude)),
            _ => Err(Error::UnexpectedFormat),
        }
    }
}

impl TryFrom<String> for Move {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Move::from_str(&value)
    }
}

pub(crate) fn part_1(moves: Vec<Move>) -> usize {
    let (mut x, mut depth) = (0, 0);
    for a_move in moves {
        match a_move {
            Move::Forward(amp) => {
                x += amp;
            }
            Move::Down(amp) => depth += amp,
            Move::Up(amp) => depth -= amp,
        }
    }
    x * depth
}

pub(crate) fn part_2(moves: Vec<Move>) -> usize {
    let (mut x, mut aim, mut depth) = (0, 0i64, 0usize);
    for a_move in moves {
        match a_move {
            Move::Forward(amp) => {
                x += amp;
                depth += (aim * amp as i64) as usize;
            }
            Move::Down(amp) => aim += amp as i64,
            Move::Up(amp) => aim -= amp as i64,
        }
    }
    x * depth
}
