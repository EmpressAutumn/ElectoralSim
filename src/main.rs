mod data;

use crate::data::{io, Scenario};
use slint::ToSharedString;
use std::sync::{LazyLock, Mutex};

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

    // Tabs callbacks
    window.on_select_election({
        let window_weak = window_weak.clone();
        move |e_num: i32| {
            if let Some(window) = window_weak.upgrade() {
                let mut races: Vec<SlintRace> = Vec::new();
                for race in SCENARIO.lock().unwrap().elections().to_vec()[e_num as usize].races() {
                    races.push(SlintRace { id: race.id().to_shared_string() });
                }
                window.set_races(std::rc::Rc::new(slint::VecModel::from(races)).clone().into());
            }
        }
    });

    window.run()
}
