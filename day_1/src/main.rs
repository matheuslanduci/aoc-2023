use std::fs;

fn main() {
    let file = fs::read_to_string("./input").expect("The file wasn't found");

    let result: i32 = file
        .lines()
        .map(|line| {
            let matches: Vec<_> = line.matches(char::is_numeric).collect();

            let mut digits = matches.first().unwrap_or(&"").to_string();

            digits.push_str(matches.last().unwrap_or(&""));

            return digits.parse::<i32>().unwrap_or(0);
        })
        .sum();

    println!("The result is: {}", result);
}
