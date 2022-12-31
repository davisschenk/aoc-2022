#![feature(test)]
extern crate test;

#[derive(Debug)]
struct Tree {
    value: i32,
    visible: bool,
}

#[derive(Debug)]
struct Trees(Vec<Vec<Tree>>);

#[derive(Debug)]
enum Direction {
    Top,
    Left,
    Right,
    Bottom,
}

impl Direction {
    fn iter() -> [Self; 4] {
        // TODO: figure out how to iterate an enum
        [
            Direction::Top,
            Direction::Bottom,
            Direction::Left,
            Direction::Right,
        ]
    }
}

impl Trees {
    fn new(input: &str) -> Self {
        let trees: Vec<Vec<Tree>> = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|s| s.to_digit(10).expect("Bad input") as i32)
                    .map(|n| Tree {
                        value: n,
                        visible: false,
                    })
                    .collect()
            })
            .collect();

        Self(trees)
    }

    fn find_visible(&mut self) {
        let count = self.0.len();

        for dir in Direction::iter() {
            for xr in 0..count {
                let mut max: i32 = -1;

                for yr in 0..count {
                    let (x, y) = match dir {
                        Direction::Top => (xr, yr),
                        Direction::Bottom => (xr, count - yr - 1),
                        Direction::Left => (yr, xr),
                        Direction::Right => (count - yr - 1, xr),
                    };

                    let mut tree = &mut self.0[y][x];

                    if max < tree.value {
                        tree.visible = true;
                        max = tree.value;
                    }
                }
            }
        }
    }

    fn count_visible(&self) -> u64 {
        let mut count = 0;

        for i in &self.0 {
            for j in i {
                if j.visible {
                    count += 1;
                }

                eprint!("{}", j.visible as u8);
            }
            eprintln!();
        }

        count
    }

    fn calculate_scenic_score(&self) -> Vec<Vec<u64>> {
        let mut scores = Vec::new();
        let size = self.0.len();

        for y in 0..size {
            scores.push(Vec::new());
            for x in 0..size {
                let value = &self.0[y][x];
                let mut scenic_score = 1;

                for dir in Direction::iter() {
                    let (x_offset, y_offset): (i64, i64) = match dir {
                        Direction::Top => (0, 1),
                        Direction::Bottom => (0, -1),
                        Direction::Left => (-1, 0),
                        Direction::Right => (1, 0),
                    };
                    let mut viewing_distance = 0;

                    for i in 1..size {
                        let cx = x as i64 + (i as i64 * x_offset);
                        let cy = y as i64 + (i as i64 * y_offset);

                        if cx < 0 || cy < 0 || cx >= size as i64 || cy >= size as i64 {
                            break;
                        }

                        let tree = &self.0[cy as usize][cx as usize];

                        viewing_distance += 1;
                        if tree.value >= value.value {
                            break;
                        }
                    }

                    scenic_score *= viewing_distance;
                }

                scores[y].push(scenic_score);
            }
        }
        scores
    }
}

fn part_one(input: &str) -> u64 {
    let mut trees = Trees::new(input);
    trees.find_visible();

    trees.count_visible()
}

fn part_two(input: &str) -> u64 {
    let trees = Trees::new(input);
    *trees
        .calculate_scenic_score()
        .iter()
        .map(|v| v.iter().max().unwrap())
        .max()
        .unwrap()
}

utils::aoc_problem!(day_08, part_one, 21, 1776, part_two, 8, 234416);

#[cfg(test)]
mod day_08_additional {}
