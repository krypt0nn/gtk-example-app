use gtk4::{self as gtk, prelude::*};
use libadwaita as adw;

/// This function loads object from builder or panics if it doesn't exist
pub fn get_object<T: IsA<gtk::glib::Object>>(builder: &gtk::Builder, name: &str) -> T {
    builder.object::<T>(name).unwrap()
}

struct App {
    pub window: adw::ApplicationWindow,
    pub hello_rust: gtk::Button,
    pub greetings_counter: gtk::Label,
    // pub about: gtk::AboutDialog,
    // pub menu: gtk::MenuButton
}

impl App {
    pub fn new(app: &gtk::Application) -> Self {
        // Create builder from UI file
        let builder = gtk::Builder::from_string(include_str!("../ui/.dist/main.ui"));

        // Parse objects from builder
        let result = Self {
            window: get_object(&builder, "window"),
            hello_rust: get_object(&builder, "hello_rust"),
            greetings_counter: get_object(&builder, "greetings_counter"),
            // about: get_object(&builder, "about"),
            // menu: get_object(&builder, "menu"),
        };

        // Bind app to the window
        result.window.set_application(Some(app));

        result
    }
}

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
        let app = App::new(app);

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
