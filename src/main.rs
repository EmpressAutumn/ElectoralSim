mod data;

use std::sync::{LazyLock, Mutex};
use slint::{Model, ToSharedString, VecModel};
use crate::data::{io, Scenario};

slint::include_modules!();

// Potentially unsafe, definitely suboptimal, change this later
static SCENARIO: LazyLock<Mutex<Scenario>> = LazyLock::new(|| Mutex::new(Scenario::default()));

fn main() -> Result<(), slint::PlatformError> {
    let window = MainWindow::new()?;
    let window_weak = window.as_weak();

    // Menubar callbacks
    window.on_new_requested(|| {
        *SCENARIO.lock().unwrap() = Scenario::default();
    });
    window.on_open_requested({
        let window_weak = window_weak.clone();
        move || {
            if let Some(path) = rfd::FileDialog::new()
                .add_filter("Electoral Sim files", &["json"])
                .add_filter("All files", &["*"])
                .set_title("Open File")
                .pick_file()
            {
                *SCENARIO.lock().unwrap() = io::open(path);

                if let Some(window) = window_weak.upgrade() {
                    let mut elections: Vec<SlintElection> = Vec::new();
                    for election in SCENARIO.lock().unwrap().elections() {
                        elections.push(SlintElection { date: election.date().to_shared_string() });
                    }
                    window.set_elections(std::rc::Rc::new(slint::VecModel::from(elections)).clone().into());
                }
            }
        }
    });
    window.on_import_requested(|| {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Comma separated value files", &["csv"])
            .add_filter("All files", &["*"])
            .set_title("Import File")
            .pick_file()
        {
            *SCENARIO.lock().unwrap() = io::import(path);
        }
    });
    window.on_quit_requested(|| {
        std::process::exit(0);
    });

    window.run()
}
