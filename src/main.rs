mod data;

use std::collections::HashMap;
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
                //println!("Loading {}...", input_terms[1]);
                election = Election::load_file(&input_terms[1]);
            } else if input_terms[0] == "save" {
                election.save_file(&input_terms[1]);
            } else if input_terms[0] == "shift" && input_terms.len() > 1{
                let mut voteshare: HashMap<String, f32> = HashMap::new();
                for i in 0..((input_terms.len() - 1)/2) {
                    voteshare.insert(input_terms[2*i + 1].to_string(), input_terms[2 * i + 2].to_string().parse::<f32>().unwrap());
                }
                election.op_vote_split_full(voteshare);
            }
        }
    }
}
