mod digit;
use digit::Digit;

use std::collections::{HashMap, HashSet};
use std::convert::Infallible;

use std::str::FromStr;

use derive_more::Display;
use derive_more::From;
use itertools::Itertools;

use std::fmt::Display;

use crate::application::{Application, ApplicationExt};

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

// #[derive(From)]
// struct Ens<T>(HashSet<T>);
//
// impl<T: Display> Display for Ens<T> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.write_str("{")?;
//
//         for el in self.0.iter() {
//             f.write_str(&format!("{}, ", el))?;
//         }
//         f.write_str("}")?;
//         writeln!(f, "")
//     }
// }

struct Len {}

impl Element for usize {}
impl Application for Len {
    type In = Digit;

    type Out = usize;

    fn start(&self) -> HashSet<Self::In> {
        digit::digits().into_iter().collect()
    }

    fn image(&self, e: &Self::In) -> Self::Out {
        e.lighted_on_segments()
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

struct Identifier {
    identified: Digit,
    disc: Discriminant,
    expected_value: usize,
}

impl Element for bool {}
impl Application for Identifier {
    type In = Digit;

    type Out = bool;

    fn start(&self) -> HashSet<Self::In> {
        self.disc.start()
    }

    fn image(&self, e: &Self::In) -> Self::Out {
        self.disc.image(e) == self.expected_value
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

impl Note {
    fn decode(&self, mapping: &HashMap<Digit, Digit>) -> usize {
        let mut number = 0;
        for secret in self.digits {
            let ori = mapping.iter().find(|(k, v)| **v == secret).unwrap().0;
            let ori: usize = (*ori).into();
            number *= 10;
            number += ori;
        }
        number
    }
}

pub struct SevenSegmentSearch {
    notes: Vec<Note>,
}

fn build_plan() -> Vec<(Digit, Digit, usize)> {
    let mut known = HashSet::new();
    known.insert(Digit::off());

    let mut plan: Vec<(Digit, Digit, _)> = vec![];
    let mut updated = true;
    while updated {
        let unknown: HashSet<_> = HashSet::from(digit::digits())
            .difference(&known.iter().cloned().collect())
            .cloned()
            .collect();

        let mut founds = vec![];
        for el in &known {
            let discriminant = Discriminant {
                start: unknown.clone(),
                digit: *el,
            };
            let discriminable_list = discriminant.ensemble_injectif();
            if !discriminable_list.is_empty() {
                founds.extend(discriminable_list.iter());
                for discriminable in &discriminable_list {
                    if !plan.iter().map(|step| &step.0).contains(discriminable) {
                        plan.push((*discriminable, *el, discriminant.image(discriminable)))
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
        for note in &self.notes {
            //
            let mut mapping = HashMap::new();
            mapping.insert(Digit::off(), Digit::off());
            let mut unknown = HashSet::from(note.signals);

            for (to_found, disc, value) in &plan {
                let discriminant = Discriminant {
                    start: unknown.clone(),
                    digit: mapping.get(&disc).unwrap().clone(),
                };
                let ante: [Digit; 1] = discriminant.antecedent(&value).try_into().unwrap();

                unknown.remove(&ante[0]);
                mapping.insert(*to_found, ante[0].clone());
            }

            the_sum += note.decode(&mapping);
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
        mapping.insert(Digit::off(), Digit::off());
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
        .map(|(d, encoding)| (digit::digits()[d].clone(), Digit::from(encoding)))
        .into();
        expected.insert(Digit::off(), Digit::off());

        for secret in note.digits {
            let ori = mapping.iter().find(|(k, v)| v == &&secret).unwrap().0;
            println!("{}", ori);
        }
    }
}
