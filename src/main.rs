use gtk::Orientation::{Horizontal, Vertical};
use gtk::{ButtonExt, EntryExt, Inhibit, OrientableExt, WidgetExt};
use relm::Widget;
use relm_derive::{widget, Msg};
use webkit2gtk::WebViewExt;

use self::Msg::*;

pub struct Model {
    counter: i32,
}

#[derive(Msg)]
pub enum Msg {
    Back,
    Go(String),
    Quit,
    Next,
    Search,
}

#[widget]
impl Widget for Win {
    fn init_view(&mut self) {
        // self.webview.load_uri("https://crates.io/");
    }

    fn model() -> Model {
        Model { counter: 0 }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Back => (),
            Go(url) => {
                println!("{:?}", url);
                self.webview.load_uri(&url);
            }
            Quit => gtk::main_quit(),
            Next => (),
            Search => (),
        }
    }

    view! {
        gtk::Window {
            gtk::Box {
                orientation: Vertical,
                gtk::ButtonBox {
                    orientation: Horizontal,
                    #[name="search_box"]

                    #[name="back_button"]
                    gtk::Button {
                        label: "<",
                        clicked => Back,
                    },

                    #[name="next_button"]
                    gtk::Button {
                        label: ">",
                        clicked => Next,
                    },

                    #[name="url_bar"]
                    gtk::Entry {
                        activate(url_bar) => {
                            let url = url_bar.get_text().expect("get_text failed").to_string();
                            Go(url)
                        },
                        placeholder_text: Some("Enter a URL"),
                        width_chars: 40,
                    },
                },
                #[name="webview"]
                webkit2gtk::WebView {
                    vexpand: true,
                },
            },
            delete_event(_, _) => (Quit, Inhibit(false)),
        }
    }
}

fn main() {
    Win::run(()).expect("Win::run failed");
}
