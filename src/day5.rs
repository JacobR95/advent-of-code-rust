use std::{fs, ops::Index};

pub const DAY5_TEST_DATA: &str = "[D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

pub fn day5(data: String) -> &'static str {

    let result = get_stacks_from(&data);
    // data.lines().for_each(|line| {

    // });

    return "CMZ";
}

// --- Part Two ---
pub fn day5_part2(data: String) -> &'static str {

    // data.lines().for_each(|line| {
    // });

    return "CMZ";
}

pub fn get_stacks_from<'a>(crates: &'a str) -> Vec<Vec<&'a str>> {
    let mut result = Vec::new();

    for line in crates.lines() {
        if !line.starts_with("[") {
            break;
        }

        let trimmed_line = line.trim_matches(|c: char| c == '[' || c == ']');
        let row = trimmed_line.split_whitespace().collect::<Vec<&'a str>>();
        result.push(row);
    }

    return result;
}

pub fn get_moves_from(data: String) -> Vec<(u16, u16, u16)> {
    let mut moves: Vec<(u16, u16, u16)> = vec![];

    let empty_line_index = data
        .lines()
        .enumerate()
        .find(|&(_, line)| line.trim().is_empty())
        .map(|(index, _)| index)
        .unwrap();

    for line in data.lines().skip(empty_line_index + 1) {
        let trimmed_line = line
            .replace("move ", "")
            .replace(" from ", ",")
            .replace(" to ", ",");

        let numbers: Vec<&str> = trimmed_line.split(",").collect();

        moves.push((
            numbers[0].parse::<u16>().unwrap(),
            numbers[1].parse::<u16>().unwrap(),
            numbers[2].parse::<u16>().unwrap(),
        ));
    }

    return moves;
}

#[cfg(test)]
mod day5_tests {
    use super::*;

    #[test]
    fn should_return_expected_amount_with_example() {
        let data = String::from(DAY5_TEST_DATA);

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
        let data = String::from(DAY5_TEST_DATA);

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

#[cfg(test)]
mod day5_utils_tests {
    use super::*;

    #[test]
    fn should_return_correct_moves() {
        let data = String::from(DAY5_TEST_DATA);
        let expected: Vec<(u16, u16, u16)> = vec![
            (1, 2, 1),
            (3, 1, 3),
            (2, 2, 1),
            (1, 1, 2),
        ];

        let actual = get_moves_from(data);

        assert_eq!(actual, expected);
    }
}
