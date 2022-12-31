use regex::Regex;

const MONKEY_REGEX: &str = r"Monkey (\d):\n +Starting items: ([\d, ]+)\n +Operation: new = (.*)\n +Test: divisible by (\d+)\n + If true: throw to monkey (\d)\n +If false: throw to monkey (\d)";

#[derive(Debug)]
struct Monkeys(Vec<Monkey>);

#[derive(Debug)]
struct Monkey {
    items: Vec<usize>,
    operation: String,
    divisible: usize,
    if_true: usize,
    if_false: usize,
    items_inspected: usize,
}

impl Monkey {
    fn inspect_item(&mut self, worry: Option<usize>) -> Option<(usize, usize)> {
        let mut item = match self.items.pop() {
            Some(x) => x,
            None => return None,
        };

        self.items_inspected += 1;

        // println!("  Monkey inspects item with worry level of {}.", item);
        // Calculate new worry level
        let operands: Vec<&str> = self.operation.split_whitespace().collect();

        item = match operands[..] {
            ["old", "*", "old"] => item * item,
            ["old", "+", x] => x.parse::<usize>().unwrap() + item,
            ["old", "*", x] => x.parse::<usize>().unwrap() * item,
            _ => panic!("Cannot parse operator"),
        };

        // Monkey gets bored
        if worry.is_none() {
            item /= 3;
        } else if let Some(m) = worry {
            item %= m;
        }

        // println!("    Monkey gets bored. Worry level is divided by 3 and now {}", item);

        if item % self.divisible == 0 {
            // println!("    Item is divisible by {}. Thrown to {}", self.divisible, self.if_true);
            Some((self.if_true, item))
        } else {
            // println!("    Item is not divisible by {}. Thrown to {}", self.divisible, self.if_false);
            Some((self.if_false, item))
        }
    }
}

impl Monkeys {
    fn round(&mut self, worry: Option<usize>) {
        for monkey_i in 0..self.0.len() {
            // println!("Monkey {}:", monkey_i);
            while let Some((index, item)) = self.0[monkey_i].inspect_item(worry) {
                self.0[index].items.push(item);
            }
        }
    }

    fn calculate_divisor(&self) -> usize {
        self.0.iter().map(|m| m.divisible).product()
    }
}

fn read_input(input: &str) -> Monkeys {
    let re = Regex::new(MONKEY_REGEX).unwrap();

    Monkeys(
        re.captures_iter(input)
            .map(|cap| Monkey {
                items: cap[2]
                    .split(", ")
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>()
                    .into_iter()
                    .rev()
                    .collect(),
                operation: cap[3].to_owned(),
                divisible: cap[4].parse().unwrap(),
                if_true: cap[5].parse().unwrap(),
                if_false: cap[6].parse().unwrap(),
                items_inspected: 0,
            })
            .collect::<Vec<Monkey>>(),
    )
}

fn part_one(input: &str) -> usize {
    let mut monkeys = read_input(input);

    for _ in 0..20 {
        monkeys.round(None)
    }

    let mut inspected = monkeys
        .0
        .iter()
        .map(|m| m.items_inspected)
        .collect::<Vec<usize>>();

    inspected.sort_unstable();
    inspected.iter().rev().take(2).product()
}

fn part_two(input: &str) -> usize {
    let mut monkeys = read_input(input);

    for _ in 0..10000 {
        monkeys.round(Some(monkeys.calculate_divisor()))
    }

    let mut inspected = monkeys
        .0
        .iter()
        .map(|m| m.items_inspected)
        .collect::<Vec<usize>>();

    inspected.sort_unstable();
    inspected.iter().rev().take(2).product()
}

utils::aoc_problem!(
    day_11,
    part_one,
    10605,
    90294,
    part_two,
    2713310158,
    18170818354
);

#[cfg(test)]
mod day_11_additional {}
