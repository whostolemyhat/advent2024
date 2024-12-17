#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use crate::{run, Direction, Guard, Map};

    #[test]
    fn it_should_create_map() {
        let input = "....#.....
          .........#
          ..........
          ..#.......
          .......#..
          ..........
          .#..^.....
          ........#.
          #.........
          ......#...";
        let map = Map::parse(&input);
        let mut guard_route = HashSet::new();
        let guard = Guard {
            position: (4, 6),
            facing: Direction::North,
        };
        guard_route.insert(guard);

        assert_eq!(
            map,
            Map {
                width: 10,
                height: 10,
                obstacles: vec![
                    (4, 0),
                    (9, 1),
                    (2, 3),
                    (7, 4),
                    (1, 6),
                    (8, 7),
                    (0, 8),
                    (6, 9)
                ],
                map: vec![
                    '.', '.', '.', '.', '#', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                    '.', '.', '.', '#', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                    '#', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '#',
                    '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '#', '.', '.',
                    '^', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '#', '.',
                    '#', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                    '#', '.', '.', '.'
                ],
                start: guard,
                guard
            }
        )
    }

    #[test]
    fn it_should_create_guard() {
        let input = "....#.....
          .........#
          ..........
          ..#.......
          .......#..
          ..........
          .#..^.....
          ........#.
          #.........
          ......#...";
        let map = Map::parse(&input);
        assert_eq!(
            map.guard,
            Guard {
                facing: Direction::North,
                position: (4, 6)
            }
        );

        let input = "....#.....
          .........#
          ..........
          ..#.......
          ..>....#..
          ..........
          .#........
          ........#.
          #.........
          ......#...";
        let map = Map::parse(&input);
        assert_eq!(
            map.guard,
            Guard {
                facing: Direction::East,
                position: (2, 4)
            }
        );

        let input = "....#.....
        .........#
        ..........
        ..#.......
        .......#..
        ..........
        .#........
        ........#.
        #.....v...
        ......#...";
        let map = Map::parse(&input);
        assert_eq!(
            map.guard,
            Guard {
                facing: Direction::South,
                position: (6, 8)
            }
        );

        let input = "....#.....
          .........#
          ..........
          ..#.......
          .......#..
          ..........
          .#........
          ........#.
          #<........
          ......#...";
        let map = Map::parse(&input);
        assert_eq!(
            map.guard,
            Guard {
                facing: Direction::West,
                position: (1, 8)
            }
        );
    }

    mod path {
        use crate::Map;

        #[test]
        fn it_should_draw_path_north() {
            let input = "....#.....
                     .........#
                     ..........
                     ..#.......
                     .......#..
                     ..........
                     .#..^.....
                     ........#.
                     #.........
                     ......#...";
            let mut map = Map::parse(&input);
            map.step();

            assert_eq!(map.guard.position, (4, 5));
        }

        #[test]
        fn it_should_draw_path_east() {
            let input = "....#.....
                         ....>....#
                         ..........
                         ..#.......
                         .......#..
                         ..........
                         .#........
                         ........#.
                         #.........
                         ......#...";
            let mut map = Map::parse(&input);
            map.step();

            assert_eq!(map.guard.position, (5, 1));
        }

        #[test]
        fn it_should_draw_path_south() {
            let input = "....#.....
                             ........v#
                             ..........
                             ..#.......
                             .......#..
                             ..........
                             .#........
                             ........#.
                             #.........
                             ......#...";
            let mut map = Map::parse(&input);
            map.step();

            assert_eq!(map.guard.position, (8, 2));
        }

        #[test]
        fn it_should_draw_path_west() {
            let input = "....#.....
                             .........#
                             ..........
                             ..#.......
                             .......#..
                             ..........
                             .#......<.
                             ........#.
                             #.........
                             ......#...";
            let mut map = Map::parse(&input);
            map.step();

            assert_eq!(map.guard.position, (7, 6),);
        }
    }

    #[test]
    fn it_should_finish() {
        let input = "....#.....
                         .........#
                         ..........
                         ..#.......
                         .......#..
                         ....^.....
                         .#........
                         ........#.
                         #.........
                         ......#...";

        let mut map = Map::parse(&input);
        let result = run(&mut map);

        assert_eq!(result.unwrap().len(), 41);
    }
}
