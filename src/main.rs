use gtk4::{self as gtk, prelude::*};
use libadwaita::{self as adw, prelude::*};

pub mod ui;

use ui::MainApp;

fn main() {
    gtk::init().expect("GTK initialization failed");
    adw::init();

    // Create app
    let application = gtk::Application::new(
        Some("com.github.krypt0nn.gtk-example-app"),
        Default::default()
    );

    // Init app window and show it
    application.connect_activate(|app| {
        let app = MainApp::new(app);

        app.show();
    });

    // Run app
    application.run();
}
