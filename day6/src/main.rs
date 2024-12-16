use std::{collections::HashSet, env::args, fs::read_to_string, io};

mod test;

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = args().collect();

    let filename = &args[1];
    let input = read_to_string(filename)?;

    // let mut game_over = false;
    // calc straight line to next obstacle or bounds
    let mut map = Map::parse(&input);
    let part_one = run(&mut map);
    match part_one {
        Some(positions) => {
            println!("Part one: {}", positions.len());

            // part 2
            // create guard route
            // for each pos
            // add obstacle and run entire game
            // check if stuck in a loop
            // loop = adding same guard pos and direction
            // note takes a while
            let loop_total = check_for_loops(positions, &mut map);
            // off by one :shrug:
            println!("part two: {:?}", loop_total);
        }

        None => println!("uh oh"),
    }

    Ok(())
}

fn run(map: &mut Map) -> Option<HashSet<Position>> {
    let mut route = HashSet::new();

    'game: loop {
        let worked = route.insert(map.guard);

        if !worked {
            // it's a loop!
            return None;
        }
        map.step();
        if map.out_of_bounds(&map.guard.position) {
            break 'game;
        }
    }

    // let mut debug = map.map.clone();

    // map.guard_route.clone().iter().for_each(|v| {
    // debug[map.coord(v.position.0, v.position.1)] = 'X';
    // });

    // dbg!(debug);

    // get unique positions by stripping direction
    let positions: HashSet<_> = route.iter().map(|guard| guard.position).collect();

    Some(positions)
}

fn check_for_loops(route: HashSet<Position>, map: &mut Map) -> i32 {
    let mut loops = 0;

    for pos in route.iter() {
        // reset
        map.guard.position = map.start.position;
        map.guard.facing = map.start.facing;

        map.obstacles.push(*pos);

        if run(map).is_none() {
            loops += 1;
        }
        map.obstacles.remove(map.obstacles.len() - 1);
    }

    loops
}

type Position = (usize, usize);

#[derive(PartialEq, Debug, Clone)]
struct Map {
    map: Vec<char>,
    width: usize,
    height: usize,
    obstacles: Vec<Position>, // or hashmap
    start: Guard,
    guard: Guard,
}

impl Map {
    // fn coord(&self, x: usize, y: usize) -> usize {
    //     (y * self.width) + x
    // }

    fn out_of_bounds(&self, pos: &Position) -> bool {
        let x_out_of_bounds = pos.0 == 0 || pos.0 >= self.width;
        let y_out_of_bounds = pos.1 == 0 || pos.1 >= self.height;
        x_out_of_bounds || y_out_of_bounds
    }

    fn step(&mut self) {
        match self.guard.facing {
            Direction::East => {
                self.guard.position.0 += 1;
                if self.obstacles.contains(&self.guard.position) {
                    self.guard.turn();
                    self.guard.position.0 -= 1;
                }
            }
            Direction::South => {
                self.guard.position.1 += 1;
                if self.obstacles.contains(&self.guard.position) {
                    self.guard.turn();
                    self.guard.position.1 -= 1;
                }
            }
            Direction::West => {
                self.guard.position.0 -= 1;
                if self.obstacles.contains(&self.guard.position) {
                    self.guard.turn();
                    self.guard.position.0 += 1;
                }
            }
            Direction::North => {
                self.guard.position.1 -= 1;
                if self.obstacles.contains(&self.guard.position) {
                    self.guard.turn();
                    self.guard.position.1 += 1;
                }
            }
        }
    }

    fn parse(input: &str) -> Self {
        let mut height = 0;
        let mut obstacles = Vec::new();
        let mut guard = Guard {
            facing: Direction::North,
            position: (0, 0),
        };

        let map: Vec<char> = input
            .lines()
            .flat_map(|line| {
                let row = line.trim().chars().collect::<Vec<_>>();
                row.iter().enumerate().for_each(|(index, p)| {
                    if *p == '#' {
                        // push to obstacles
                        obstacles.push((index, height));
                    }
                    if *p == '^' {
                        guard = Guard {
                            facing: Direction::North,
                            position: (index, height),
                        };
                    }
                    if *p == '>' {
                        guard = Guard {
                            facing: Direction::East,
                            position: (index, height),
                        };
                    }
                    if *p == 'v' {
                        guard = Guard {
                            facing: Direction::South,
                            position: (index, height),
                        };
                    }
                    if *p == '<' {
                        guard = Guard {
                            facing: Direction::West,
                            position: (index, height),
                        };
                    }
                });
                height += 1;
                row
            })
            .collect();
        let width = map.len() / height;

        Map {
            width,
            height,
            obstacles,
            start: guard.clone(),
            map,
            guard,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
struct Guard {
    position: Position,
    facing: Direction,
}

impl Guard {
    fn turn(&mut self) {
        match self.facing {
            Direction::East => {
                self.facing = Direction::South;
            }
            Direction::South => {
                self.facing = Direction::West;
            }
            Direction::West => {
                self.facing = Direction::North;
            }
            Direction::North => {
                self.facing = Direction::East;
            }
        }
    }
}

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}
