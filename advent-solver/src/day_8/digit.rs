use std::{collections::HashSet, convert::Infallible, fmt::Display, str::FromStr};

use crate::{application::Element, bin_seq::BinSeq};
use derive_more::From;
use itertools::Itertools;

#[derive(PartialEq, Eq, From, Debug, Copy, Clone, Hash)]
pub struct Digit(pub BinSeq);

impl Digit {
    pub fn lighted_on_segments(&self) -> usize {
        self.0.card()
    }

    pub fn off() -> Digit {
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
