use std::collections::HashSet;

pub fn day6(data: &str, window_size: usize) -> u32 {

    let mut result: u32 = 0;

    for start in 0..data.len() {

        let end = start + window_size;
        let mut window = HashSet::new();

        for index in start..end {
            let letter = data.chars().nth(index).unwrap();

            if window.contains(&letter) {
                break;
            }

            window.insert(letter);
        }

        if window.len() == window_size {
            result = end as u32;
            break;
        }
    }

    return result;
}

#[cfg(test)]
mod day6_utils_tests {
    use std::fs;
    use super::*;

    const WINDOW_SIZE_PART1: usize = 4;
    const WINDOW_SIZE_PART2: usize = 14;

    #[test]
    fn should_return_expected_for_example_data() {

        let example1 = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let example2 = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let example3 = "nppdvjthqldpwncqszvftbrmjlhg";
        let example4 = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let example5 = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

        assert_eq!(day6(example1, WINDOW_SIZE_PART1), 7);
        assert_eq!(day6(example2, WINDOW_SIZE_PART1), 5);
        assert_eq!(day6(example3, WINDOW_SIZE_PART1), 6);
        assert_eq!(day6(example4, WINDOW_SIZE_PART1), 10);
        assert_eq!(day6(example5, WINDOW_SIZE_PART1), 11);
    }

    #[test]
    fn should_return_expected_for_input_data() {
        let data = fs::read_to_string("data/day6.txt").unwrap();
        let expected = 1896;

        let actual = day6(&data, WINDOW_SIZE_PART1);

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_return_expected_for_example_data_part2() {

        let example1 = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let example2 = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let example3 = "nppdvjthqldpwncqszvftbrmjlhg";
        let example4 = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let example5 = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

        assert_eq!(day6(example1, WINDOW_SIZE_PART2), 19);
        assert_eq!(day6(example2, WINDOW_SIZE_PART2), 23);
        assert_eq!(day6(example3, WINDOW_SIZE_PART2), 23);
        assert_eq!(day6(example4, WINDOW_SIZE_PART2), 29);
        assert_eq!(day6(example5, WINDOW_SIZE_PART2), 26);
    }

    #[test]
    fn should_return_expected_for_input_data_part2() {
        let data = fs::read_to_string("data/day6.txt").unwrap();
        let expected = 3452;

        let actual = day6(&data, WINDOW_SIZE_PART2);

        assert_eq!(actual, expected);
    }
}
