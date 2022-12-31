#![feature(test)]
#![feature(exclusive_range_pattern)]
extern crate test;

#[derive(Debug, PartialEq)]
struct Rucksack<'a> {
    one: &'a str,
    two: &'a str,
}

impl<'a> Rucksack<'a> {
    fn from_string(inp: &'a str) -> Self {
        let index: usize = inp.len() / 2;
        let (one, two) = inp.split_at(index);
        Self { one, two }
    }
}

fn priority(c: char) -> u64 {
    match c {
        'a'..='z' => (c as u64) - 96,
        'A'..='Z' => (c as u64) - 38,
        _ => panic!("unexpected item in rucksack"),
    }
}

fn common_chars_two(a: &str, b: &str) -> Vec<char> {
    a.chars().filter(|c| b.contains(*c)).collect()
}

fn common_chars_three(a: &str, b: &str, c: &str) -> Vec<char> {
    a.chars()
        .filter(|d| b.contains(*d) & c.contains(*d))
        .collect()
}

fn parse_input(input: &str) -> Vec<Rucksack> {
    input.lines().map(Rucksack::from_string).collect()
}

fn part_one(input: &str) -> u64 {
    let sacks = parse_input(input);
    sacks
        .iter()
        .map(|s| common_chars_two(s.one, s.two))
        .map(|c| priority(c[0]))
        .sum::<u64>()
}

fn part_two(input: &str) -> u64 {
    input
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|c| common_chars_three(c[0], c[1], c[2]))
        .map(|c| priority(c[0]))
        .sum()
}

utils::aoc_problem!(day_03, part_one, 157, 8105, part_two, 70, 2363);

#[cfg(test)]
mod day_03_additional {
    #[test]
    fn test_priorities() {
        for (c, p) in ('a'..='z').zip(1..=26) {
            assert_eq!(p, crate::priority(c))
        }

        for (c, p) in ('A'..='Z').zip(27..=52) {
            assert_eq!(p, crate::priority(c))
        }
    }

    #[test]
    fn test_rucksack_construction() {
        let contents = "abcdefgh";
        let rucksack = crate::Rucksack::from_string(contents);

        assert_eq!(
            rucksack,
            crate::Rucksack {
                one: "abcd",
                two: "efgh"
            }
        );
    }
}
