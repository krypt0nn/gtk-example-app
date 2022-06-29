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

        // Set button event
        app.hello_rust.connect_clicked(move |_| {
            // Parse counter
            let counter = app.greetings_counter.label()[11..].parse::<u32>().unwrap();

            // And update label text
            app.greetings_counter.set_label(format!("Greetings: {}", counter + 1).as_str());

            // app.about.show();
        });

        app.window.show();
    });

    // Run app
    application.run();
}
