#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(i64),
}

impl Instruction {
    #[allow(dead_code)]
    fn clock_cycles(&self) -> usize {
        match self {
            Self::Noop => 1,
            Self::Addx(_) => 2,
        }
    }
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        let parts: Vec<&str> = value.split_whitespace().collect();

        match parts[..] {
            ["noop"] => Self::Noop,
            ["addx", x] => Self::Addx(x.parse().expect("Bad adding value")),
            _ => panic!("Couldnt parse instruction"),
        }
    }
}

fn read_instructions(input: &str) -> Vec<Instruction> {
    input.lines().map(Instruction::from).collect()
}

fn part_one(input: &str) -> i64 {
    let instructions = read_instructions(input);
    let mut values = vec![];
    let mut register = 1;

    for instruction in instructions {
        match instruction {
            Instruction::Addx(x) => {
                values.push(register);
                register += x;
                values.push(register);
            }
            Instruction::Noop => {
                values.push(register);
            }
        }
    }

    [20usize, 60, 100, 140, 180, 220]
        .into_iter()
        .map(|i| values[i - 2] * i as i64)
        .sum()
}

fn part_two(_input: &str) -> &str {
    unimplemented!("Day 10: Part Two")
}

// const PART_TWO_EXAMPLE: &'static str = "
// ##..##..##..##..##..##..##..##..##..##..
// ###...###...###...###...###...###...###.
// ####....####....####....####....####....
// #####.....#####.....#####.....#####.....
// ######......######......######......####
// #######.......#######.......#######.....";

// const PART_TWO: &'static str = "";

utils::aoc_problem!(day_10, part_one, 13140, 11960, part_two, "", "");

#[cfg(test)]
mod day_10_additional {}
