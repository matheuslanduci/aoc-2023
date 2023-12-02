use std::fs;

use regex::{Regex, RegexSet};

const NUMBERS: [(&str, &str); 9] = [
    ("one", "1"),
    ("two", "2"),
    ("three", "3"),
    ("four", "4"),
    ("five", "5"),
    ("six", "6"),
    ("seven", "7"),
    ("eight", "8"),
    ("nine", "9"),
];

// It doesn't work 😥

fn full_to_digit(input: &str) -> &str {
    let (_, value) = NUMBERS
        .iter()
        .find(|&&x| {
            let (identifier, _) = x;

            return identifier == input;
        })
        .unwrap();

    return value;
}

fn main() {
    let file = fs::read_to_string("./input").expect("The file wasn't found");

    let patterns = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "[1-9]",
    ];

    let set = RegexSet::new(patterns).unwrap();

    let regexes: Vec<_> = set
        .patterns()
        .iter()
        .map(|pattern| Regex::new(pattern).unwrap())
        .collect();

    let result: i32 = file
        .lines()
        .map(|line| {
            let matches: Vec<&str> = set
                .matches(line)
                .into_iter()
                .map(|index| &regexes[index])
                .map(|regex| {
                    let value = regex.find(line).unwrap().as_str();

                    let first_char = value.chars().next().unwrap();

                    if first_char.is_numeric() {
                        return value;
                    }

                    return full_to_digit(value);
                })
                .rev()
                .collect();

            let mut digits = matches.first().unwrap_or(&"").to_string();

            digits.push_str(matches.last().unwrap_or(&""));

            return digits.parse::<i32>().unwrap_or(0);
        })
        .sum();

    println!("The result is: {}", result);
}
