use itertools::Itertools;

use crate::solver::{Error, Parse, Solver};
use std::str::FromStr;

#[derive(Default, Debug)]
pub struct LanterfishCrew {
    short_cycles: [usize; 7],
    long_cycles: [usize; 9],
}

impl LanterfishCrew {
    fn size(&self) -> usize {
        self.short_cycles.iter().sum::<usize>() + self.long_cycles.iter().sum::<usize>()
    }
}

impl Iterator for LanterfishCrew {
    type Item = LanterfishCrew;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next_short_cycles = [0; 7];
        //Cycling through old age
        for i in 0..=6 {
            next_short_cycles[i] = self.short_cycles[(i + 1) % 7];
        }

        //youngs become adults
        next_short_cycles[6] += self.long_cycles[0];

        let mut next_long_cycles = [0; 9];

        //Youngs become older
        for i in 0..=7 {
            next_long_cycles[i] = self.long_cycles[i + 1];
        }
        // baby fishes
        next_long_cycles[8] = self.short_cycles[0] + self.long_cycles[0];

        self.short_cycles = next_short_cycles;
        self.long_cycles = next_long_cycles;
        Some(LanterfishCrew {
            short_cycles: next_short_cycles,
            long_cycles: next_long_cycles,
        })
    }
}

impl Parse for LanterfishCrew {
    type ProblemModel = LanterfishCrew;

    fn parse(&self, lines: &[String]) -> Result<Self::ProblemModel, crate::solver::Error> {
        let crew_cyle = lines
            .first()
            .ok_or(Error::EmptyFile)?
            .split(",")
            .into_iter()
            .map(AsRef::as_ref)
            .map(usize::from_str)
            .fold_ok([0; 16], |mut acc, v| {
                acc[v] += 1;
                acc
            })
            .map_err(|e| Error::WrongLine {
                description: e.to_string(),
                line_number: 0,
                line: lines[0].clone(),
            })?;
        println!("{:?}", crew_cyle);
        let short_cycles: [usize; 7] = crew_cyle[0..7].try_into().unwrap();
        let long_cycles: [usize; 9] = crew_cyle[7..(7 + 9)].try_into().unwrap();
        if long_cycles.iter().any(|c| *c > 0) {
            return Err(crate::solver::Error::ExpectationUnfulfilled(
                "Found unexpected young fish in first generation".into(),
            ));
        }
        Ok(LanterfishCrew {
            short_cycles,
            long_cycles,
        })
    }
}

impl Solver for LanterfishCrew {
    type ProblemModel = LanterfishCrew;

    type Solution = usize;

    fn solve(&self, mut model: Self::ProblemModel) -> Result<Self::Solution, Error> {
        for _i in 1..=80 {
            model.next();
        }
        Ok(model.size())
    }
}

#[derive(Default)]
pub struct Second {}

impl Solver for Second {
    type ProblemModel = LanterfishCrew;

    type Solution = usize;

    fn solve(&self, mut model: Self::ProblemModel) -> Result<Self::Solution, Error> {
        for _i in 1..=256 {
            model.next();
        }
        Ok(model.size())
    }
}

#[cfg(test)]
mod test {}
