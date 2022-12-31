#![feature(test)]
extern crate test;

use std::collections::HashSet;
use std::hash::Hash;

fn parse_input(input: &str) -> Vec<char> {
    input.lines().next().expect("EMPTY FILE").chars().collect()
}

fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}

fn find_unique_index(input: &str, n: usize) -> usize {
    let buffer = parse_input(input);

    let mut window = buffer
        .windows(n)
        .zip(n..)
        .filter(|(w, _)| has_unique_elements(*w));

    let (_, index) = window.next().expect("Failed to find unique window");

    index
}
fn part_one(input: &str) -> usize {
    find_unique_index(input, 4)
}

fn part_two(input: &str) -> usize {
    find_unique_index(input, 14)
}

utils::aoc_problem!(day_06, part_one, 7, 1538, part_two, 19, 2315);

#[cfg(test)]
mod day_06_additional {}
