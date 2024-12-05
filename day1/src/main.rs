use std::collections::HashMap;
use std::env::args;
use std::{fs::read_to_string, io};

fn part_one(filename: &str) -> u32 {
    // read file
    // sort both lists
    // iterate and compare
    let mut first = vec![];
    let mut second = vec![];
    let mut total = 0;

    read_to_string(filename).unwrap().lines().for_each(|line| {
        let locations = line.split_whitespace().collect::<Vec<&str>>();
        first.push(locations[0].parse::<i32>().unwrap());
        second.push(locations[1].parse::<i32>().unwrap());
    });
    first.sort();
    second.sort();

    let _ = first.iter().enumerate().for_each(|(index, location)| {
        // get difference between left and right
        total += location.abs_diff(second[index])
    });

    total
}

fn part_two(filename: &str) -> u64 {
    let mut first = vec![];
    let mut map = HashMap::new();
    let mut total = 0;

    read_to_string(filename).unwrap().lines().for_each(|line| {
        let locations = line.split_whitespace().collect::<Vec<&str>>();
        first.push(locations[0].parse::<u64>().unwrap());
        let key = locations[1]
            .parse::<u64>()
            .expect("Failed to parse second number");
        *map.entry(key).or_insert(0) += 1;
    });

    first.iter().for_each(|item| {
        let amount = item * map.get(item).or(Some(&0)).unwrap();
        total += amount;
    });
    total
}

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = args().collect();

    let filename = &args[1];
    let _part1 = part_one(&filename);
    let part2 = part_two(&filename);

    println!("total {:?}", part2);

    Ok(())
}
