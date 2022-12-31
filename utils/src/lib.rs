#![feature(test)]
extern crate test;

#[macro_export]
macro_rules! aoc_problem {
    ($day:ident, $part_one:ident, $part_one_example_answer:literal, $part_one_answer:literal, $part_two:ident, $part_two_example_answer:literal, $part_two_answer:literal) => {
        const INPUT: &str = include_str!(concat!("inputs/", stringify!($day)));
        const EXAMPLE_INPUT: &str = include_str!(concat!("inputs/", stringify!($day), "_example"));

        fn main() {
            println!("---  {} ---", stringify!($day));
            println!("Part One: {:?}", $part_one(crate::INPUT));
            println!("Part Two: {:?}", $part_two(crate::INPUT));
        }

        #[cfg(test)]
        mod $day {
            use test::{Bencher, black_box};
            use super::*;

            #[test]
            fn test_example_part_one() {
                let result = crate::$part_one(crate::EXAMPLE_INPUT);
                assert_eq!(result, $part_one_example_answer);
            }

            #[test]
            fn test_example_part_two() {
                let result = crate::$part_two(crate::EXAMPLE_INPUT);
                assert_eq!(result, $part_two_example_answer);
            }

            #[test]
            fn test_part_one() {
                let result = crate::$part_one(crate::INPUT);
                assert_eq!(result, $part_one_answer);
            }

            #[test]
            fn test_part_two() {
                let result = crate::$part_two(crate::INPUT);
                assert_eq!(result, $part_two_answer);
            }

            #[bench]
            fn bench_part_one(b: &mut Bencher) {
                b.iter(|| crate::$part_one(black_box(crate::INPUT)));
            }

            #[bench]
            fn bench_part_two(b: &mut Bencher) {
                b.iter(|| crate::$part_two(black_box(crate::INPUT)));
            }
        }
    }
}
