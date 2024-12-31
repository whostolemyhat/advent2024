use std::{collections::HashMap, env::args, fs::read_to_string, io, iter::repeat};

mod test;

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = args().collect();

    let filename = &args[1];
    let input = read_to_string(filename)?;
    let mut programme = Program::parse(&input);
    // programme.frag();
    // println!("Part one: {}", checksum(&programme.list));

    println!("Part 2: {}", programme.defrag());

    Ok(())
}

#[derive(PartialEq, Debug)]
struct Program {
    // blocks: String,
    free_count: u32,
    list: Vec<String>,
    //            length, indices
    // spaces: HashMap<usize, Vec<usize>>,
    blocks: Vec<Block>,
}

#[derive(PartialEq, Copy, Clone, Debug)]
struct File {
    id: usize,
    size: usize,
    offset: usize,
}

#[derive(PartialEq, Copy, Clone, Debug)]
struct Space {
    size: usize,
    offset: usize,
}

#[derive(PartialEq, Copy, Clone, Debug)]
enum Block {
    Empty(Space),
    File(File),
}
impl Program {
    fn parse(input: &str) -> Self {
        let mut count = 0;
        let mut offset = 0;
        let mut id = 0;
        // let mut spaces = HashMap::new();
        let mut blocks = Vec::new();

        let output = input
            .chars()
            .enumerate()
            .flat_map(|(index, c)| {
                if index % 2 == 0 {
                    let size = c.to_digit(10).expect("Not a number") as usize;
                    let block = repeat(format!("{}", id))
                        .take(size)
                        .collect::<Vec<String>>();
                    blocks.push(Block::File(File { id, size, offset }));
                    offset += size;
                    id += 1;
                    block
                } else {
                    let size = c.to_digit(10).expect("Not a number") as usize;
                    count += size;
                    // spaces
                    //     .entry(multiplier)
                    //     .and_modify(|l: &mut Vec<usize>| l.push(index * (multiplier as usize)))
                    //     .or_insert(vec![index * (multiplier as usize)]);

                    blocks.push(Block::Empty(Space { size, offset }));
                    offset += size;
                    repeat(String::from("."))
                        .take(size)
                        .collect::<Vec<String>>()
                }
            })
            .collect::<Vec<String>>();

        Self {
            free_count: count as u32,
            list: output,
            // spaces,
            blocks,
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

    fn defrag(&self) -> usize {
        let mut sum = 0;
        let mut cache = [0; 10];

        let (mut files, mut spaces): (Vec<Block>, Vec<Block>) = self
            .blocks
            .iter()
            .partition(|b| matches!(**b, Block::File(_)));

        // from end
        for file in files.iter_mut().rev() {
            let mut found = false;
            if let Block::File(file) = file {
                for i in cache[file.size]..file.id {
                    if let Block::Empty(mut space) = spaces[i] {
                        if space.size >= file.size {
                            let offset = space.offset;
                            sum += file.id * (offset * 2 + file.size - 1) * file.size / 2;
                            space.size -= file.size;
                            space.offset += file.size;
                            cache[file.size] = i;
                            found = true;
                            break;
                        }
                    }
                }
                // from start, find block with length >= file.len
                // for space in spaces.iter_mut() {
                //     if let Block::Empty(space) = space {
                //         if space.size >= file.size {
                //             sum += file.id * (space.offset * 2 + file.size - 1) * file.size / 2;
                //             space.size -= file.size;
                //             space.offset += file.size;
                //             found = true;
                //             break;
                //         }
                //     }
                // }
                if !found {
                    sum += file.id * (file.offset * 2 + file.size - 1) * file.size / 2;
                    cache[file.size] = usize::MAX;
                }
            }
        }

        // dbg!(files);
        // dbg!(spaces);
        sum
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
