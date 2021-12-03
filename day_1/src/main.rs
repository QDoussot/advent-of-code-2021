use std::collections::LinkedList;
use std::io::{self, BufRead, BufReader};
use std::str::FromStr;

#[allow(dead_code)]
fn number_of_increase(mut depth_list: impl Iterator<Item = usize>) -> usize {
    let last_depth = depth_list.next();
    let mut number = 0;
    if let Some(mut last_depth) = last_depth {
        for depth in depth_list {
            if depth > last_depth {
                number += 1;
            }
            last_depth = depth;
        }
    }
    number
}

#[allow(dead_code)]
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

#[allow(dead_code)]
struct WindowIter<'a, I, It>
where
    It: Iterator<Item = &'a I>,
{
    size: usize,
    window: LinkedList<&'a I>,
    iter: It,
}

#[allow(dead_code)]
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
    WindowIter { size, iter, window }
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

fn main() -> Result<(), String> {
    let stdin = BufReader::new(io::stdin());
    let lines = stdin.lines().collect::<Result<Vec<_>, io::Error>>().unwrap();

    let depths = lines
        .into_iter()
        .map(|line| usize::from_str(&line))
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    println!("{}", number_of_increase_bis(depths.into_iter()));

    let vec = (1..10).collect::<Vec<usize>>();
    let iter = window_iter(vec.iter(), 3);
    for win in iter {
        println!("{:?}", win);

        println!("{:?}", win.into_iter().fold(0, |acc, x| acc + x));
    }

    Ok(())
}

#[cfg(test)]
mod test {}
