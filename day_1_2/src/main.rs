use std::{fs::File, io::{self, BufRead}};
use std::path::Path;

fn main() {
    let input_path = "input.txt";

    let mut total : u32 = 0;
    if let Ok(lines) = read_lines(input_path) {
        for line in lines {
            if let Ok(line_value) = line {
                total += process_line(line_value);
            }
        }

        println!("Total: {}", total);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    return Ok(io::BufReader::new(file).lines());
}

fn process_line(line: String) -> u32 {
    let mut idx: usize = 0; 
    let mut match_idx: u32 = 0;
    let mut first_digit: u32 = 0;
    let mut last_digit: u32 = 0;

    while idx < line.chars().count() {
        
        let end_idx: usize; 
        if idx + MAX_LENGTH >= line.chars().count() {
            end_idx = line.chars().count();
        } else {
            end_idx = idx + MAX_LENGTH;
        } 

        let slice = &line[idx..end_idx];
        for search in SEARCH_STRINGS {
            if slice.starts_with(search) {
                let score = get_score(&search);

                if match_idx == 0 {
                    first_digit = score;
                    last_digit = score;
                } else {
                    last_digit = score;
                }

                match_idx += 1;
                break;
            }
        }

        idx += 1;
    }

    let total = (first_digit * 10) + last_digit;
    println!("{}", total);

    return total;
}

fn get_score(value: &str) -> u32 {
    match value {
        "1" | "one" => 1,
        "2" | "two" => 2,
        "3" | "three" => 3,
        "4" | "four" => 4,
        "5" | "five" => 5,
        "6" | "six" => 6,
        "7" | "seven" => 7,
        "8" | "eight" => 8,
        "9" | "nine" => 9,
        _ => 0
    }
}

// Longest string value in the array
const MAX_LENGTH : usize = 5;

// Strings to search for
const SEARCH_STRINGS: &[&str] = &[
    "1", "2", "3", "4", "5", "6", "7", "8", "9",
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"    
];