use crate::day5::DAY5_TEST_DATA;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

fn main() {
    println!("day1: {}", day1::day1());
    println!("day1_part2: {}", day1::day1_part2());
    println!("day2: {}", day2::day2());
    println!("day2_part2: {}", day2::day2_part2());
    println!("day3: {}", day3::day3());
    println!("day3_part2: {}", day3::day3_part2());
    println!("day4: {}", day4::day4());
    println!("day4_part2: {}", day4::day4_part2());
    println!("day5: {}", day5::day5(DAY5_TEST_DATA.to_string()));
    println!("daty5: {}", day5::day5_part2(DAY5_TEST_DATA.to_string()));
 }
