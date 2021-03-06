use std::str::FromStr;

use crate::solver::{self, Exercice, Solver};

const SEQ_LEN: usize = 12;

#[derive(Debug)]
pub struct BinSeq([bool; SEQ_LEN]);

impl BinSeq {
    fn as_number(&self) -> usize {
        //
        let mut value = 0;
        for i in 0..SEQ_LEN {
            if self.0[SEQ_LEN - 1 - i] {
                value += 1 << i;
            }
        }
        value
    }

    fn matches(&self, pattern: &[bool]) -> bool {
        pattern.len() <= SEQ_LEN && pattern.iter().enumerate().all(|(i, bit)| self.0[i] == *bit)
    }
}

impl std::ops::Not for BinSeq {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(self.0.map(std::ops::Not::not))
    }
}

impl FromStr for BinSeq {
    type Err = solver::FormatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bool_array = s
            .chars()
            .map(|c| match c {
                '0' => Ok(false),
                '1' => Ok(true),
                _ => Err(solver::FormatError::UnexpectedCharacter),
            })
            .collect::<Result<Vec<bool>, solver::FormatError>>()?
            .try_into()
            .map_err(|e: Vec<bool>| solver::FormatError::WrongLenght(e.len()))?;
        Ok(Self(bool_array))
    }
}

struct OccSeq([i64; SEQ_LEN]);

impl OccSeq {
    fn new() -> Self {
        Self([0; SEQ_LEN])
    }

    fn with_account(&self, bin_seq: BinSeq) -> Self {
        Self(self.0.zip(bin_seq.0).map(|(c, y)| {
            c + match y {
                false => -1,
                true => 1,
            }
        }))
    }

    fn as_most_common(&self) -> Result<BinSeq, solver::Error> {
        let bin_seq = self
            .0
            .map(|x| match x {
                x if x < 0 => Ok(false),
                0 => Err(solver::Error::ExpectationUnfulfilled(
                    "No most common bytes when required".into(),
                )),
                _ => Ok(true),
            })
            .into_iter()
            .collect::<Result<Vec<_>, solver::Error>>()?
            .try_into()
            .map_err(|_| solver::Error::Unexpected)?;

        Ok(BinSeq(bin_seq))
    }
}

#[derive(Default)]
pub struct Parser {}
impl crate::solver::Parse for Parser {
    type ProblemModel = Vec<BinSeq>;

    fn parse(&self, lines: &[String]) -> Result<Self::ProblemModel, crate::solver::Error> {
        let diagnostic = lines
            .iter()
            .enumerate()
            .map(|(line_number, line)| {
                BinSeq::from_str(line).map_err(|e| solver::Error::WrongLine {
                    line_number,
                    line: line.into(),
                    description: e.to_string(),
                })
            })
            .collect::<Result<Vec<_>, solver::Error>>();
        diagnostic
    }
}

#[derive(Default)]
pub struct First {}

impl Solver for First {
    type ProblemModel = Vec<BinSeq>;
    type Solution = usize;

    fn solve(&self, model: Self::ProblemModel) -> Result<Self::Solution, solver::Error> {
        first_part(model).map_err(Into::into)
    }
}

impl Exercice for First {
    fn solve(&self, lines: &[String]) -> Result<String, crate::solver::Error> {
        let diagnostic = lines
            .iter()
            .enumerate()
            .map(|(line_number, line)| {
                BinSeq::from_str(line).map_err(|e| solver::Error::WrongLine {
                    line_number,
                    line: line.into(),
                    description: e.to_string(),
                })
            })
            .collect::<Result<Vec<_>, solver::Error>>()?;

        first_part(diagnostic)
            .map_err(Into::into)
            .map(|power_consumption| power_consumption.to_string())
    }
}

fn first_part(diagnostic: Vec<BinSeq>) -> Result<usize, solver::Error> {
    let most_common = diagnostic
        .into_iter()
        .fold(OccSeq::new(), |acc, number| acc.with_account(number))
        .as_most_common()?;

    let gamma = most_common.as_number();
    let epsilon = (!most_common).as_number();
    println!("{}", gamma);
    println!("{}", epsilon);

    Ok(gamma * epsilon)
}

#[derive(Default)]
pub struct Second {}

impl Solver for Second {
    type ProblemModel = Vec<BinSeq>;

    type Solution = usize;

    fn solve(&self, model: Self::ProblemModel) -> Result<Self::Solution, solver::Error> {
        let oxygen = most_common_in_matching(&model, MostCommon {}, vec![])?;
        let co2 = most_common_in_matching(&model, LeastCommon {}, vec![])?;
        Ok(oxygen.as_number() * co2.as_number())
    }
}

fn bool_to_value(b: bool) -> i64 {
    match b {
        true => 1,
        false => -1,
    }
}

trait BitCriteria {
    fn keep(&self, count: i64) -> bool;
}

struct MostCommon {}
impl BitCriteria for MostCommon {
    fn keep(&self, count: i64) -> bool {
        count >= 0
    }
}

struct LeastCommon {}
impl BitCriteria for LeastCommon {
    fn keep(&self, count: i64) -> bool {
        count < 0
    }
}

fn most_common_in_matching(
    diagnostic: &[BinSeq],
    criteria: impl BitCriteria,
    mut pattern: Vec<bool>,
) -> Result<&BinSeq, solver::Error> {
    let count = diagnostic.iter().filter(|b| b.matches(&pattern)).count();

    if count == 1 {
        return Ok(diagnostic.iter().find(|b| b.matches(&pattern)).unwrap());
    }
    if pattern.len() == SEQ_LEN {
        return Err(solver::Error::ExpectationUnfulfilled(
            "Duplicated binary sequence found".into(),
        ));
    }

    let signed_count: i64 = diagnostic
        .iter()
        .filter(|b| b.matches(&pattern))
        .inspect(|b| println!("{:?}", b))
        .map(|b| b.0[pattern.len()])
        .map(bool_to_value)
        .sum();

    pattern.push(criteria.keep(signed_count));

    most_common_in_matching(diagnostic, criteria, pattern)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_matches_correctly() {
        let inner = [
            true, true, false, false, true, true, false, false, true, true, false, false,
        ];
        let bin_seq = BinSeq(inner);
        let matcher = [true, true];
        let not_matcher = [false, true];
        assert!(bin_seq.matches(&matcher));
        assert!(!bin_seq.matches(&not_matcher));
    }
}
