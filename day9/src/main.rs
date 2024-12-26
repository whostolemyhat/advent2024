use std::{env::args, fs::read_to_string, io, iter::repeat};

mod test;

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = args().collect();

    let filename = &args[1];
    let input = read_to_string(filename)?;
    let mut programme = Program::parse(&input);
    programme.frag();
    println!("Part one: {}", checksum(&programme.list));

    Ok(())
}

#[derive(PartialEq, Debug)]
struct Program {
    blocks: String,
    free_count: u32,
    list: Vec<String>,
}

impl Program {
    fn parse(input: &str) -> Self {
        let mut count = 0;
        let mut id = 0;

        let output = input
            .chars()
            .enumerate()
            .flat_map(|(index, c)| {
                if index % 2 == 0 {
                    let block = repeat(format!("{}", id))
                        .take(c.to_digit(10).expect("Not a number") as usize)
                        .collect::<Vec<String>>();
                    id += 1;
                    block
                } else {
                    let multiplier = c.to_digit(10).expect("Not a number");
                    count += multiplier;
                    repeat(String::from("."))
                        .take(multiplier as usize)
                        .collect::<Vec<String>>()
                }
            })
            .collect::<Vec<String>>();

        Self {
            free_count: count,
            blocks: output.join(""),
            list: output,
        }
    }

    fn frag(&mut self) {
        let len = self.list.len();

        let mut first_index = self
            .list
            .iter()
            .position(|c| c == ".")
            .expect("Didn't find dot");
        let mut letter_index = self
            .list
            .iter()
            .rev()
            .position(|c| c != ".")
            .expect("No letters");

        loop {
            first_index = first_index
                + self
                    .list
                    .iter()
                    .skip(first_index)
                    .position(|c| c == ".")
                    .expect("Didn't find dot");

            letter_index = self
                .list
                .iter()
                .skip(letter_index)
                .rev()
                .position(|c| c != ".")
                .expect("No letters");

            self.list.swap(first_index, (len - 1) - letter_index);

            // TODO check magic num
            if self.list.iter().skip(first_index + 2).all(|c| c == ".") {
                break;
            }
        }
    }
}

fn checksum(input: &Vec<String>) -> u64 {
    input
        .iter()
        .enumerate()
        .fold(0, |acc, (index, e)| match e.parse::<u64>() {
            Ok(num) => acc + (index as u64 * num as u64),
            Err(_) => acc,
        })
}
