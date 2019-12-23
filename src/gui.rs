use gtk::prelude::*;
use gtk::{ApplicationWindow, Button, Entry, TextView};


pub struct Gui {
    window: ApplicationWindow,
    url_bar: Entry,
    content_view: TextView,
    back_button: Button,
}

impl Gui {
    pub fn new() -> Gui {
        // Initialize the UI from the Glade XML.
        let glade_src = include_str!("castor.glade");
        let builder = gtk::Builder::new_from_string(glade_src);

        // Get handles for the various controls we need to use.
        let window: ApplicationWindow = builder.get_object("window").expect("Couldn't get window");
        let url_bar: Entry = builder.get_object("url_bar").expect("Couldn't get url_bar");
        let content_view: TextView = builder.get_object("content_view").expect("Couldn't get content_view");
        let back_button: Button = builder.get_object("back_button").expect("Couldn't get back_button");

        Gui {
            window,
            url_bar,
            content_view,
            back_button,
        }
    }

    // Set up naming for the window and show it to the user.
    pub fn start(&self) {
        glib::set_application_name("Castor");
        self.window.set_wmclass("Castor", "Castor");
        self.window.connect_delete_event(|_, _| { gtk::main_quit(); Inhibit(false) });
        self.window.show_all();
    }

    // pub fn update_from(&self, state: &State) {
    //     if let Some(ref err) = state.error {
    //         self.error_label.set_text(
    //             &format!("The dice expression entered is not valid:\n{}", err)
    //         );
    //         self.popover.show_all();
    //     } else {
    //         // The popover will hide itself anyway when the user clicks
    //         // outside of it, but we shouldn't leave an error indicator in it.
    //         self.error_label.set_text("");
    //     }

    //     self.result.set_text(&format!("{}", state.value));
    // }

    pub fn url_bar(&self) -> &Entry {
        &self.url_bar
    }

    pub fn content_view(&self) -> &TextView {
        &self.content_view
    }

    pub fn back_button(&self) -> &Button {
        &self.back_button
    }
}
