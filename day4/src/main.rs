use std::{fs::read_to_string, io};

fn main() -> Result<(), io::Error> {
    let input = read_to_string("./input.txt")?;
    let grid = Grid::new(&input);
    let part_one_result = grid.count();

    let cross_grid = Grid::with_diagonal_only(&input);
    let _part_two_result = cross_grid.count();

    println!("Total: {:?}", part_one_result);

    Ok(())
}

#[derive(PartialEq, Debug)]
enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

type Position = (usize, usize);

fn get_directions(pos: Position, max_x: usize, max_y: usize, cross_only: bool) -> Vec<Direction> {
    let mut directions = vec![];
    // 2: it's 0-indexed!
    // 3: .len() isn't
    if !cross_only {
        if pos.0 > 2 {
            directions.push(Direction::West);
        }
        if pos.0 < max_x - 3 {
            directions.push(Direction::East);
        }
        if pos.1 > 2 {
            directions.push(Direction::North);
        }

        if pos.1 < max_y - 3 {
            directions.push(Direction::South);
        }
    }

    if pos.0 > 2 && pos.1 > 2 {
        directions.push(Direction::NorthWest);
    }

    if pos.0 > 2 && pos.1 < max_y - 3 {
        directions.push(Direction::SouthWest)
    }

    if pos.0 < max_x - 3 && pos.1 > 2 {
        directions.push(Direction::NorthEast);
    }

    if pos.0 < max_x - 3 && pos.1 < max_y - 3 {
        directions.push(Direction::SouthEast)
    }

    directions
}

struct Grid {
    // grid: Vec<Vec<char>>,
    grid: Vec<char>,
    width: usize,
    height: usize,
    cross_only: bool,
}

impl Grid {
    fn new(input: &str) -> Self {
        // let mut matrix = vec![];
        let mut height = 0;
        // for line in input.lines() {
        //     width += 1;
        //     matrix.push(line.trim().chars().collect());
        // }
        let matrix: Vec<char> = input
            .lines()
            .flat_map(|line| {
                height += 1;
                line.trim().chars().collect::<Vec<_>>()
            })
            .collect();

        let width = matrix.len() / height;

        Grid {
            grid: matrix,
            height,
            width,
            cross_only: false,
        }
    }

    fn with_diagonal_only(input: &str) -> Self {
        let grid = Grid::new(&input);

        Grid {
            grid: grid.grid,
            width: grid.width,
            height: grid.height,
            cross_only: true,
        }
    }

    fn coord(&self, x: usize, y: usize) -> char {
        self.grid[(y * self.width) + x]
    }

    fn count(&self) -> u32 {
        let mut total = 0;
        let search_char = if self.cross_only { 'A' } else { 'X' };

        if self.cross_only {
            for y in 0..self.height {
                for x in 0..self.width {
                    // if self.grid[y][x] == search_char {
                    if self.coord(x, y) == search_char {
                        // check both diags for SAM and MAS
                        // let upleft = MAS || SAM && upright = SAM || MAS
                        if y > 0 && y < self.height - 1 && x > 0 && x < self.width - 1 {
                            // top-left to bottom-right
                            let up_left = (self.coord(x - 1, y - 1) == 'S'
                                && self.coord(x + 1, y + 1) == 'M')
                                || (self.coord(x - 1, y - 1) == 'M'
                                    && self.coord(x + 1, y + 1) == 'S');

                            // top-right to bottom-left
                            let up_right = (self.coord(x + 1, y - 1) == 'M'
                                && self.coord(x - 1, y + 1) == 'S')
                                || (self.coord(x + 1, y - 1) == 'S'
                                    && self.coord(x - 1, y + 1) == 'M');

                            if up_left && up_right {
                                total += 1;
                            }
                        }
                    }
                }
            }

            return total;
        }

        for y in 0..self.height {
            for x in 0..self.width {
                if self.coord(x, y) == search_char {
                    let directions =
                        get_directions((x, y), self.width, self.height, self.cross_only);
                    for direction in directions {
                        match direction {
                            Direction::North => {
                                if self.coord(x, y - 1) == 'M'
                                    && self.coord(x, y - 2) == 'A'
                                    && self.coord(x, y - 3) == 'S'
                                {
                                    total += 1;
                                }
                            }
                            Direction::West => {
                                if self.coord(x - 1, y) == 'M'
                                    && self.coord(x - 2, y) == 'A'
                                    && self.coord(x - 3, y) == 'S'
                                {
                                    total += 1;
                                }
                            }
                            Direction::South => {
                                if self.coord(x, y + 1) == 'M'
                                    && self.coord(x, y + 2) == 'A'
                                    && self.coord(x, y + 3) == 'S'
                                {
                                    total += 1;
                                }
                            }
                            Direction::East => {
                                if self.coord(x + 1, y) == 'M'
                                    && self.coord(x + 2, y) == 'A'
                                    && self.coord(x + 3, y) == 'S'
                                {
                                    total += 1;
                                }
                            }
                            Direction::NorthWest => {
                                if self.coord(x - 1, y - 1) == 'M'
                                    && self.coord(x - 2, y - 2) == 'A'
                                    && self.coord(x - 3, y - 3) == 'S'
                                {
                                    total += 1;
                                }
                            }

                            Direction::SouthWest => {
                                if self.coord(x - 1, y + 1) == 'M'
                                    && self.coord(x - 2, y + 2) == 'A'
                                    && self.coord(x - 3, y + 3) == 'S'
                                {
                                    total += 1;
                                }
                            }

                            Direction::SouthEast => {
                                if self.coord(x + 1, y + 1) == 'M'
                                    && self.coord(x + 2, y + 2) == 'A'
                                    && self.coord(x + 3, y + 3) == 'S'
                                {
                                    total += 1;
                                }
                            }

                            Direction::NorthEast => {
                                if self.coord(x + 1, y - 1) == 'M'
                                    && self.coord(x + 2, y - 2) == 'A'
                                    && self.coord(x + 3, y - 3) == 'S'
                                {
                                    total += 1;
                                }
                            }
                        }
                    }
                }
            }
        }

        total
    }
}

#[cfg(test)]
mod tests {
    use crate::{get_directions, Direction, Grid};

    #[test]
    fn input_to_matrix() {
        let input = "MMMSXXMASM
          MSAMXMSMSA
          AMXSXMAAMM
          MSAMASMSMX
          XMASAMXAMM
          XXAMMXXAMA
          SMSMSASXSS
          SAXAMASAAA
          MAMMMXMMMM
          MXMXAXMASX";
        let grid = Grid::new(&input);
        assert_eq!(
            grid.grid,
            vec![
                'M', 'M', 'M', 'S', 'X', 'X', 'M', 'A', 'S', 'M', 'M', 'S', 'A', 'M', 'X', 'M',
                'S', 'M', 'S', 'A', 'A', 'M', 'X', 'S', 'X', 'M', 'A', 'A', 'M', 'M', 'M', 'S',
                'A', 'M', 'A', 'S', 'M', 'S', 'M', 'X', 'X', 'M', 'A', 'S', 'A', 'M', 'X', 'A',
                'M', 'M', 'X', 'X', 'A', 'M', 'M', 'X', 'X', 'A', 'M', 'A', 'S', 'M', 'S', 'M',
                'S', 'A', 'S', 'X', 'S', 'S', 'S', 'A', 'X', 'A', 'M', 'A', 'S', 'A', 'A', 'A',
                'M', 'A', 'M', 'M', 'M', 'X', 'M', 'M', 'M', 'M', 'M', 'X', 'M', 'X', 'A', 'X',
                'M', 'A', 'S', 'X',
            ]
        )
    }

    #[test]
    fn should_count_xmas() {
        let input = "MMMSXXMASM
          MSAMXMSMSA
          AMXSXMAAMM
          MSAMASMSMX
          XMASAMXAMM
          XXAMMXXAMA
          SMSMSASXSS
          SAXAMASAAA
          MAMMMXMMMM
          MXMXAXMASX";
        let grid = Grid::new(&input);
        assert_eq!(grid.count(), 18);
    }

    #[test]
    fn should_return_directions() {
        assert_eq!(
            get_directions((0, 0), 8, 8, false),
            vec![Direction::East, Direction::South, Direction::SouthEast,]
        );
        assert_eq!(
            get_directions((4, 0), 8, 8, false),
            vec![
                Direction::West,
                Direction::East,
                Direction::South,
                Direction::SouthWest,
                Direction::SouthEast,
            ]
        );
        assert_eq!(
            get_directions((7, 0), 8, 8, false),
            vec![Direction::West, Direction::South, Direction::SouthWest,]
        );
        assert_eq!(
            get_directions((0, 4), 10, 10, false),
            vec![
                Direction::East,
                Direction::North,
                Direction::South,
                Direction::NorthEast,
                Direction::SouthEast
            ]
        );
        assert_eq!(
            get_directions((4, 4), 10, 10, false),
            vec![
                Direction::West,
                Direction::East,
                Direction::North,
                Direction::South,
                Direction::NorthWest,
                Direction::SouthWest,
                Direction::NorthEast,
                Direction::SouthEast,
            ]
        );
        assert_eq!(
            get_directions((7, 4), 10, 10, false),
            vec![
                Direction::West,
                Direction::North,
                Direction::South,
                Direction::NorthWest,
                Direction::SouthWest,
            ]
        );
        assert_eq!(
            get_directions((0, 7), 10, 10, false),
            vec![Direction::East, Direction::North, Direction::NorthEast,]
        );

        assert_eq!(
            get_directions((4, 7), 10, 10, false),
            vec![
                Direction::West,
                Direction::East,
                Direction::North,
                Direction::NorthWest,
                Direction::NorthEast,
            ]
        );
        assert_eq!(
            get_directions((7, 7), 10, 10, false),
            vec![Direction::West, Direction::North, Direction::NorthWest,]
        );
    }

    #[test]
    fn should_find_cross_mas() {
        let input = "MMMSXXMASM
          MSAMXMSMSA
          AMXSXMAAMM
          MSAMASMSMX
          XMASAMXAMM
          XXAMMXXAMA
          SMSMSASXSS
          SAXAMASAAA
          MAMMMXMMMM
          MXMXAXMASX";
        let grid = Grid::with_diagonal_only(&input);
        assert_eq!(grid.count(), 9);
    }
}
