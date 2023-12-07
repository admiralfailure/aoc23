use std::{fs::File, io::{self, BufRead}};
use std::path::Path;

fn main() {
    let input_path = "input.txt";

    let mut total : u32 = 0;
    if let Ok(lines) = read_lines(input_path) {
        for line in lines {
            if let Ok(line_value) = line {
                total += process_line(&line_value);
            }
        }

        println!("Total: {}", total);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    return Ok(io::BufReader::new(file).lines());
}

const MAX_REDS: u32 = 12;
const MAX_GREENS: u32 = 13;
const MAX_BLUES: u32 = 14;

fn process_line(line: &str) -> u32 {
    let index: u32;
    let mut most_reds: u32 = 0;
    let mut most_greens: u32 = 0;
    let mut most_blues: u32 = 0;

    // Split on : to get the game index
    let parts: Vec<&str> = line.split(":").collect();
    let game: &str = parts.first().expect("Unable to get game info from line");
    let game_parts: Vec<&str> = game.split_whitespace().collect();
    index = game_parts.get(1).expect("Unable to get game index").parse::<u32>().expect("Unable to parse index");
    
    // Split on ; to get each group
    let groups = parts[1].split(";");
    for group in groups {
        // Split on , to get each value
        let sets = group.split(",");
        for set in sets {
            // Compare to current highest values
            let mut set_parts = set.trim().split_whitespace();
            let quantity = set_parts.next().expect("Cannot read quantity from string").parse::<u32>().expect("Unable to parse quantity");
            let colour = set_parts.next().expect("Cannot read colour from string").trim();

            match colour {
                "red" if quantity > most_reds => most_reds = quantity,
                "green" if quantity > most_greens => most_greens = quantity,
                "blue" if quantity > most_blues => most_blues = quantity,
                "red" => (),
                "green" => (),
                "blue" => (),
                _ => panic!("Unmatched colour '{}'", colour)
            }
        }
    }

    // Calculate the power
    let power = most_reds * most_greens * most_blues;
    println!("Power for game {}: {}", index, power);

    return power;
}
