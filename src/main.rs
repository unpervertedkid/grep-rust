use std::env;
use std::io;
use std::process;

fn match_pattern(input_line: &str, pattern: &str) -> bool {
    if pattern.chars().count() == 1 {
        return input_line.contains(pattern);
    } else {
        match pattern {
            r"\d" => return match_single_digits(input_line),
            r"\w" => return match_alphanumeric(input_line),
            _ => panic!("Unhandled pattern: {}", pattern),
        }
    }
}

fn match_single_digits(input_line: &str) -> bool {
    let mut matched = false;

    for c in input_line.chars() {
        if c.is_digit(10) {
            matched = true;
            break;
        }
    }

    matched
}

fn match_alphanumeric(input_line: &str) -> bool {
    let mut matched = false;

    for c in input_line.chars() {
        if c.is_alphanumeric() {
            matched = true;
            break;
        }
    }

    matched
}

// Usage: echo <input_text> | your_grep.sh -E <pattern>
fn main() {
    if env::args().nth(1).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let pattern = env::args().nth(2).unwrap();
    let mut input_line = String::new();

    io::stdin().read_line(&mut input_line).unwrap();

    if match_pattern(&input_line, &pattern) {
        process::exit(0)
    } else {
        process::exit(1)
    }
}
