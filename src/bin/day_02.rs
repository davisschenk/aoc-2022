// A X- Rock
// B Y - Paper
// C Z- Scissors
//
#[derive(Debug, PartialEq, Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn score(&self) -> u64 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

impl From<&str> for Move {
    fn from(c: &str) -> Self {
        match c {
            "A" => Move::Rock,
            "X" => Move::Rock,
            "B" => Move::Paper,
            "Y" => Move::Paper,
            "C" => Move::Scissors,
            "Z" => Move::Scissors,
            _ => panic!("Bad move"),
        }
    }
}

fn process_input(input: &str) -> Vec<(Move, Move)> {
    input
        .lines()
        .map(|l| {
            let whitespace: Vec<&str> = l.split_whitespace().collect();

            if let [a, b] = whitespace[..] {
                (Move::from(a), Move::from(b))
            } else {
                panic!("Unexpected input")
            }
        })
        .collect()
}

#[derive(Debug)]
enum Outcome {
    Lose,
    Win,
    Draw,
}

impl Outcome {
    fn score(&self) -> u64 {
        match self {
            Self::Win => 6,
            Self::Draw => 3,
            Self::Lose => 0,
        }
    }
}

impl From<Move> for Outcome {
    fn from(m: Move) -> Self {
        match m {
            Move::Rock => Self::Lose,
            Move::Paper => Self::Draw,
            Move::Scissors => Self::Win,
        }
    }
}

fn calculate_outcome(opponent: Move, me: Move) -> Outcome {
    match (opponent, me) {
        (Move::Rock, Move::Paper) => Outcome::Win,
        (Move::Paper, Move::Scissors) => Outcome::Win,
        (Move::Scissors, Move::Rock) => Outcome::Win,
        (x, y) if x == y => Outcome::Draw,
        _ => Outcome::Lose,
    }
}

fn part_one(input: &str) -> u64 {
    let input = process_input(input);
    let mut score = 0;

    for inp in input {
        score += inp.1.score();
        score += calculate_outcome(inp.0, inp.1).score();
    }

    score
}

fn calculate_move(opponent: &Move, outcome: &Outcome) -> Move {
    match outcome {
        Outcome::Draw => opponent.clone(),
        Outcome::Win => match opponent {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock,
        },
        Outcome::Lose => match opponent {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper,
        },
    }
}

fn part_two(input: &str) -> u64 {
    let input = process_input(input);
    let mut score = 0;

    for round in input {
        let (opponent, me) = round;
        let me: Outcome = me.into();

        let my_move = calculate_move(&opponent, &me);

        score += me.score();
        score += my_move.score();
    }

    score
}

utils::aoc_problem!(day_02, part_one, 15, 13809, part_two, 12, 12316);
