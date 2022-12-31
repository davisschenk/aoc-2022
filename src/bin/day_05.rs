use regex::Regex;

#[derive(Debug)]
struct Move {
    quantity: usize,
    from: usize,
    to: usize,
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<Move>) {
    let mut raw_map = Vec::new();
    let mut moves = Vec::new();

    let crate_line = Regex::new(r"(?:((?:   )|(?:\[[A-Z]\]))(?: |\n))+").unwrap();
    let individual_crate = Regex::new(r"((?:   )|(?:\[([A-Z])\]))(?: |\n)?").unwrap();
    let move_line = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    for line in input.lines() {
        if crate_line.is_match(line) {
            let crates = individual_crate
                .captures_iter(line)
                .map(|c| {
                    if &c[1] == "   " {
                        None
                    } else {
                        Some(c[2].chars().next().unwrap())
                    }
                })
                .collect::<Vec<Option<char>>>();

            raw_map.push(crates)
        } else if move_line.is_match(line) {
            let cap = move_line.captures(line).unwrap();
            moves.push(Move {
                quantity: cap[1].parse().unwrap(),
                from: cap[2].parse().unwrap(),
                to: cap[3].parse().unwrap(),
            })
        }
    }

    let total_columns = raw_map.iter().map(|c| c.len()).max().unwrap();
    let mut map = Vec::new();

    for i in 0..total_columns {
        let c = raw_map
            .iter()
            .map(|m| m.get(i).unwrap_or(&None))
            .filter(|v| matches!(v, Some(_)))
            .map(|v| v.unwrap())
            .collect();

        map.push(c);
    }

    (map, moves)
}

fn part_one(input: &str) -> String {
    let (mut crates, moves) = parse_input(input);

    for mv in moves {
        let removed: Vec<char> = crates[mv.from - 1].drain(0..mv.quantity).collect();

        for i in removed.iter() {
            crates[mv.to - 1].insert(0, *i)
        }
    }

    let mut string = String::new();

    for v in crates {
        string.push(v[0]);
    }

    string
}

fn part_two(input: &str) -> String {
    let (mut crates, moves) = parse_input(input);

    for mv in moves {
        let removed: Vec<char> = crates[mv.from - 1].drain(0..mv.quantity).collect();

        for i in removed.iter().rev() {
            crates[mv.to - 1].insert(0, *i)
        }
    }

    let mut string = String::new();

    for v in crates {
        string.push(v[0]);
    }

    string
}

utils::aoc_problem!(
    day_05,
    part_one,
    "CMZ",
    "LJSVLTWQM",
    part_two,
    "MCD",
    "BRQWDBBJM"
);

#[cfg(test)]
mod day_05_additional {}
