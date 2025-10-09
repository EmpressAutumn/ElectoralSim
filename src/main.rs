mod data;

use std::io::stdin;

use data::Election;

fn main() {
    // Program memory
    let mut election: Election = Election::new("empty".to_string());

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
                election = Election::load_file(&input_terms[1]);
            } else if input_terms[0] == "save" {
                election.save_file(&input_terms[1]);
            }
        }
    }
}
