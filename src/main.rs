mod convert;
mod data;

use std::collections::HashMap;
use std::io::stdin;

use data::Election;
use crate::data::io;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let main_window = MainWindow::new()?;

    // Menubar callbacks
    main_window.on_open_requested(|| {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Electoral Sim files", &["json"])
            .add_filter("All files", &["*"])
            .set_title("Open File")
            .pick_file()
        {
            io::open(path);
        }
    });
    main_window.on_import_requested(|| {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Comma separated value files", &["csv"])
            .add_filter("All files", &["*"])
            .set_title("Import File")
            .pick_file()
        {
            println!("User imported file: {}", path.display());
        }
    });
    main_window.on_quit_requested(|| {
        std::process::exit(0);
    });

    main_window.run()
}

/*fn main() {
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
            if input_terms[0] == "convert" {
                if input_terms[1] == "csv" {
                    
                }
            }
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
            } else if input_terms[0] == "display" {
                //iced::run("Election Simulator", ElectoralSim::update, ElectoralSim::view).expect("TODO: panic message");
            }
        }
    }
}*/
