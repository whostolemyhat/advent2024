#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::{checksum, Program};

    #[test]
    fn it_should_parse() {
        let input = "2333133121414131402";
        let programmes = Program::parse(&input);
        let mut spaces = HashMap::new();
        spaces.insert(1, vec![7, 9, 11, 13, 15]);
        spaces.insert(0, vec![17]);
        spaces.insert(3, vec![1, 3, 5]);

        assert_eq!(
            programmes,
            Program {
                // blocks: String::from("00...111...2...333.44.5555.6666.777.888899"),
                free_count: 14,
                list: vec![
                    "0", "0", ".", ".", ".", "1", "1", "1", ".", ".", ".", "2", ".", ".", ".", "3",
                    "3", "3", ".", "4", "4", ".", "5", "5", "5", "5", ".", "6", "6", "6", "6", ".",
                    "7", "7", "7", ".", "8", "8", "8", "8", "9", "9"
                ]
                .iter()
                .map(|s| s.to_string())
                .collect(),
                spaces
            }
        );

        let input = "12345";
        let programmes = Program::parse(&input);
        let mut spaces = HashMap::new();

        spaces.insert(2, vec![1]);
        spaces.insert(4, vec![3]);

        assert_eq!(
            programmes,
            Program {
                free_count: 6,
                list: vec![
                    "0", ".", ".", "1", "1", "1", ".", ".", ".", ".", "2", "2", "2", "2", "2"
                ]
                .iter()
                .map(|s| s.to_string())
                .collect(),
                spaces
            }
        );
    }

    #[test]
    fn it_should_frag() {
        let input = "12345";
        let mut programme = Program::parse(&input);
        programme.frag();

        assert_eq!(
            programme.list,
            vec![
                "0".to_string(),
                "2".to_string(),
                "2".to_string(),
                "1".to_string(),
                "1".to_string(),
                "1".to_string(),
                "2".to_string(),
                "2".to_string(),
                "2".to_string(),
                ".".to_string(),
                ".".to_string(),
                ".".to_string(),
                ".".to_string(),
                ".".to_string(),
                ".".to_string()
            ]
        );

        let input = "2333133121414131402";
        let mut programme = Program::parse(&input);
        programme.frag();

        assert_eq!(
            programme.list,
            vec![
                "0".to_string(),
                "0".to_string(),
                "9".to_string(),
                "9".to_string(),
                "8".to_string(),
                "1".to_string(),
                "1".to_string(),
                "1".to_string(),
                "8".to_string(),
                "8".to_string(),
                "8".to_string(),
                "2".to_string(),
                "7".to_string(),
                "7".to_string(),
                "7".to_string(),
                "3".to_string(),
                "3".to_string(),
                "3".to_string(),
                "6".to_string(),
                "4".to_string(),
                "4".to_string(),
                "6".to_string(),
                "5".to_string(),
                "5".to_string(),
                "5".to_string(),
                "5".to_string(),
                "6".to_string(),
                "6".to_string(),
                ".".to_string(),
                ".".to_string(),
                ".".to_string(),
                ".".to_string(),
                ".".to_string(),
                ".".to_string(),
                ".".to_string(),
                ".".to_string(),
                ".".to_string(),
                ".".to_string(),
                ".".to_string(),
                ".".to_string(),
                ".".to_string(),
                ".".to_string()
            ]
        );
    }

    #[test]
    fn calc_checksum() {
        let input = "0099811188827773336446555566.............."
            .split("")
            .map(|s| String::from(s))
            .filter(|s| s.len() != 0)
            .collect::<Vec<String>>();
        assert_eq!(checksum(&input), 1928);
    }

    #[test]
    fn checksum_should_handle_double_digit_index() {
        let input = vec![
            "0", "10", "9", "9", "11", "1", "8", "8", "8", "21", "1", "6", "5", "5", "5", ".", ".",
            ".", ".", ".", ".",
        ]
        .iter()
        .map(|s| String::from(*s))
        .filter(|s| s.len() != 0)
        .collect::<Vec<String>>();
        assert_eq!(checksum(&input), 732);
    }

    #[test]
    fn it_should_defrag() {
        let input = "2333133121414131402";
        let mut programme = Program::parse(&input);
        programme.defrag();
    }
}
