#[cfg(test)]
mod test {
    use std::collections::{HashMap, HashSet};

    use crate::{Map, Position};

    #[test]
    fn it_should_parse_map() {
        let input = "..........
          ..........
          ..........
          ....a.....
          ..........
          .....a....
          ..........
          ..........
          ..........
          ..........";
        let map = Map::parse(&input);

        let mut antennae = HashMap::new();
        antennae.insert('a', vec![Position { x: 4, y: 3 }, Position { x: 5, y: 5 }]);
        assert_eq!(
            map,
            Map {
                map: vec![
                    '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                    '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                    '.', '.', 'a', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                    '.', '.', '.', '.', '.', '.', '.', 'a', '.', '.', '.', '.', '.', '.', '.', '.',
                    '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                    '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                    '.', '.', '.', '.'
                ],
                antennae,
                antinodes: HashSet::new(),
                width: 10,
                height: 10
            }
        );
    }

    #[test]
    fn it_should_find_antinodes() {
        let input = "..........
          ..........
          ..........
          ....a.....
          ..........
          .....a....
          ..........
          ..........
          ..........
          ..........";
        let mut map = Map::parse(&input);

        println!("{}", map);

        map.find_antinodes();

        assert!(map.antinodes.contains(&Position { x: 3, y: 1 }));
        assert!(map.antinodes.contains(&Position { x: 6, y: 7 }));
    }

    #[test]
    fn it_should_find_antinodes_for_multiple_antennae() {
        let input = "..........
          ..........
          ..........
          ....a.....
          ........a.
          .....a....
          ..........
          ..........
          ..........
          ..........";
        let mut map = Map::parse(&input);
        map.find_antinodes();

        println!("{}", map);

        assert_eq!(
            map.antinodes,
            HashSet::from([
                Position { x: 0, y: 2 },
                Position { x: 3, y: 1 },
                Position { x: 6, y: 7 },
                Position { x: 2, y: 6 }
            ])
        );
    }

    #[test]
    fn it_should_find_antinodes_for_different_antennae() {
        let input = "..........
          ..........
          ..........
          ....a.....
          ........a.
          .....a....
          ..........
          ......A...
          ..........
          ..........";
        let mut map = Map::parse(&input);
        map.find_antinodes();

        println!("{}", map);

        assert_eq!(
            map.antinodes,
            HashSet::from([
                Position { x: 0, y: 2 },
                Position { x: 3, y: 1 },
                Position { x: 6, y: 7 },
                Position { x: 2, y: 6 }
            ])
        );
    }

    #[test]
    fn it_should_find_all_antinodes() {
        let input = "............
          ........0...
          .....0......
          .......0....
          ....0.......
          ......A.....
          ............
          ............
          ........A...
          .........A..
          ............
          ............";
        let mut map = Map::parse(&input);
        map.find_antinodes();

        println!("{}", map);

        // assert_eq!(map.antinodes.len(), 14);

        assert!(map.antinodes.contains(&Position { x: 11, y: 0 }));
        assert!(map.antinodes.contains(&Position { x: 2, y: 3 }));
        assert!(map.antinodes.contains(&Position { x: 6, y: 5 }));
        assert!(map.antinodes.contains(&Position { x: 0, y: 7 }));
        assert!(map.antinodes.contains(&Position { x: 3, y: 1 }));
        assert!(map.antinodes.contains(&Position { x: 9, y: 4 }));
        assert!(map.antinodes.contains(&Position { x: 6, y: 0 }));
        assert!(map.antinodes.contains(&Position { x: 3, y: 6 }));
        assert!(map.antinodes.contains(&Position { x: 10, y: 2 }));
        assert!(map.antinodes.contains(&Position { x: 1, y: 5 }));
        assert!(map.antinodes.contains(&Position { x: 4, y: 2 }));
        assert!(map.antinodes.contains(&Position { x: 10, y: 11 }));
        assert!(map.antinodes.contains(&Position { x: 3, y: 1 }));
        assert!(map.antinodes.contains(&Position { x: 7, y: 7 }));
        assert!(map.antinodes.contains(&Position { x: 10, y: 10 }));
    }

    #[test]
    fn it_should_not_generate_antinodes_out_of_bounds() {
        let input = "........
                    ..A.....
                    .....A..
                    ........";
        let mut map = Map::parse(&input);
        map.find_antinodes();

        println!("{}", map);

        assert_eq!(map.antinodes, HashSet::new());
    }

    mod part2 {
        use crate::Map;

        #[test]
        fn it_should_do_top_right() {
            let input = "..........
                    ...T......
                    .T........";
            let mut map = Map::parse(&input);
            map.find_antinodes();

            println!("{}", map);
            println!("{:?}", map.antinodes);

            assert_eq!(map.antinodes.len(), 3);
        }

        #[test]
        fn it_should_create_totally_unlimitless_depth() {
            let input = "T.........
                    ...T......
                    .T........
                    ..........
                    ..........
                    ..........
                    ..........
                    ..........
                    ..........
                    ..........";
            let mut map = Map::parse(&input);
            map.find_antinodes();

            println!("{}", map);
            println!("{:?}", map.antinodes);

            assert_eq!(map.antinodes.len(), 9);
        }

        #[test]
        fn it_should_create_many_totally_unlimitless_depth() {
            let input = "............
                    ........0...
                    .....0......
                    .......0....
                    ....0.......
                    ......A.....
                    ............
                    ............
                    ........A...
                    .........A..
                    ............
                    ............";
            let mut map = Map::parse(&input);
            map.find_antinodes();

            println!("{}", map);
            println!("{:?}", map.antinodes);

            assert_eq!(map.antinodes.len(), 34);
        }
    }
}
