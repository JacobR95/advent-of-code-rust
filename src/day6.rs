pub fn day6(data: &str) -> u32 {

    let result: u32 = 7;
    return result;
}

pub fn day6_part(data: &str) -> u32 {

    let result: u32 = 0;
    return result;
}


#[cfg(test)]
mod day6_utils_tests {
    use super::*;

    #[test]
    fn should_return_expected_for_example_data() {

        let example1 = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let example2 = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let example3 = "nppdvjthqldpwncqszvftbrmjlhg";
        let example4 = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let example5 = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

        assert_eq!(day6(example1), 7);
        assert_eq!(day6(example2), 5);
        assert_eq!(day6(example3), 6);
        assert_eq!(day6(example4), 10);
        assert_eq!(day6(example5), 11);
    }

    #[test]
    fn should_return_expected_for_input_data() {
        let data = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let expected = 7;

        let actual = day6(&data);

        assert_eq!(actual, expected);
    }
}
