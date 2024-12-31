use std::{collections::VecDeque, env::args, fs::read_to_string, io};

mod test;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Map {
    map: Vec<u8>,
    width: usize,
    height: usize,
    trailheads: Vec<Position>,
}
impl Map {
    fn parse(input: &str) -> Self {
        let mut height = 0;
        let mut trailheads = vec![];

        let map: Vec<u8> = input
            .lines()
            .flat_map(|line| {
                let row = line
                    .trim()
                    .chars()
                    .enumerate()
                    .map(|(index, n)| {
                        let val = n.to_digit(10).expect("Not a number") as u8;
                        if val == 0 {
                            trailheads.push(Position {
                                x: index,
                                y: height,
                            });
                        }
                        val
                    })
                    .collect::<Vec<_>>();
                height += 1;
                row
            })
            .collect();
        let width = map.len() / height;

        Map {
            width,
            height,
            map,
            trailheads,
        }
    }

    // fn position(&self, coord: usize) -> Position {
    //     Position {
    //         x: (coord % self.width) as usize,
    //         y: (coord / self.width) as usize,
    //     }
    // }

    fn coord(&self, pos: &Position) -> usize {
        (pos.y * self.width) + pos.x
    }

    fn neighbours(&self, pos: &Position) -> Vec<Position> {
        let mut neighbours = vec![];

        if pos.y > 0 {
            neighbours.push(Position {
                x: pos.x,
                y: pos.y - 1,
            });
        }
        if pos.y < self.height - 1 {
            neighbours.push(Position {
                x: pos.x,
                y: pos.y + 1,
            });
        }

        if pos.x < self.width - 1 {
            neighbours.push(Position {
                x: pos.x + 1,
                y: pos.y,
            });
        }
        if pos.x > 0 {
            neighbours.push(Position {
                x: pos.x - 1,
                y: pos.y,
            });
        }
        neighbours
    }

    fn find_trail(&self) -> usize {
        let mut total = 0;
        let mut trails = vec![];

        for pos in self.trailheads[..].iter() {
            let mut visited = vec![];
            let mut count = 0;
            trails.push(self.search(&mut visited, pos, &mut count));
            total += count;
        }

        total
    }

    fn rate_trails(&self) -> usize {
        let mut total = 0;

        for pos in self.trailheads[..].iter() {
            let mut visited = vec![];
            let mut count = 0;
            let mut queue = VecDeque::new();
            queue.push_back(*pos);
            self.find_distinct(&mut visited, &mut queue, &mut count);
            total += count;
        }

        total
    }

    // how many '9's can be reached? (dfs)
    fn search(
        &self,
        visited: &mut Vec<Position>,
        current: &Position,
        count: &mut usize,
    ) -> Vec<Position> {
        let neighbours = self.neighbours(&current);
        let val = self.map[self.coord(current)];
        if self.map[self.coord(current)] == 9 {
            *count += 1;
        }

        visited.push(*current);

        for pos in neighbours {
            let neighbour_val = self.map[self.coord(&pos)];
            if neighbour_val == val + 1 && !visited.contains(&pos) {
                self.search(visited, &pos, count);
            }
        }

        visited.to_vec()
    }

    // find distinct paths (bfs)
    fn find_distinct(
        &self,
        visited: &mut Vec<Position>,
        queue: &mut VecDeque<Position>,
        count: &mut usize,
    ) {
        if queue.len() == 0 {
            return;
        }

        let current = queue.pop_front().expect("Failed to get current pos");
        let neighbours = self.neighbours(&current);
        let val = self.map[self.coord(&current)];
        if self.map[self.coord(&current)] == 9 {
            *count += 1;
        }

        for pos in neighbours {
            let neighbour_val = self.map[self.coord(&pos)];
            if neighbour_val == val + 1 {
                queue.push_back(pos);
                self.find_distinct(visited, queue, count);
            }
        }
    }
}

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = args().collect();

    let filename = &args[1];
    let input = read_to_string(filename)?;
    let map = Map::parse(&input);

    println!("Part 1: {:?}", map.find_trail());
    println!("Rating: {:?}", map.rate_trails());

    Ok(())
}
