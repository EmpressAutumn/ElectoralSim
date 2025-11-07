mod convert;
mod data;

use slint::{Model, VecModel};
use crate::data::{io, Election, Scenario};

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let window = MainWindow::new()?;
    let mut scenario = Scenario::default();

    // Menubar callbacks
    window.on_open_requested(|| {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Electoral Sim files", &["json"])
            .add_filter("All files", &["*"])
            .set_title("Open File")
            .pick_file()
        {
            /*scenario = */io::open(path);
        }
    });
    window.on_import_requested(|| {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Comma separated value files", &["csv"])
            .add_filter("All files", &["*"])
            .set_title("Import File")
            .pick_file()
        {
            println!("User imported file: {}", path.display());
        }
    });
    window.on_quit_requested(|| {
        std::process::exit(0);
    });

    window.run()
}
