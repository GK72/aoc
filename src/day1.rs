use std::fs;

fn first_digit(s: &str) -> char {
    for ch in s.chars() {
        if ch.is_digit(10) {
            return ch;
        }
    }

    panic!("No digit found in string");
}

fn last_digit(s: &str) -> char {
    for ch in s.chars().rev() {
        if ch.is_digit(10) {
            return ch;
        }
    }

    panic!("No digit found in string");
}

fn process_line(line: &str) -> String {
    return format!("{}{}", first_digit(line), last_digit(line));
}

pub fn run() {
    let data = fs::read_to_string("res/day1.txt").unwrap();

    let xs = data.lines().into_iter().map(|line| process_line(line).parse::<i32>().unwrap());
    let sum : i32 = xs.sum();
    println!("Sum: {}", sum);
}
