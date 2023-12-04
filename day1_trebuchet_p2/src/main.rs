use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;

fn calculate_calibration_value(line: &str) -> u32 {
    let re = Regex::new(r"[1-9]|one|two|three|four|five|six|seven|eight|nine").unwrap();
    let re_greed = Regex::new(r".*([1-9]|one|two|three|four|five|six|seven|eight|nine)").unwrap();

    let matches: Vec<_> = re.find_iter(line).map(|m| m.as_str()).collect();
    let caps = re_greed.captures(line).unwrap();
    let substr1 = caps.get(1).map_or("", |m| m.as_str());

    let first = match matches[0] {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        _ => matches[0],
    };

    let last = match substr1 {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        _ => substr1,
    };

    let mut firstlast: String = first.to_owned();
    firstlast.push_str(last);
    let num: u32 = firstlast.parse().unwrap();

    num
}

fn main() {
    let file = File::open("input.txt").expect("Failed to open file");

    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .filter_map(|line| line.ok())
        .collect();

    let sum: u32 = lines.iter().map(|line| calculate_calibration_value(line)).sum();

    println!("Sum of calibration values: {}", sum);
}
