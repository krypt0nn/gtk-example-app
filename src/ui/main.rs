use gtk4::{self as gtk, prelude::*};
use libadwaita::{self as adw, prelude::*};

use std::rc::Rc;
use std::cell::Cell;

use crate::ui::*;

/// This structure is used to describe widgets used in application
/// 
/// `AppWidgets::default` function loads UI file from `.assets/ui/.dist` folder and returns structure with references to its widgets
/// 
/// This function does not implement events
#[derive(Clone)]
pub struct AppWidgets {
    pub window: adw::ApplicationWindow,
    pub increment: gtk::Button,
    pub decrement: gtk::Button,
    pub counter: gtk::Label
}

impl Default for AppWidgets {
    fn default() -> Self {
        let builder = gtk::Builder::from_string(include_str!("../../assets/ui/.dist/main.ui"));

        Self {
            window: get_object(&builder, "window"),
            increment: get_object(&builder, "increment"),
            decrement: get_object(&builder, "decrement"),
            counter: get_object(&builder, "counter")
        }
    }
}

/// This enum is used to describe an action inside of this application
/// 
/// It may be helpful if you want to add the same event for several widgets, or call an action inside of another action
#[derive(Debug)]
pub enum Actions {
    Increment,
    Decrement
}

/// This enum is used to store some of this application data
/// 
/// In this example we store a counter here to know what should we increment or decrement
/// 
/// This must implement `Default` trait
#[derive(Debug, Default)]
pub struct Values {
    pub counter: u8
}

/// The main application structure
/// 
/// `Default` macro automatically calls `AppWidgets::default`, i.e. loads UI file and reference its widgets
/// 
/// `Rc<Cell<Values>>` means this:
/// - `Rc` addeds ability to reference the same value from various clones of the structure.
///   This will guarantee us that inner `Cell<Values>` is the same for all the `App::clone()` values
/// - `Cell` addeds inner mutability to its value, so we can mutate it even without mutable reference.
/// 
/// So we have a shared reference to some value that can be changed without mutable reference.
/// That's what we need and what we use in `App::update` method
#[derive(Default, Clone)]
pub struct App {
    widgets: AppWidgets,
    values: Rc<Cell<Values>>
}

impl App {
    /// Create new application
    pub fn new(app: &gtk::Application) -> Self {
        // 
        let result = Self::default().init_events();

        // Bind app to the window
        result.widgets.window.set_application(Some(app));

        result
    }

    /// Add default events and values to the widgets
    fn init_events(self) -> Self {
        // Add increment action
        let result_copy = self.clone();

        self.widgets.increment.connect_clicked(move |_| {
            result_copy.update(Actions::Increment);
        });

        // Add decrement action
        let result_copy = self.clone();

        self.widgets.decrement.connect_clicked(move |_| {
            result_copy.update(Actions::Decrement);
        });

        self
    }

    /// Update widgets state by calling some action
    pub fn update(&self, action: Actions) {
        let mut values = self.values.take();

        println!("[update] action: {:?}, counter: {:?}", &action, &values);

        match action {
            Actions::Increment => {
                if values.counter < 255 {
                    values.counter += 1;

                    self.widgets.counter.set_label(&format!("Counter: {}", values.counter));
                }
            }

            Actions::Decrement => {
                if values.counter > 0 {
                    values.counter -= 1;

                    self.widgets.counter.set_label(&format!("Counter: {}", values.counter));
                }
            }
        }

        self.values.set(values);
    }

    /// Show application window
    pub fn show(&self) {
        self.widgets.window.show();
    }
}
