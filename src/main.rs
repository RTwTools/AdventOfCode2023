use regex::Regex;
use std::fs;

fn main() {
    let data = read_file("src/day01.txt");
    let answer1: i32 = calibration_value(data.lines());
    println!("The sum of the calibration values is: {answer1}");
}

pub fn calibration_value(lines: std::str::Lines<'_>) -> i32 {
    lines.map(|l| combine_first_and_last_digit(l)).sum()
}

pub fn combine_first_and_last_digit(line: &str) -> i32 {
    let reg1 = Regex::new(r"^[^\d]*(?<n>\d).*$").unwrap();
    let reg2 = Regex::new(r".*(?<n>\d)").unwrap();
    let first = reg1.captures(line).unwrap();
    let last = reg2.captures(line).unwrap();

    (first["n"].to_string() + &last["n"]).parse().unwrap()
}

fn read_file(filename: &str) -> String {
    fs::read_to_string(filename).expect("Failed to read file!")
}

#[cfg(test)]
mod test {
    use crate::{read_file, calibration_value};

    #[test]
    fn test() {
        let data = read_file("src/test01.txt");
        assert_eq!(calibration_value(data.lines()), 142);
    }
}
