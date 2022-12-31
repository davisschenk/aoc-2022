type Elfs = Vec<Vec<u64>>;

fn process_input(input: &str) -> Elfs {
    input
        .split("\n\n")
        .map(|i| {
            i.split('\n')
                .filter(|i| !i.is_empty())
                .map(|i| i.parse::<u64>().unwrap())
                .collect()
        })
        .collect()
}

fn part_one(input: &str) -> u64 {
    let elfs = process_input(input);
    elfs.iter().map(|i| i.iter().sum::<u64>()).max().unwrap()
}

fn part_two(input: &str) -> u64 {
    let elfs = process_input(input);
    let mut sums = elfs
        .iter()
        .map(|i| i.iter().sum::<u64>())
        .collect::<Vec<u64>>();
    sums.sort();
    sums.iter().rev().take(3).sum::<u64>()
}

utils::aoc_problem!(day_01, part_one, 24000, 71506, part_two, 45000, 209603);
