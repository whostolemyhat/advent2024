use std::{env::args, fs::read_to_string, io};

mod test;

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = args().collect();

    let filename = &args[1];
    let input = read_to_string(filename)?;
    let combos = Calibration::parse(&input);

    let solvable = find_solvable(&combos, false);
    let total: u64 = get_total(&solvable);
    println!("Part one: {}", total);

    let part_two_solvable = find_solvable(&combos, true);
    let part_two_total = get_total(&part_two_solvable);
    println!("Part two: {}", part_two_total);

    Ok(())
}

fn get_total(valid: &Vec<&Calibration>) -> u64 {
    valid.iter().map(|c| c.target).fold(0, |acc, n| acc + n)
}

fn find_solvable(combos: &Vec<Calibration>, use_concat: bool) -> Vec<&Calibration> {
    combos
        .iter()
        .filter(|c| solve(c.target, &c.nums[1..], c.nums[0], use_concat))
        .collect()
}

#[derive(Debug, PartialEq)]
struct Calibration {
    target: u64,
    nums: Vec<u64>,
}

impl Calibration {
    fn parse(input: &str) -> Vec<Calibration> {
        input
            .lines()
            .map(|line| {
                let split: Vec<&str> = line.trim().split(":").collect();
                let target = split[0]
                    .trim()
                    .parse::<u64>()
                    .expect("Target must be a number");
                let nums = split[1]
                    .split_whitespace()
                    .map(|n| n.parse::<u64>().expect("items must be numbers"))
                    .collect();

                Calibration { target, nums }
            })
            .collect()
    }
}

fn concat(a: u64, b: u64) -> u64 {
    // str/parse is fairly slow
    format!("{}{}", a, b)
        .parse()
        .expect("Couldn't concat numbers")
}

fn solve(target: u64, rest: &[u64], current: u64, use_concat: bool) -> bool {
    if rest.is_empty() {
        return target == current;
    }

    // rest.len == 1 = no early return, need to use all nums
    if current * rest[0] == target && rest.len() == 1 {
        return true;
    } else if current + rest[0] == target && rest.len() == 1 {
        return true;
    } else if use_concat && concat(current, rest[0]) == target && rest.len() == 1 {
        return true;
    }

    if current * rest[0] <= target {
        if solve(target, &rest[1..], current * rest[0], use_concat) {
            return true;
        }
    }

    if current + rest[0] <= target {
        if solve(target, &rest[1..], current + rest[0], use_concat) {
            return true;
        }
    }

    if use_concat {
        if concat(current, rest[0]) <= target {
            if solve(target, &rest[1..], concat(current, rest[0]), use_concat) {
                return true;
            }
        }
    }

    false
}
