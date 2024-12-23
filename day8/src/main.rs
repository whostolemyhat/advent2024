use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    env::args,
    fmt::Display,
    fs::read_to_string,
    io,
};

mod test;

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = args().collect();

    let filename = &args[1];
    let input = read_to_string(filename)?;
    let mut map = Map::parse(&input);
    map.find_antinodes();
    println!("Part one: {}", map.antinodes.len());

    Ok(())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

// fn difference(a: Position, b: Position) -> (i32, i32) {
//     (
//         (b.x as i32 - a.x as i32).abs(),
//         (b.y as i32 - a.y as i32).abs(),
//     )
// }

#[derive(Debug, PartialEq)]
struct Map {
    map: Vec<char>,
    antennae: HashMap<char, Vec<Position>>,
    antinodes: HashSet<Position>,
    width: usize,
    height: usize,
}

impl Map {
    fn parse(input: &str) -> Self {
        let mut antennae: HashMap<char, Vec<Position>> = HashMap::new();
        let mut height: usize = 0;

        let map: Vec<char> = input
            .lines()
            .flat_map(|line| {
                let row = line.trim().chars().collect::<Vec<_>>();
                row.iter().enumerate().for_each(|(index, item)| {
                    if *item != '.' {
                        let position = Position {
                            x: index as i32,
                            y: height as i32,
                        };
                        antennae
                            .entry(*item)
                            .and_modify(|pos: &mut Vec<Position>| pos.push(position))
                            .or_insert(vec![position.clone()]);
                    }
                });
                height += 1;
                row
            })
            .collect();
        let width = map.len() / height;

        Map {
            map,
            antennae,
            antinodes: HashSet::new(),
            width,
            height,
        }
    }

    // fn coord(&self, pos: Position) -> usize {
    //     (pos.y * self.width) + pos.x
    // }

    fn position(&self, coord: usize) -> Position {
        Position {
            x: (coord % self.width) as i32,
            y: (coord / self.width) as i32,
        }
    }

    fn out_of_bounds(&self, pos: &Position) -> bool {
        pos.x >= self.width as i32 || pos.x < 0 || pos.y >= self.height as i32 || pos.y < 0
    }

    // for part 1, limit depth to 1 and don't add origin
    fn find_antinodes(&mut self) {
        let mut antinodes: HashSet<Position> = HashSet::new();

        self.antennae.iter().for_each(|(_key, val)| {
            val.iter().combinations(2).for_each(|pair| {
                let mut loop_count = 1;

                'place: loop {
                    let difference = Position {
                        x: pair[1].x - pair[0].x,
                        y: pair[1].y - pair[0].y,
                    };
                    let mut added = 0;

                    let possible_pos = (
                        Position {
                            x: pair[0].x + difference.x * loop_count,
                            y: pair[0].y + difference.y * loop_count,
                        },
                        Position {
                            x: (pair[1].x - difference.x * loop_count),
                            y: (pair[1].y - difference.y * loop_count),
                        },
                    );

                    if !(self.out_of_bounds(&possible_pos.0)) {
                        antinodes.insert(possible_pos.0);
                        added += 1;
                    }
                    if !(self.out_of_bounds(&possible_pos.1)) {
                        antinodes.insert(possible_pos.1);
                        added += 1;
                    }

                    loop_count += 1;

                    if added == 0 {
                        break 'place;
                    }
                }
            });
        });

        self.antinodes = antinodes;
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.map.iter().enumerate().for_each(|(index, item)| {
            let pos = self.position(index);
            if self.antinodes.get(&pos).is_some() && item == &'.' {
                write!(f, "#").expect("Failed to write antinode");
            } else {
                write!(f, "{}", item).expect("Failed to write item");
            }
            if (index + 1) % self.width == 0 {
                write!(f, "\n").expect("Failed to add new line");
            }
        });
        Ok(())
    }
}
