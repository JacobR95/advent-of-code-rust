pub const DAY5_TEST_DATA: &str = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

pub fn day5(data: String) -> String {

    let mut stacks = get_stacks_from(&data);
    let moves = get_moves_from(&data);

    for value in moves {
        let (amount, from, to) = value;

        for _ in 0..amount {
            let value = stacks[from-1].remove(0);
            stacks[to-1].insert(0, value);
        }
    }

    return stacks.iter()
        .map(|stack| stack[0].chars().nth(0).unwrap())
        .collect();
}

// --- Part Two ---
pub fn day5_part2(data: String) -> String {

    let mut stacks = get_stacks_from(&data);
    let moves = get_moves_from(&data);

    for (amount, from, to) in moves {
        let moved_crates: Vec<_> = stacks[from-1].drain(0..amount).collect();
        for c in moved_crates.iter().rev() {
            stacks[to-1].insert(0, c.to_string());
        }
    }

    return stacks.iter()
        .map(|stack| stack[0].chars().nth(0).unwrap())
        .collect();
}

pub fn get_stacks_from<'a>(crates: &'a str) -> Vec<Vec<String>> {

    let total_stacks = crates
        .lines()
        .find(|line| line.starts_with(" 1"))
        .unwrap()
        .replace(" ", "")
        .len();

    let mut stacks = vec![vec![];total_stacks];

    for line in crates.lines() {
        if line.starts_with(" 1") {
            break;
        }

        for stack in 0..total_stacks {
            let index = (stack*4)+1;
            match line.chars().nth(index) {
                Some(c) => {
                    if c != ' ' {
                        stacks[stack].push(c.to_string())
                    }
                },
                None => {}
            };
        }
    }

    return stacks;
}

pub fn get_moves_from(data: &str) -> Vec<(usize, usize, usize)> {
    let mut moves: Vec<(usize, usize, usize)> = vec![];

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
            numbers[0].parse::<usize>().unwrap(),
            numbers[1].parse::<usize>().unwrap(),
            numbers[2].parse::<usize>().unwrap(),
        ));
    }

    return moves;
}

#[cfg(test)]
mod day5_tests {
    use std::fs;

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
        assert_eq!(result, "GRTSWNJHH");
    }

    #[test]
    fn should_return_expected_amount_for_part_2_with_example() {
        let data = String::from(DAY5_TEST_DATA);
        let result = day5_part2(data);
        assert_eq!(result, "MCD");
    }

    #[test]
    fn should_return_expected_amount_for_part_2_with_input_data() {
        let data = fs::read_to_string("data/day5.txt").unwrap();
        let result = day5_part2(data);
        assert_eq!(result, "QLFQDBBHM");
    }
}

#[cfg(test)]
mod day5_utils_tests {
    use super::*;

    #[test]
    fn should_return_correct_moves() {
        let data = String::from(DAY5_TEST_DATA);
        let expected: Vec<(usize, usize, usize)> = vec![
            (1, 2, 1),
            (3, 1, 3),
            (2, 2, 1),
            (1, 1, 2),
        ];

        let actual = get_moves_from(&data);

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_return_correct_stacks() {
        let data = String::from(DAY5_TEST_DATA);
        let expected = vec![
            vec!["N", "Z"],
            vec!["D", "C", "M"],
            vec!["P"],
        ];

        let actual = get_stacks_from(&data);

        assert_eq!(actual, expected);
    }
}
