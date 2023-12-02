use regex::Regex;
use std::fs;

fn main() {
    let data = read_file("src/day01.txt");

    let part1: i32 = calibration(data.lines());
    println!("The sum of the calibration values is: {part1}");

    let part2: i32 = calibration_extended(data.lines());
    println!("The sum of the calibration values is: {part2}");
}

pub fn calibration(lines: std::str::Lines<'_>) -> i32 {
    lines.map(|l| combine_digits(l)).sum()
}

pub fn calibration_extended(lines: std::str::Lines<'_>) -> i32 {
    lines.map(|l| combine_digits_extended(l)).sum()
}

pub fn combine_digits(line: &str) -> i32 {
    let reg1 = Regex::new(r"^[^\d]*(?<n>\d).*$").unwrap();
    let reg2 = Regex::new(r".*(?<n>\d)").unwrap();
    let first = reg1.captures(line).unwrap();
    let last = reg2.captures(line).unwrap();

    (first["n"].to_string() + &last["n"]).parse().unwrap()
}

pub fn combine_digits_extended(line: &str) -> i32 {
    let reg1 = Regex::new(r"(?<n>one|two|three|four|five|six|seven|eight|nine|\d).*").unwrap();
    let reg2 = Regex::new(r".*(?<n>one|two|three|four|five|six|seven|eight|nine|\d)").unwrap();
    let first = reg1.captures(line).unwrap();
    let last = reg2.captures(line).unwrap();

    (to_digit(&first["n"]).to_string() + to_digit(&last["n"])).parse().unwrap()
}

pub fn to_digit(value: &str) -> &str {
    match value {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        _ => value
    }
}

fn read_file(filename: &str) -> String {
    fs::read_to_string(filename).expect("Failed to read file!")
}

#[cfg(test)]
mod test {
    use crate::{read_file, calibration, calibration_extended};

    #[test]
    fn part1() {
        let data = read_file("src/test01.txt");
        assert_eq!(calibration(data.lines()), 142);
    }

    #[test]
    fn part2() {
        let data = read_file("src/test02.txt");
        assert_eq!(calibration_extended(data.lines()), 281);
    }
}
