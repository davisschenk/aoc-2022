#![feature(test)]
#![feature(assert_matches)]
extern crate test;

use std::collections::HashSet;

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'U' => Self::Up,
            'D' => Self::Down,
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!("Not a direction"),
        }
    }
}

impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        Self::from(value.chars().next().unwrap())
    }
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    distance: i64,
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        let parts = value.split_whitespace().collect::<Vec<&str>>();

        let (direction, distance) = match parts[..] {
            [a, b] => (Direction::from(a), b.parse().unwrap()),
            _ => panic!("Not an instruction"),
        };

        Self {
            direction,
            distance,
        }
    }
}

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
struct RopeEnd {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct Board {
    knots: Vec<RopeEnd>,
    tail_positions: HashSet<RopeEnd>,
}

impl Board {
    fn new(length: usize) -> Self {
        let mut hs = HashSet::new();
        hs.insert(RopeEnd { x: 0, y: 0 });

        let mut knots = Vec::new();
        for _ in 0..length {
            knots.push(RopeEnd { x: 0, y: 0 });
        }

        Self {
            knots,
            tail_positions: hs,
        }
    }

    fn move_rope(&mut self, instruction: Instruction) {
        for _ in 0..instruction.distance {
            match instruction.direction {
                Direction::Up => self.knots[0].y += 1,
                Direction::Down => self.knots[0].y -= 1,
                Direction::Left => self.knots[0].x -= 1,
                Direction::Right => self.knots[0].x += 1,
            }

            for knot in 1..self.knots.len() {
                let dx = self.knots[knot - 1].x - self.knots[knot].x;
                let dy = self.knots[knot - 1].y - self.knots[knot].y;

                if dx.abs() > 1 || dy.abs() > 1 {
                    self.knots[knot].x += dx.signum();
                    self.knots[knot].y += dy.signum();
                }
            }

            self.tail_positions
                .insert(self.knots[self.knots.len() - 1].clone());
        }
    }
}

fn get_instructions(input: &str) -> Vec<Instruction> {
    input.lines().map(Instruction::from).collect()
}

fn simulate_rope(input: &str, length: usize) -> usize {
    let instructions = get_instructions(input);
    let mut board = Board::new(length);

    for instruction in instructions {
        board.move_rope(instruction);
    }

    board.tail_positions.len()
}

fn part_one(input: &str) -> usize {
    simulate_rope(input, 2)
}

fn part_two(input: &str) -> usize {
    simulate_rope(input, 10)
}

utils::aoc_problem!(day_09, part_one, 13, 6081, part_two, 1, 2487);

#[cfg(test)]
mod day_09_additional {
    use super::*;
    use std::assert_matches::assert_matches;

    #[test]
    fn test_direction_from_char() {
        assert_matches!(Direction::from('U'), Direction::Up);
        assert_matches!(Direction::from('D'), Direction::Down);
        assert_matches!(Direction::from('R'), Direction::Right);
        assert_matches!(Direction::from('L'), Direction::Left);
    }

    #[test]
    fn test_direction_from_str() {
        assert_matches!(Direction::from("U"), Direction::Up);
        assert_matches!(Direction::from("D"), Direction::Down);
        assert_matches!(Direction::from("R"), Direction::Right);
        assert_matches!(Direction::from("L"), Direction::Left);
    }

    #[test]
    fn test_instruction_from_str() {
        assert_matches!(
            Instruction::from("U 1"),
            Instruction {
                direction: Direction::Up,
                distance: 1
            }
        );
        assert_matches!(
            Instruction::from("R 100"),
            Instruction {
                direction: Direction::Right,
                distance: 100
            }
        );
        assert_matches!(
            Instruction::from("L 10"),
            Instruction {
                direction: Direction::Left,
                distance: 10
            }
        );
        assert_matches!(
            Instruction::from("L        10"),
            Instruction {
                direction: Direction::Left,
                distance: 10
            }
        );
    }

    // fn build_board(head: RopeEnd, tail: RopeEnd) -> Board {
    //     Board {
    //         tail,
    //         head,
    //         tail_positions: HashSet::new(),
    //     }
    // }

    // #[test]
    // fn test_is_touching() {
    //     for x in [-1, 0, 1] {
    //         for y in [-1, 0, 1] {
    //             assert!(
    //                 build_board((x, y), (0, 0)).is_touching(),
    //                 "Head: ({}, {}) Tail: (0, 0)",
    //                 x,
    //                 y
    //             );
    //             assert!(
    //                 build_board((0, 0), (x, y)).is_touching(),
    //                 "Head: (0, 0) Tail: ({}, {})",
    //                 x,
    //                 y
    //             );
    //         }
    //     }
    // }

    // #[test]
    // fn test_is_not_touching() {
    //     assert!(!(build_board((0, 0), (0, 2)).is_touching()));
    //     assert!(!(build_board((2, 0), (0, 2)).is_touching()));
    //     assert!(!(build_board((0, 5), (0, -5)).is_touching()));
    //     assert!(!(build_board((5, 5), (-5, -5)).is_touching()));
    // }

    // #[test]
    // fn test_inline() {
    //     assert!(build_board((0, 0), (0, 2)).inline());
    //     assert!(build_board((0, 0), (0, -2)).inline());
    //     assert!(build_board((0, 0), (2, 0)).inline());
    //     assert!(build_board((0, 0), (-2, 0)).inline());
    //     assert!(!(build_board((0, 0), (0, 0)).inline()));
    // }
}
