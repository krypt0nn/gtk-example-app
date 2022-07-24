use gtk4::{self as gtk, prelude::*};
use libadwaita::{self as adw, prelude::*};

use gtk::glib;
use gtk::glib::clone;

pub mod ui;

fn main() {
    gtk::init().expect("GTK initialization failed");
    adw::init();

    // Register and include resources
    gtk::gio::resources_register_include!(".assets.gresource")
        .expect("Failed to register resources");

    // Create app
    let application = gtk::Application::new(
        Some("com.github.krypt0nn.gtk-example-app"),
        Default::default()
    );

    // Init app window and show it
    application.connect_activate(|app| {
        let app = ui::main::App::new(app);

        // Increment counter every second
        std::thread::spawn(clone!(@strong app => move || {
            loop {
                app.update(ui::main::Actions::Increment);

                std::thread::sleep(std::time::Duration::from_secs(1));
            }
        }));

        app.show();
    });

    // Run app
    application.run();
}
