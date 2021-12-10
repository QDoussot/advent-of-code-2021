use std::collections::{HashMap, HashSet};
use std::convert::Infallible;

use std::str::FromStr;

use derive_more::Display;
use derive_more::From;
use itertools::Itertools;

use std::fmt::Display;

use crate::application::{Application, FunctionExt};

use crate::application::Element;
use crate::bin_seq::BinSeq;
use crate::pop_array::PopArray;
use crate::solver::{ParsingError, Problem};

#[derive(Display, Debug)]
enum NoteParsingError {
    MissingSignals,
    NotAMiddlePipe(String),
    MissingDigits,
}

#[derive(From)]
struct Ens<T>(HashSet<T>);

impl<T: Display> Display for Ens<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("{")?;

        for el in self.0.iter() {
            f.write_str(&format!("{}, ", el))?;
        }
        f.write_str("}")?;
        writeln!(f, "")
    }
}

#[derive(PartialEq, Eq, From, Debug, Copy, Clone, Hash)]
pub struct Digit(BinSeq);

impl Digit {
    fn lighted_on_segments(&self) -> usize {
        self.0.card()
    }

    fn zero() -> Digit {
        Digit(BinSeq([false; 7]))
    }
}

impl From<HashSet<char>> for Digit {
    fn from(set: HashSet<char>) -> Self {
        let chars = ['a', 'b', 'c', 'd', 'e', 'f', 'g'];
        Digit(BinSeq(chars.map(|c| set.contains(&c))))
    }
}

impl Into<usize> for Digit {
    fn into(self) -> usize {
        digits().iter().position(|d| *d == self).unwrap()
    }
}

impl Element for Digit {}

impl Display for Digit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            &digits()
                .into_iter()
                .position(|x| x == *self)
                .map(|pos| pos.to_string())
                .unwrap_or("X".into()),
        )
    }
}

impl FromStr for Digit {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s.chars().collect::<HashSet<char>>()))
    }
}

impl From<String> for Digit {
    fn from(s: String) -> Self {
        Self::from_str(&s).unwrap()
    }
}

impl From<&str> for Digit {
    fn from(s: &str) -> Self {
        let chars = ['a', 'b', 'c', 'd', 'e', 'f', 'g'];
        Self(BinSeq(chars.map(|c| s.chars().into_iter().contains(&c))))
    }
}

pub fn digits() -> [Digit; 10] {
    let digits: [&str; 10] = [
        "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
    ];
    digits.map(ToString::to_string).map(Into::into)
}

struct Len {}

impl Element for usize {}
impl Application for Len {
    type In = Digit;

    type Out = usize;

    fn start(&self) -> HashSet<Self::In> {
        digits().into_iter().collect()
    }

    fn image(&self, e: &Self::In) -> Self::Out {
        e.0 .0.iter().filter(|bit| **bit).count()
    }
}

struct Discriminant {
    start: HashSet<Digit>,
    digit: Digit,
}

impl Application for Discriminant {
    type In = Digit;

    type Out = usize;

    fn start(&self) -> HashSet<Self::In> {
        self.start.iter().cloned().collect()
    }

    fn image(&self, e: &Self::In) -> Self::Out {
        (e.0 | self.digit.0).card()
    }
}

#[derive(Display, Debug)]
#[display(fmt = "signals: {:?}, digits: {:?}", signals, digits)]
struct Note {
    signals: [Digit; 10],
    digits: [Digit; 4],
}

impl FromStr for Note {
    type Err = NoteParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split(" ").into_iter();

        let signals = tokens
            .pop_array::<10>()
            .map_err(|_| NoteParsingError::MissingSignals)?
            .map(Into::into);

        match tokens.next() {
            Some("|") => (),
            Some(wrong) => return Err(NoteParsingError::NotAMiddlePipe(wrong.to_string())),
            _ => return Err(NoteParsingError::NotAMiddlePipe("".into())),
        };

        let digits = tokens
            .pop_array::<4>()
            .or(Err(NoteParsingError::MissingDigits))?
            .map(Into::into);

        Ok(Note { signals, digits })
    }
}

pub struct SevenSegmentSearch {
    notes: Vec<Note>,
}

fn build_plan() -> Vec<(Digit, Digit, usize)> {
    let mut known = HashSet::new();
    known.insert(Digit::zero());

    let mut plan: Vec<(Digit, Digit, _)> = vec![];
    let mut updated = true;
    while updated {
        let unknown: HashSet<_> = HashSet::from(digits())
            .difference(&known.iter().cloned().collect())
            .cloned()
            .collect();

        let mut founds = vec![];
        for el in &known {
            let discriminant = Discriminant {
                start: unknown.clone(),
                digit: *el,
            };
            let found = discriminant.ensemble_injectif();
            if !found.is_empty() {
                founds.extend(found.iter());
                for found_el in &found {
                    if !plan.iter().map(|step| &step.0).contains(found_el) {
                        plan.push((*found_el, *el, discriminant.image(found_el)))
                    }
                }
            }
        }
        updated = !founds.is_empty();
        known.extend(founds.iter())
    }

    plan
}

impl Problem for SevenSegmentSearch {
    fn parse(lines: &[String]) -> Result<Self, crate::solver::ParsingError> {
        let notes = lines
            .into_iter()
            .enumerate()
            .map(|(n, line)| {
                Note::from_str(&line).map_err(|e| ParsingError::IncorrectLine {
                    number: n,
                    line: line.clone(),
                    description: e.to_string(),
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(SevenSegmentSearch { notes })
    }

    fn part_one(&self) -> Result<usize, crate::solver::SolvingError> {
        let easy_numbers = |n: &usize| [2, 3, 4, 7].contains(n);
        let easy_count = self
            .notes
            .iter()
            .flat_map(|note| &note.digits)
            .map(|digit| digit.lighted_on_segments())
            .filter(easy_numbers)
            .count();
        Ok(easy_count)
    }

    fn part_two(&self) -> Result<usize, crate::solver::SolvingError> {
        let plan = build_plan();
        let mut the_sum = 0;
        for Note { signals, digits } in &self.notes {
            //
            let mut mapping = HashMap::new();
            mapping.insert(Digit::zero(), Digit::zero());
            let mut unknown = HashSet::from(*signals);
            for (to_found, disc, value) in &plan {
                let discriminant = Discriminant {
                    start: unknown.clone(),
                    digit: mapping.get(&disc).unwrap().clone(),
                };
                let ante: [Digit; 1] = discriminant.antecedent(&value).try_into().unwrap();
                //println!("Found {} to be {:?}", to_found, ante);
                unknown.remove(&ante[0]);
                mapping.insert(*to_found, ante[0].clone());
            }

            let mut number: usize = 0;
            for secret in digits {
                let ori = mapping.iter().find(|(k, v)| v == &secret).unwrap().0;
                let ori: usize = (*ori).into();
                number *= 10;
                number += ori;
            }
            the_sum += number;
        }
        Ok(the_sum)
    }
}

#[cfg(test)]
mod test {

    use std::collections::HashMap;

    use itertools::Itertools;

    use super::*;
    #[test]
    fn it_parses_note() {
        let note = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
        Note::from_str(note).unwrap();
    }

    #[test]
    fn it_finds_injective_ensemble() {
        let plan = build_plan();
        println!("Here is the plan ");
        for (new_found, disc, value) in &plan {
            println!("Find {} with disc({}) == {:?}", new_found, disc, value);
        }

        let note = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
        let note = Note::from_str(note).unwrap();

        let mut mapping = HashMap::new();
        mapping.insert(Digit::zero(), Digit::zero());
        let mut unknown = HashSet::from(note.signals);
        for (to_found, disc, value) in plan {
            let discriminant = Discriminant {
                start: unknown.clone(),
                digit: mapping.get(&disc).unwrap().clone(),
            };
            let ante: [Digit; 1] = discriminant.antecedent(&value).try_into().unwrap();
            println!("Found {} to be {:?}", to_found, ante);
            unknown.remove(&ante[0]);
            mapping.insert(to_found, ante[0].clone());
        }

        let mut expected: HashMap<_, _> = [
            (8, "acedgfb"),
            (5, "cdfbe"),
            (2, "gcdfa"),
            (3, "fbcad"),
            (7, "dab"),
            (9, "cefabd"),
            (6, "cdfgeb"),
            (4, "eafb"),
            (0, "cagedb"),
            (1, "ab"),
        ]
        .map(|(d, encoding)| (digits()[d].clone(), Digit::from(encoding)))
        .into();
        expected.insert(Digit::zero(), Digit::zero());

        for secret in note.digits {
            let ori = mapping.iter().find(|(k, v)| v == &&secret).unwrap().0;
            println!("{}", ori);
        }
    }
}
