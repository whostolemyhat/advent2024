#[cfg(test)]
mod test {
    use crate::{find_solvable, solve, Calibration};

    #[test]
    fn it_should_solve_simple_case() {
        let input = "190: 10 19";
        let calibrations = Calibration::parse(&input);
        assert_eq!(
            solve(
                calibrations[0].target,
                &calibrations[0].nums[1..],
                calibrations[0].nums[0],
                false
            ),
            true
        );
    }

    #[test]
    fn it_should_handle_1() {
        let input = "3358431: 72 52 7 96 873 1";
        let calibrations = Calibration::parse(&input);
        assert_eq!(
            solve(
                calibrations[0].target,
                &calibrations[0].nums[1..],
                calibrations[0].nums[0],
                false
            ),
            true
        );
    }

    #[test]
    fn it_should_not_return_early() {
        let input = "4: 2 2 2";
        let calibrations = Calibration::parse(&input);
        assert_eq!(
            solve(
                calibrations[0].target,
                &calibrations[0].nums[1..],
                calibrations[0].nums[0],
                false
            ),
            false
        );
    }

    #[test]
    fn it_should_handle_three_nums() {
        let input = "3267: 81 40 27";
        let calibrations = Calibration::parse(&input);
        assert_eq!(
            solve(
                calibrations[0].target,
                &calibrations[0].nums[1..],
                calibrations[0].nums[0],
                false
            ),
            true
        );
    }

    #[test]
    fn it_should_handle_four_nums() {
        let input = "292: 11 6 16 20";
        let calibrations = Calibration::parse(&input);
        assert_eq!(
            solve(
                calibrations[0].target,
                &calibrations[0].nums[1..],
                calibrations[0].nums[0],
                false
            ),
            true
        );
    }

    #[test]
    fn it_should_filter_solvable() {
        let input = "190: 10 19
          3267: 81 40 27
          83: 17 5
          156: 15 6
          7290: 6 8 6 15
          161011: 16 10 13
          192: 17 8 14
          21037: 9 7 18 13
          292: 11 6 16 20";
        let calibrations = Calibration::parse(&input);
        let expected: Vec<Calibration> = vec![
            Calibration {
                target: 190,
                nums: vec![10, 19],
            },
            Calibration {
                target: 3267,
                nums: vec![81, 40, 27],
            },
            Calibration {
                target: 292,
                nums: vec![11, 6, 16, 20],
            },
        ];

        assert_eq!(
            find_solvable(&calibrations, false),
            // turn vec into vec<&_>
            expected
                .iter()
                .filter(|_i| true)
                .collect::<Vec<&Calibration>>()
        );
    }

    mod concat {
        use crate::{solve, Calibration};

        #[test]
        fn it_should_filter_using_concat() {
            let input = "156: 15 6";
            let calibrations = Calibration::parse(&input);
            assert_eq!(
                solve(
                    calibrations[0].target,
                    &calibrations[0].nums[1..],
                    calibrations[0].nums[0],
                    true
                ),
                true
            );

            let input = "7290: 6 8 6 15";
            let calibrations = Calibration::parse(&input);
            assert_eq!(
                solve(
                    calibrations[0].target,
                    &calibrations[0].nums[1..],
                    calibrations[0].nums[0],
                    true
                ),
                true
            );

            let input = "192: 17 8 14";
            let calibrations = Calibration::parse(&input);
            assert_eq!(
                solve(
                    calibrations[0].target,
                    &calibrations[0].nums[1..],
                    calibrations[0].nums[0],
                    true
                ),
                true
            );
        }
    }
}
