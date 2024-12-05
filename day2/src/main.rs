use std::{fs::read_to_string, io};

fn part_one() -> Result<usize, io::Error> {
    let safe_reports: Vec<SafetyLevel> = read_to_string("./input.txt")?
        .lines()
        .map(|line| {
            let levels = line
                .split_whitespace()
                .map(|item| item.parse::<u32>().expect("Failed to parse"))
                .collect::<Vec<u32>>();

            check_levels_with_dampening(&levels)
        })
        .filter(|item| *item == SafetyLevel::Safe)
        .collect();

    Ok(safe_reports.len())
}

fn part_two() -> Result<usize, io::Error> {
    let safe_reports: Vec<SafetyLevel> = read_to_string("./input.txt")?
        .lines()
        .map(|line| {
            let levels = line
                .split_whitespace()
                .map(|item| item.parse::<u32>().expect("Failed to parse"))
                .collect::<Vec<u32>>();

            check_levels_with_dampening(&levels)
        })
        .filter(|item| *item == SafetyLevel::Safe)
        .collect();

    Ok(safe_reports.len())
}

fn main() -> Result<(), io::Error> {
    let _part_one_result = part_one();
    let part_two_result = part_two();

    println!("Part 2 total: {:?}", part_two_result);

    Ok(())
}

#[derive(PartialEq, Debug)]
enum SafetyLevel {
    Safe,
    Unsafe,
}

fn check_levels(input: &Vec<u32>) -> SafetyLevel {
    let max_difference = 3;

    let is_valid_increasing = input
        .iter()
        .zip(input.iter().skip(1))
        .all(|(current, next)| current < next && next - current <= max_difference);

    // early return
    if is_valid_increasing {
        return SafetyLevel::Safe;
    }

    let is_valid_decreasing = input
        .iter()
        .zip(input.iter().skip(1))
        .all(|(current, next)| current > next && current - next <= max_difference);

    if is_valid_decreasing {
        return SafetyLevel::Safe;
    }

    SafetyLevel::Unsafe
}

fn check_levels_with_dampening(input: &Vec<u32>) -> SafetyLevel {
    let max_difference = 3;
    // let mut error_count = 0;
    let mut error_pair = (0, 0);

    let is_valid_increasing = input
        .iter()
        .enumerate()
        // all short-circuits
        .all(|(index, current)| {
            if index + 1 < input.len() {
                if current < &input[index + 1] && input[index + 1] - current <= max_difference {
                    return true;
                } else {
                    // error_count += 1;
                    error_pair = (index, index + 1);
                    return false;
                }
            }
            return true;
        });

    if is_valid_increasing {
        return SafetyLevel::Safe;
    }

    let mut without_first = input.clone();
    without_first.remove(error_pair.0);
    let mut without_second = input.clone();
    without_second.remove(error_pair.1);

    let increasing_dampened = check_levels(&without_first) == SafetyLevel::Safe
        || check_levels(&without_second) == SafetyLevel::Safe;

    // if any of these pass, it's safe
    if increasing_dampened {
        return SafetyLevel::Safe;
    }

    error_pair = (0, 0);
    let is_valid_decreasing = input
        .iter()
        .enumerate()
        // all short-circuits
        .all(|(index, current)| {
            if index + 1 < input.len() {
                if current > &input[index + 1] && current - input[index + 1] <= max_difference {
                    return true;
                } else {
                    // error_count += 1;
                    error_pair = (index, index + 1);
                    return false;
                }
            }
            return true;
        });

    if is_valid_decreasing {
        return SafetyLevel::Safe;
    }

    without_first = input.clone();
    without_first.remove(error_pair.0);
    without_second = input.clone();
    without_second.remove(error_pair.1);

    if check_levels(&without_first) == SafetyLevel::Safe {
        return SafetyLevel::Safe;
    }
    return check_levels(&without_second);
}

#[cfg(test)]
mod test {
    mod check_levels {
        use crate::{check_levels, SafetyLevel};

        #[test]
        fn returns_safe_when_levels_all_decrease() {
            assert_eq!(check_levels(&vec![7, 6, 4, 2, 1]), SafetyLevel::Safe);
        }

        #[test]
        fn returns_unsafe_with_large_increase() {
            assert_eq!(check_levels(&vec![1, 2, 7, 8, 9]), SafetyLevel::Unsafe);
        }

        #[test]
        fn returns_unsafe_with_large_decrease() {
            assert_eq!(check_levels(&vec![9, 7, 6, 2, 1]), SafetyLevel::Unsafe);
        }

        #[test]
        fn returns_unsafe_with_decrease_in_increasing_pattern() {
            assert_eq!(check_levels(&vec![1, 3, 2, 4, 5]), SafetyLevel::Unsafe);
        }

        #[test]
        fn returns_safe_when_all_levels_increase() {
            assert_eq!(check_levels(&vec![1, 3, 6, 7, 9]), SafetyLevel::Safe);
            assert_eq!(check_levels(&vec![11, 12, 13, 15, 18]), SafetyLevel::Safe);
        }

        #[test]
        fn returns_unsafe_with_equal_entries() {
            assert_eq!(check_levels(&vec![8, 6, 4, 4, 1]), SafetyLevel::Unsafe);
        }
    }

    mod dampening {
        use crate::{check_levels_with_dampening, SafetyLevel};

        #[test]
        fn returns_safe_with_increasing() {
            assert_eq!(
                check_levels_with_dampening(&vec![1, 3, 6, 7, 9]),
                SafetyLevel::Safe
            );
        }

        #[test]
        fn returns_safe_with_decreasing() {
            assert_eq!(
                check_levels_with_dampening(&vec![7, 6, 4, 2, 1]),
                SafetyLevel::Safe
            );
        }

        #[test]
        fn returns_unsafe_with_increasing_and_more_than_one_error() {
            assert_eq!(
                check_levels_with_dampening(&vec![1, 2, 7, 8, 9]),
                SafetyLevel::Unsafe
            );
        }

        #[test]
        fn returns_unsafe_with_decreasing_and_more_than_one_error() {
            assert_eq!(
                check_levels_with_dampening(&vec![9, 7, 6, 2, 1]),
                SafetyLevel::Unsafe
            );
        }

        #[test]
        fn returns_safe_with_increasing_and_one_error() {
            assert_eq!(
                check_levels_with_dampening(&vec![1, 3, 2, 4, 5]),
                SafetyLevel::Safe
            );
            assert_eq!(
                check_levels_with_dampening(&vec![48, 46, 47, 49, 51, 54, 56]),
                SafetyLevel::Safe
            );
            assert_eq!(
                check_levels_with_dampening(&vec![1, 1, 2, 3, 4, 5]),
                SafetyLevel::Safe
            );
            assert_eq!(
                check_levels_with_dampening(&vec![1, 2, 3, 4, 5, 5]),
                SafetyLevel::Safe
            );
            assert_eq!(
                check_levels_with_dampening(&vec![5, 1, 2, 3, 4, 5]),
                SafetyLevel::Safe
            );
            assert_eq!(
                check_levels_with_dampening(&vec![1, 6, 7, 8, 9]),
                SafetyLevel::Safe
            );
            assert_eq!(
                check_levels_with_dampening(&vec![1, 2, 3, 4, 3]),
                SafetyLevel::Safe
            );
            assert_eq!(
                check_levels_with_dampening(&vec![7, 10, 8, 10, 11]),
                SafetyLevel::Safe
            );
        }

        #[test]
        fn returns_safe_with_decreasing_and_one_error() {
            assert_eq!(
                check_levels_with_dampening(&vec![8, 6, 4, 4, 1]),
                SafetyLevel::Safe
            );
            assert_eq!(
                check_levels_with_dampening(&vec![29, 28, 27, 25, 26, 25, 22, 20]),
                SafetyLevel::Safe
            );
            assert_eq!(
                check_levels_with_dampening(&vec![9, 8, 7, 6, 7]),
                SafetyLevel::Safe
            );
            assert_eq!(
                check_levels_with_dampening(&vec![1, 4, 3, 2, 1]),
                SafetyLevel::Safe
            );
        }
    }
}
