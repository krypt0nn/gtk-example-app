use gtk4::{self as gtk, prelude::*};
use libadwaita::{self as adw, prelude::*};

use super::get_object;

pub struct App {
    pub window: adw::ApplicationWindow,
    pub hello_rust: gtk::Button,
    pub greetings_counter: gtk::Label,
    // pub about: gtk::AboutDialog,
    // pub menu: gtk::MenuButton
}

impl App {
    pub fn new(app: &gtk::Application) -> Self {
        // Create builder from UI file
        let builder = gtk::Builder::from_string(include_str!("../../assets/ui/.dist/main.ui"));

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
