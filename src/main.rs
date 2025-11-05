mod convert;
mod data;

use slint::VecModel;
use crate::data::{io, Scenario};

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let main_window = MainWindow::new()?;
    let mut scenario = Scenario::default();

    // Menubar callbacks
    main_window.on_open_requested(|| {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Electoral Sim files", &["json"])
            .add_filter("All files", &["*"])
            .set_title("Open File")
            .pick_file()
        {
            /*scenario = */io::open(path);
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

    // TabWidget
    let tabs = std::rc::Rc::new(VecModel::from(vec![
        TabInfo { title: "Home".into(), content: "Welcome to the home tab!".into() },
        TabInfo { title: "About".into(), content: "This app uses Slint!".into() },
    ]));

    main_window.run()
}
