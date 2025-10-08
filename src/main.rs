mod io;
mod data;

use std::collections::HashMap;
use std::io::stdin;

use data::Election;
use io::load_file;

fn main() {
    // Program memory
    let mut election: Election;

    let mut input: String;
    loop {
        // Get user input from the terminal
        input = String::new();
        stdin().read_line(&mut input).expect("Failed to read line");
        input = input.trim().to_string();

        // Parse input commands
        if input == "exit" {
            break;
        } else if input.len() != 0 {
            let input_terms = input.split_whitespace().collect::<Vec<_>>();
            if input_terms[0] == "load" {
                println!("Loading {}...", input_terms[1]);
                election = load_file(&input_terms[1]);
            }
        }
    }
}
