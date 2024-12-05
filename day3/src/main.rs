use regex::{Regex, RegexBuilder};
use std::{fs::read_to_string, io};

fn main() -> Result<(), io::Error> {
    let _part_one_total = get_sum(&get_instructions(&read_to_string("./input.txt")?));
    let part_two_total = get_sum(&get_toggle_instructions(&read_to_string("./input.txt")?));

    println!("Sum: {}", part_two_total);

    Ok(())
}

fn get_instructions(input: &str) -> Vec<(u32, u32)> {
    let regex = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

    let results: Vec<(u32, u32)> = regex
        .captures_iter(input)
        .map(|caps| {
            let (_, [first, second]) = caps.extract();
            (
                first.parse::<u32>().unwrap(),
                second.parse::<u32>().unwrap(),
            )
        })
        .collect();

    results
}

fn get_toggle_instructions(input: &str) -> Vec<(u32, u32)> {
    // strip everything between `don't()` and `do()`
    let regex = RegexBuilder::new(r"don\'t\(\).*?do\(\)")
        .dot_matches_new_line(true)
        .build()
        .unwrap();
    let only_valid = regex.replace_all(input, "");

    get_instructions(&only_valid)
}

fn get_sum(input: &Vec<(u32, u32)>) -> u32 {
    input
        .iter()
        .fold(Vec::new(), |mut array, item| {
            array.push(item.0 * item.1);
            array
        })
        .iter()
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::{get_instructions, get_sum, get_toggle_instructions};

    #[test]
    fn should_match_sequence() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(
            get_instructions(&input),
            vec![(2, 4), (5, 5), (11, 8), (8, 5)]
        );
    }

    #[test]
    fn should_sum() {
        assert_eq!(get_sum(&vec![(2, 4), (5, 5), (11, 8), (8, 5)]), 161);
    }

    #[test]
    fn should_toggle() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

        assert_eq!(get_toggle_instructions(&input), vec![(2, 4), (8, 5)]);

        let input = "mul(1,2)don't()xyz
        
        mul(2,100)do()";
        assert_eq!(get_toggle_instructions(&input), vec![(1, 2)]);
    }
}
