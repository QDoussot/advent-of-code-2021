use crate::solver::{Error, Parse, Solver};
use std::{num::ParseIntError, ops::Deref, str::FromStr};

use itertools::Itertools;

#[derive(Default)]
pub struct CrabCrew(Vec<usize>);

impl CrabCrew {}

impl Parse for CrabCrew {
    type ProblemModel = CrabCrew;

    fn parse(&self, lines: &[String]) -> Result<Self::ProblemModel, crate::solver::Error> {
        let positions = lines
            .first()
            .ok_or(Error::EmptyFile)?
            .split(",")
            .into_iter()
            .map(AsRef::as_ref)
            .map(usize::from_str)
            .collect::<Result<Vec<usize>, ParseIntError>>()
            .map_err(|_| Error::WrongLine {
                description: "Wrong interger".into(),
                line_number: 0,
                line: lines[0].clone(),
            })?;
        let max = positions.iter().max().ok_or(Error::Unexpected)?;

        let mut crabs_count_by_pos = vec![0; max + 1];

        for pos in positions {
            crabs_count_by_pos[pos] += 1;
        }

        Ok(CrabCrew(crabs_count_by_pos))
    }
}

fn accumulated(vec: &[usize]) -> Vec<usize> {
    //vec.iter().fold(vec![], |mut acc, occ| {
    //    acc.push(acc.last().map(|v| *v).unwrap_or(0) + occ);
    //    acc
    //})

    let mut accumulated: Vec<usize> = vec.iter().cloned().collect();
    let mut acc = 0;
    for v in accumulated.iter_mut() {
        *v += acc;
        acc = *v;
    }
    accumulated
}

fn median(vec: &[usize]) -> Option<(usize, usize)> {
    let accumulated = accumulated(vec);

    accumulated
        .last()
        .and_then(|total| accumulated.iter().find_position(|count| **count > total / 2))
        .map(|(pos, count)| (pos, *count))
}
//fn abs_diff(x:usize, y:usize)
fn manhantan_dist_vec(vec_size: usize, pos: usize) -> Vec<usize> {
    (0..vec_size).map(|x| x.abs_diff(pos)).collect()
}

fn quadra_cost(dist: usize) -> usize {
    dist * (dist + 1) / 2
}

fn vec_cost(vec_size: usize, pos: usize) -> Vec<usize> {
    (0..vec_size).map(|x| quadra_cost(x.abs_diff(pos))).collect()
}

#[derive(Default)]
pub struct First {}

impl Solver for First {
    type ProblemModel = CrabCrew;

    type Solution = usize;

    fn solve(&self, model: Self::ProblemModel) -> Result<Self::Solution, Error> {
        let accumulated = accumulated(&model.0);

        let total = accumulated.last().unwrap();
        let median = median(&model.0).unwrap();
        let score = model
            .0
            .iter()
            .zip(manhantan_dist_vec(model.0.len(), median.0).into_iter())
            .map(|(count, dist)| count * dist)
            .sum();

        Ok(score)
    }
}

#[derive(Default)]
pub struct Second {}

impl Solver for Second {
    type ProblemModel = CrabCrew;

    type Solution = usize;

    fn solve(&self, model: Self::ProblemModel) -> Result<Self::Solution, Error> {
        let count: usize = model.0.len();

        let costs: Vec<usize> = (0..count)
            .map(|pos| {
                let vec_costs = vec_cost(count, pos)
                    .into_iter()
                    .zip(model.0.iter())
                    .map(|(x, y)| x * y)
                    .sum();
                //println!("{}", vec_costs);
                vec_costs
            })
            .collect();

        let min_cost = costs.into_iter().min();

        min_cost.ok_or(Error::Unexpected)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_accumulates() {
        assert_eq!(accumulated(&[1, 1, 1, 1]), &[1, 2, 3, 4]);
        assert_eq!(accumulated(&[1, 2, 4, 8]), &[1, 3, 7, 15]);
    }

    #[test]
    fn it_computes_median() {
        assert_eq!(median(&[1, 1, 4, 1]), Some((2, 6)));
        assert_eq!(median(&[1, 1, 1, 1]), Some((2, 3)));
        assert_eq!(median(&[1, 1, 0, 2, 5, 6, 3, 1, 1]), Some((5, 15)));
    }
}
