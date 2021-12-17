use crate::solver::SolvingError;
use crate::solver::{ParsingError, Problem};
use regex::Regex;
use std::str::FromStr;

#[derive(Debug)]
pub struct ProbeSystem {
    target_min_x: usize,
    target_max_x: usize,
    target_min_y: i64,
    target_max_y: i64,
}

impl ProbeSystem {
    fn is_in(&self, x: usize, y: i64) -> bool {
        self.target_max_x >= x && x >= self.target_min_x && self.target_max_y >= y && y >= self.target_min_y
    }

    fn hits(&self, mut vx: usize, mut vy: i64) -> bool {
        let (mut x, mut y) = (0, 0i64);
        while x <= self.target_max_x && y >= self.target_min_y {
            if self.is_in(x, y) {
                return true;
            }
            x += vx;
            y += vy;
            if vx > 0 {
                vx -= 1;
            }
            vy -= 1;
        }
        return false;
    }
}

impl Problem for ProbeSystem {
    // target area: x=117..164, y=-140..-89
    fn parse(lines: &[String]) -> Result<Self, ParsingError> {
        let re = Regex::new(r"target area: x=(\d+)\.\.(\d+), y=(-\d+)\.\.(-\d+)").unwrap();
        let tokens = re.captures(lines.first().unwrap()).unwrap();
        Ok(ProbeSystem {
            target_min_x: usize::from_str(&tokens[1]).unwrap(),
            target_max_x: usize::from_str(&tokens[2]).unwrap(),
            target_min_y: i64::from_str(&tokens[3]).unwrap(),
            target_max_y: i64::from_str(&tokens[4]).unwrap(),
        })
    }

    fn part_one(&self) -> Result<usize, SolvingError> {
        let max_velocity = self.target_min_y.abs() as usize;
        let max_high = max_velocity * (max_velocity + 1) / 2;
        Ok(max_high - self.target_min_y.abs() as usize)
    }

    fn part_two(&self) -> Result<usize, SolvingError> {
        let min_speed_x = (2.0 * self.target_min_x as f64 + 0.5 * 0.5).sqrt() - 0.5;
        let min_speed_x = min_speed_x.ceil() as usize;
        let max_speed_x = self.target_max_x;

        let min_speed_y = self.target_min_y;
        let max_speed_y = -self.target_min_y;

        let mut count = 0;
        for vx in min_speed_x..=max_speed_x {
            for vy in min_speed_y..=max_speed_y {
                if self.hits(vx, vy) {
                    count += 1;
                }
            }
        }
        Ok(count)
    }
}
