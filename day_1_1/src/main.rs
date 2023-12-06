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
    let mut first_digit: u32 = 0;
    let mut second_digit: u32 = 0;

    if !line.chars().count() == 0 {
        return 0;
    }

    for char in line.chars() {
        if char.is_numeric() {
            first_digit = char.to_digit(10).expect("Unable to parse first digit!");
            break;
        }
    }

    for char in line.chars().rev() {
        if char.is_numeric() {
            second_digit = char.to_digit(10).expect("Unable to parse second digit!");
            break;
        }
    }
    
    let result = (first_digit * 10) + second_digit;
    print!("{}\n", result);

    return result;
    
}