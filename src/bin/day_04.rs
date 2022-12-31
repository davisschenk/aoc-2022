use core::ops::Range;
use std::cmp::{max, min};

type Pair = (Range<u64>, Range<u64>);

fn split_at_char(input: &str, character: char) -> (&str, &str) {
    let idx = input.find(character).unwrap();

    (&input[..idx], &input[idx + 1..])
}

fn parse_line(input: &str) -> Pair {
    let (first, last) = split_at_char(input, ',');
    let (a, b) = split_at_char(first, '-');
    let (c, d) = split_at_char(last, '-');

    (
        a.parse::<u64>().unwrap()..b.parse::<u64>().unwrap(),
        c.parse::<u64>().unwrap()..d.parse::<u64>().unwrap(),
    )
}

fn parse_input(input: &str) -> Vec<Pair> {
    input.lines().map(parse_line).collect()
}

fn part_one(input: &str) -> u64 {
    parse_input(input)
        .iter()
        .filter(|(a, b)| {
            ((a.start <= b.start) && (a.end >= b.end)) || ((b.start <= a.start) && (b.end >= a.end))
        })
        .count() as u64
}

fn part_two(input: &str) -> u64 {
    parse_input(input)
        .iter()
        .filter(|(a, b)| max(a.start, b.start) <= min(a.end, b.end))
        .count() as u64
}

utils::aoc_problem!(day_04, part_one, 2, 450, part_two, 4, 837);

#[cfg(test)]
mod day_04_additional {
    #[test]
    fn test_split_at_char() {
        let (a, b) = crate::split_at_char("a:b", ':');

        assert_eq!(a, "a");
        assert_eq!(b, "b");
    }
}
