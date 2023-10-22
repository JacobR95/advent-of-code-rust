use std::fs;

pub fn day5(data: String) -> &'static str {

    data.lines().for_each(|line| {
    });

    return "CMZ";
}

// --- Part Two ---
pub fn day5_part2(data: String) -> &'static str {

    data.lines().for_each(|line| {
    });

    return "CMZ";
}

#[cfg(test)]
mod day5_tests {
    use super::*;

    #[test]
    fn should_return_expected_amount_with_example() {
        let data = String::from("[D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2");

        let result = day5(data);
        assert_eq!(result, "CMZ");
    }

    #[test]
    fn should_return_expected_result_with_input_data() {
        let data = fs::read_to_string("data/day5.txt").unwrap();
        let result = day5(data);
        assert_eq!(result, "CMZ");
    }

    #[test]
    fn should_return_expected_amount_for_part_2_with_example() {
        let data = String::from("[D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2");

        let result = day5_part2(data);
        assert_eq!(result, "CMZ");
    }

    #[test]
    fn should_return_expected_amount_for_part_2_with_input_data() {
        let data = fs::read_to_string("data/day5.txt").unwrap();
        let result = day5_part2(data);
        assert_eq!(result, "CMZ");
    }
}
