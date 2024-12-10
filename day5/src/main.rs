use std::{cmp::Ordering, fs::read_to_string, io};

use itertools::Itertools;

fn main() -> Result<(), io::Error> {
    // let input = read_to_string("./input.txt")?;
    // let (rules, values) = input.split("\n\n").collect_tuple().unwrap();

    // let protocol = Protocol {
    //     rules: get_rules(rules),
    //     values: get_values(values),
    // };

    // let total: u32 = protocol.get_total();
    // println!("Total: {}", total);

    let mut protocol = Protocol {
        rules: vec![
            (75, 29),
            (61, 13),
            (75, 53),
            (29, 13),
            (97, 29),
            (53, 29),
            (61, 53),
            (97, 53),
            (61, 29),
            (47, 13),
            (75, 47),
            (97, 75),
            (47, 61),
            (75, 61),
            (47, 29),
            (75, 13),
            (53, 13),
        ],
        values: vec![
            // vec![75, 97, 47, 61, 53],
            // vec![61, 13, 29],
            vec![97, 13, 75, 29, 47],
        ],
    };

    protocol.sort_values();
    protocol.sort_values();
    dbg!(&protocol.values);

    Ok(())
}

struct Protocol {
    rules: Vec<Rule>,
    values: Vec<Vec<u32>>,
}

impl Protocol {
    fn sort_values(&mut self) {
        self.values.iter_mut().for_each(|value| {
            self.rules.iter().for_each(|rule| {
                value.sort_by(|a, b| {
                    dbg!(a, b, rule);
                    if a == &rule.0 && b == &rule.1 {
                        println!("Less");
                        Ordering::Less
                    } else if a == &rule.1 && b == &rule.0 {
                        println!("greater");
                        Ordering::Greater
                    } else {
                        println!("equal");
                        Ordering::Equal
                    }
                });
            });
            println!("sorted {:?}", value);
        });
        // self.values.sort_by(|a, b| a > b);
    }

    fn get_total(&self) -> u32 {
        self.values
            .iter()
            .filter(|v| self.check_rules(v))
            .map(|v| get_middle(v))
            .sum()
    }

    fn check_rules(&self, values: &Vec<u32>) -> bool {
        self.rules.iter().all(|rule| is_valid(rule, &values))
    }
}

type Rule = (u32, u32);

// impl Ord for Rule {
//   fn cmp(&self, other: &Self) -> std::cmp::Ordering {

//   }
// }

fn is_valid(rule: &Rule, update: &Vec<u32>) -> bool {
    let both_present = update.contains(&rule.0) && update.contains(&rule.1);
    if !both_present {
        return true;
    }

    let first_pos = update.iter().position(|i| i == &rule.0);
    let second_pos = update.iter().position(|i| i == &rule.1);

    match (first_pos, second_pos) {
        (None, None) => true,
        (Some(first), Some(second)) => second > first,
        _ => true,
    }
}

fn get_middle(values: &Vec<u32>) -> u32 {
    values[values.len() / 2]
}

fn get_values(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.trim()
                .split(",")
                .map(|v| v.parse::<u32>().unwrap())
                .collect()
        })
        .collect()
}

fn get_rules(input: &str) -> Vec<Rule> {
    input
        .lines()
        .map(|line| {
            line.trim()
                .split("|")
                .map(|rule| rule.parse::<u32>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{get_values, Protocol};

    mod is_valid {
        use crate::is_valid;

        #[test]
        fn is_valid_if_in_correct_order() {
            let rule = (47, 53);
            let update = vec![75, 47, 61, 53, 29];

            // follows rule
            assert_eq!(is_valid(&rule, &update), true);
        }

        #[test]
        fn is_invalid_if_in_incorrect_order() {
            let rule = (47, 53);
            assert_eq!(is_valid(&rule, &vec![75, 53, 61, 47, 29]), false);
        }

        #[test]
        fn is_valid_if_both_missing() {
            let rule = (47, 53);
            assert_eq!(is_valid(&rule, &vec![75, 54, 61, 48, 28]), true);
        }

        #[test]
        fn is_valid_if_one_missing() {
            let rule = (47, 55);
            assert_eq!(is_valid(&rule, &vec![75, 53, 61, 48, 28]), true);
        }
    }

    #[test]
    fn should_parse_values() {
        let input = "69,32,62,98,65,72,59,15,56,89,87
64,35,16,77,86,75,91,26,49,69,89,15,99,59,29,42,56,97,38
26,53,16,35,49,99,86,69,89,15,77,83,42,56,72,47,94,65,59,38,75,29,91
69,98,87,89,73,38,15,76,62,11,61,55,42,83,29,97,99,56,31
47,16,75,91,26,89,97,38,27
47,16,95,55,11,51,34,24,94,79,35,45,57,32,73
33,24,16,31,47,21,73,36,57,86,94
33,79,45,75,95,17,51,94,36,34,25,47,24,55,16,53,86,57,85
27,79,62,98,36
79,36,75,98,21,85,47,33,16
55,31,57,95,35,33,25,47,79,34,21,64,24,36,98,61,11,45,32,76,17
32,27,38,99,89,25,31,61,55,33,56,29,59
16,53,72,49,42,15,97,56,86,26,65,91,69,38,89,64,47
25,17,31,55,33,24,85,45,21,51,95,57,64,35,94,77,86,75,53
49,56,11,87,42,32,99,29,72,38,89,97,61,65,83,25,15,98,73,69,62,76,59
95,85,86,15,53,21,89
45,75,24,94,25,51,85,36,55,47,35,95,33,53,34
57,21,86,16,85
73,79,95,86,21,24,33,34,17,25,64
33,11,31,64,25,17,73,36,21,76,51,34,57,45,98,47,95,16,32
98,25,33,34,95,77,75
51,94,64,95,34,21,35,53,86,24,79
47,16,94,77,86,53,91,26,65,72,89,15,99,59,29,42,56,97,38,83,27
32,99,26,59,65,87,49,29,91,62,56,27,61,75,69,97,72
83,98,62,24,79,76,97,55,85,38,45
49,89,15,29,42,38,62,87,61,76,11,73,31
64,34,49,77,26,65,91,35,59,15,99,16,69,53,51,72,89,47,86
98,17,79,85,95,16,75
59,21,86,99,57,15,29,49,47
73,85,98,76,24,33,32,27,29
45,21,86,75,69
83,97,45,55,76,56,33,42,36,38,25,17,62
77,79,51,72,16,95,57,85,36,64,47,65,75,33,91,21,26
72,26,34,47,59,89,64,95,57,53,94,21,49
53,91,26,72,49,69,89,15,99,59,29,56,97,38,83,27,62,87,61,32,11
35,57,91,26,45,53,99,51,34,77,94,69,65,15,86,75,89,95,16";
        let expected = vec![
            vec![69, 32, 62, 98, 65, 72, 59, 15, 56, 89, 87],
            vec![
                64, 35, 16, 77, 86, 75, 91, 26, 49, 69, 89, 15, 99, 59, 29, 42, 56, 97, 38,
            ],
            vec![
                26, 53, 16, 35, 49, 99, 86, 69, 89, 15, 77, 83, 42, 56, 72, 47, 94, 65, 59, 38, 75,
                29, 91,
            ],
            vec![
                69, 98, 87, 89, 73, 38, 15, 76, 62, 11, 61, 55, 42, 83, 29, 97, 99, 56, 31,
            ],
            vec![47, 16, 75, 91, 26, 89, 97, 38, 27],
            vec![47, 16, 95, 55, 11, 51, 34, 24, 94, 79, 35, 45, 57, 32, 73],
            vec![33, 24, 16, 31, 47, 21, 73, 36, 57, 86, 94],
            vec![
                33, 79, 45, 75, 95, 17, 51, 94, 36, 34, 25, 47, 24, 55, 16, 53, 86, 57, 85,
            ],
            vec![27, 79, 62, 98, 36],
            vec![79, 36, 75, 98, 21, 85, 47, 33, 16],
            vec![
                55, 31, 57, 95, 35, 33, 25, 47, 79, 34, 21, 64, 24, 36, 98, 61, 11, 45, 32, 76, 17,
            ],
            vec![32, 27, 38, 99, 89, 25, 31, 61, 55, 33, 56, 29, 59],
            vec![
                16, 53, 72, 49, 42, 15, 97, 56, 86, 26, 65, 91, 69, 38, 89, 64, 47,
            ],
            vec![
                25, 17, 31, 55, 33, 24, 85, 45, 21, 51, 95, 57, 64, 35, 94, 77, 86, 75, 53,
            ],
            vec![
                49, 56, 11, 87, 42, 32, 99, 29, 72, 38, 89, 97, 61, 65, 83, 25, 15, 98, 73, 69, 62,
                76, 59,
            ],
            vec![95, 85, 86, 15, 53, 21, 89],
            vec![45, 75, 24, 94, 25, 51, 85, 36, 55, 47, 35, 95, 33, 53, 34],
            vec![57, 21, 86, 16, 85],
            vec![73, 79, 95, 86, 21, 24, 33, 34, 17, 25, 64],
            vec![
                33, 11, 31, 64, 25, 17, 73, 36, 21, 76, 51, 34, 57, 45, 98, 47, 95, 16, 32,
            ],
            vec![98, 25, 33, 34, 95, 77, 75],
            vec![51, 94, 64, 95, 34, 21, 35, 53, 86, 24, 79],
            vec![
                47, 16, 94, 77, 86, 53, 91, 26, 65, 72, 89, 15, 99, 59, 29, 42, 56, 97, 38, 83, 27,
            ],
            vec![
                32, 99, 26, 59, 65, 87, 49, 29, 91, 62, 56, 27, 61, 75, 69, 97, 72,
            ],
            vec![83, 98, 62, 24, 79, 76, 97, 55, 85, 38, 45],
            vec![49, 89, 15, 29, 42, 38, 62, 87, 61, 76, 11, 73, 31],
            vec![
                64, 34, 49, 77, 26, 65, 91, 35, 59, 15, 99, 16, 69, 53, 51, 72, 89, 47, 86,
            ],
            vec![98, 17, 79, 85, 95, 16, 75],
            vec![59, 21, 86, 99, 57, 15, 29, 49, 47],
            vec![73, 85, 98, 76, 24, 33, 32, 27, 29],
            vec![45, 21, 86, 75, 69],
            vec![83, 97, 45, 55, 76, 56, 33, 42, 36, 38, 25, 17, 62],
            vec![
                77, 79, 51, 72, 16, 95, 57, 85, 36, 64, 47, 65, 75, 33, 91, 21, 26,
            ],
            vec![72, 26, 34, 47, 59, 89, 64, 95, 57, 53, 94, 21, 49],
            vec![
                53, 91, 26, 72, 49, 69, 89, 15, 99, 59, 29, 56, 97, 38, 83, 27, 62, 87, 61, 32, 11,
            ],
            vec![
                35, 57, 91, 26, 45, 53, 99, 51, 34, 77, 94, 69, 65, 15, 86, 75, 89, 95, 16,
            ],
        ];
        assert_eq!(get_values(&input), expected);
    }

    mod rules {
        use crate::{get_rules, Protocol};

        #[test]
        fn is_valid_with_multiple_rules() {
            let protocol = Protocol {
                rules: vec![
                    (75, 29),
                    (61, 13),
                    (75, 53),
                    (29, 13),
                    (97, 29),
                    (53, 29),
                    (61, 53),
                    (97, 53),
                    (61, 29),
                    (47, 13),
                    (75, 47),
                    (97, 75),
                    (47, 61),
                    (75, 61),
                    (47, 29),
                    (75, 13),
                    (53, 13),
                ],
                values: vec![],
            };
            let update = vec![75, 47, 61, 53, 29];

            assert_eq!(protocol.check_rules(&update), true);
        }

        #[test]
        fn is_invalid_if_in_incorrect_order_multiple_rules() {
            let protocol = Protocol {
                rules: vec![
                    (75, 29),
                    (61, 13),
                    (75, 53),
                    (29, 13),
                    (97, 29),
                    (53, 29),
                    (61, 53),
                    (97, 53),
                    (61, 29),
                    (47, 13),
                    (75, 47),
                    (97, 75),
                    (47, 61),
                    (75, 61),
                    (47, 29),
                    (75, 13),
                    (53, 13),
                ],
                values: vec![],
            };
            let values = vec![47, 75, 47, 61, 53, 29];
            assert_eq!(protocol.check_rules(&values), false);

            let update = vec![97, 13, 75, 29, 47];
            assert_eq!(protocol.check_rules(&update), false);
        }

        #[test]
        fn can_generate_rules_from_input() {
            let input = "75|29
              61|13
              75|53
              29|13
              97|29
              53|29
              61|53
              97|53
              61|29
              47|13
              75|47
              97|75
              47|61
              75|61
              47|29
              75|13
              53|13";
            assert_eq!(
                get_rules(&input),
                vec![
                    (75, 29),
                    (61, 13),
                    (75, 53),
                    (29, 13),
                    (97, 29),
                    (53, 29),
                    (61, 53),
                    (97, 53),
                    (61, 29),
                    (47, 13),
                    (75, 47),
                    (97, 75),
                    (47, 61),
                    (75, 61),
                    (47, 29),
                    (75, 13),
                    (53, 13)
                ]
            );
        }
    }

    #[test]
    fn should_calculate_total() {
        let protocol = Protocol {
            values: vec![
                vec![75, 47, 61, 53, 29],
                vec![97, 61, 53, 29, 13],
                vec![75, 29, 13],
            ],

            rules: vec![
                (75, 29),
                (61, 13),
                (75, 53),
                (29, 13),
                (97, 29),
                (53, 29),
                (61, 53),
                (97, 53),
                (61, 29),
                (47, 13),
                (75, 47),
                (97, 75),
                (47, 61),
                (75, 61),
                (47, 29),
                (75, 13),
                (53, 13),
            ],
        };
        assert_eq!(protocol.get_total(), 143);
    }

    #[test]
    fn it_should_sort_based_on_rules() {
        let protocol = Protocol {
            rules: vec![
                (75, 29),
                (61, 13),
                (75, 53),
                (29, 13),
                (97, 29),
                (53, 29),
                (61, 53),
                (97, 53),
                (61, 29),
                (47, 13),
                (75, 47),
                (97, 75),
                (47, 61),
                (75, 61),
                (47, 29),
                (75, 13),
                (53, 13),
            ],
            values: vec![
                vec![75, 47, 61, 53, 29],
                vec![97, 61, 53, 29, 13],
                vec![75, 29, 13],
            ],
        };

        protocol.sort_values();
    }
}
