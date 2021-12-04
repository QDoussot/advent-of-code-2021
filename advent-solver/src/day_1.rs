use std::collections::LinkedList;

use std::str::FromStr;

use crate::Solver;

fn number_of_increase_bis(value_list: impl Iterator<Item = usize>) -> usize {
    let mut last_value = None;
    let mut number = 0;
    for value in value_list {
        if let Some(true) = last_value.map(|last| last < value) {
            number += 1
        }
        last_value = Some(value)
    }
    number
}

struct WindowIter<'a, I, It>
where
    It: Iterator<Item = &'a I>,
{
    window: LinkedList<&'a I>,
    iter: It,
}

fn window_iter<'a, I, It>(mut iter: It, size: usize) -> WindowIter<'a, I, It>
where
    It: Iterator<Item = &'a I>,
{
    let mut window = LinkedList::new();
    for _ in 1..size {
        if let Some(value) = iter.next() {
            window.push_back(value)
        } else {
            break;
        }
    }

    if let Some(discarded) = window.front().copied() {
        window.push_front(discarded);
    }
    WindowIter { iter, window }
}

impl<'a, I, It> Iterator for WindowIter<'a, I, It>
where
    It: Iterator<Item = &'a I>,
{
    type Item = LinkedList<&'a I>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.iter.next() {
            self.window.pop_front();
            self.window.push_back(item);
            Some(self.window.clone())
        } else {
            None
        }
    }
}

#[derive(Default)]
pub struct First {}

impl First {
    pub fn new() -> Box<Self> {
        Box::new(Self {})
    }
}

impl Solver for First {
    fn solve(&self, lines: &[String]) -> Result<String, crate::solver::Error> {
        let depths = lines
            .into_iter()
            .map(|line| usize::from_str(&line))
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        Ok(part_1(depths).to_string())
    }
}

#[derive(Default)]
pub struct Second {}

impl Solver for Second {
    fn solve(&self, lines: &[String]) -> Result<String, crate::solver::Error> {
        let depths = lines
            .into_iter()
            .map(|line| usize::from_str(&line))
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        Ok(part_2(depths).to_string())
    }
}

pub fn part_1(depths: Vec<usize>) -> usize {
    number_of_increase_bis(depths.into_iter())
}

pub fn part_2(depths: Vec<usize>) -> usize {
    let windows = window_iter(depths.iter(), 3);
    let sums = windows
        .map(|win| win.into_iter().fold(0, |acc, x| acc + x))
        .collect::<Vec<_>>();
    number_of_increase_bis(sums.into_iter())
}
