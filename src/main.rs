extern crate gio;
extern crate glib;
extern crate gtk;
#[macro_use]
extern crate lazy_static;

use std::str::FromStr;
use std::sync::Arc;

use glib::clone;
use gtk::prelude::*;
use gtk::{ResponseType, TextBuffer};

use url::{Position, Url};

mod gui;
use gui::Gui;
mod absolute;
mod content;
mod history;
mod link;
use link::Link;
mod parser;
use parser::{ParseError, TextElement, TextElement::*};
mod status;
use status::Status;
mod tags;

fn main() {
    // Start up the GTK3 subsystem.
    gtk::init().expect("Unable to start GTK3. Error");

    // Create the main window.
    let gui = Arc::new(Gui::new());
    let content_view = gui.content_view();

    // Bind back button
    {
        let button = gui.back_button();
        let gui = gui.clone();
        button.connect_clicked(move |_| {
            go_back(&gui);
        });
    }

    // Bind URL bar
    {
        let gui_clone = gui.clone();
        let url_bar = gui.url_bar();
        url_bar.connect_activate(move |b| {
            let url = b.get_text().expect("get_text failed").to_string();
            let full_url = if url.starts_with("gemini://") {
                url
            } else {
                format!("gemini://{}", url)
            };

            visit_url(&gui_clone, full_url);
        });
    }

    // Create Pango tags
    tags::apply_tags(&content_view.get_buffer().unwrap());

    gui.start();
    gtk::main();
}

fn go_back(gui: &Arc<Gui>) {
    let previous = history::get_previous_url();
    if let Some(url) = previous {
        visit_url(gui, url.to_string())
    }
}

fn update_url_field(gui: &Arc<Gui>, url: &str) {
    let url_bar = gui.url_bar();
    url_bar.get_buffer().set_text(url);
}

fn visit_url(gui: &Arc<Gui>, url: String) {
    {
        let content_view = gui.content_view();

        match absolute::make(url.as_str()) {
            Ok(url) => match content::get_data(&url) {
                Ok((meta, new_content)) => {
                    let meta_str = String::from_utf8_lossy(&meta).to_string();
                    if let Ok(status) = Status::from_str(&meta_str) {
                        match status {
                            Status::Success(meta) => {
                                if meta.starts_with("text/") {
                                    // display text files.
                                    history::append(url.as_str());
                                    update_url_field(&gui, url.as_str());
                                    let content_str =
                                        String::from_utf8_lossy(&new_content).to_string();

                                    let parsed_content = parser::parse(content_str);
                                    clear_buffer(&content_view);
                                    draw_content(&gui, parsed_content);

                                    content_view.show_all();
                                } else {
                                    // download and try to open the rest.
                                    content::download(new_content);
                                }
                            }
                            Status::Gone(_meta) => {
                                error_dialog(
                                    &gui,
                                    "\nSorry page is gone.\n",
                                );
                            }
                            Status::RedirectTemporary(new_url)
                            | Status::RedirectPermanent(new_url) => {
                                visit_url(&gui, new_url);
                            }
                            Status::TransientCertificateRequired(_meta)
                            | Status::AuthorisedCertificatedRequired(_meta) => {
                                error_dialog(
                                    &gui,
                                    "\nYou need a valid certificate to access this page.\n",
                                );
                            }
                            Status::Input(message) => {
                                input_dialog(&gui, url, &message);
                            }
                            _ => (),
                        }
                    }
                }
                Err(_) => {
                    error_dialog(&gui, "\nInvalid URL.\n");
                }
            },
            Err(_) => {
                error_dialog(&gui, "\nInvalid URL.\n");
            }
        }
    }
}

fn draw_content(gui: &Arc<Gui>, content: Vec<Result<TextElement, ParseError>>) -> TextBuffer {
    let content_view = gui.content_view();
    let buffer = content_view.get_buffer().unwrap();

    for el in content {
        match el {
            Ok(H1(header)) => {
                let mut end_iter = buffer.get_end_iter();
                buffer.insert_markup(
                    &mut end_iter,
                    &format!(
                        "<span foreground=\"#9932CC\" size=\"x-large\">{}</span>\n",
                        header
                    ),
                );
            }
            Ok(H2(header)) => {
                let mut end_iter = buffer.get_end_iter();
                buffer.insert_markup(
                    &mut end_iter,
                    &format!(
                        "<span foreground=\"#FF1493\" size=\"large\">{}</span>\n",
                        header
                    ),
                );
            }
            Ok(H3(header)) => {
                let mut end_iter = buffer.get_end_iter();
                buffer.insert_markup(
                    &mut end_iter,
                    &format!(
                        "<span foreground=\"#87CEFA\" size=\"medium\">{}</span>\n",
                        header
                    ),
                );
            }
            Ok(ListItem(item)) => {
                let mut end_iter = buffer.get_end_iter();
                buffer.insert_markup(
                    &mut end_iter,
                    &format!("<span foreground=\"green\">■ {}</span>\n", item),
                );
            }
            Ok(Text(text)) => {
                let mut end_iter = buffer.get_end_iter();
                buffer.insert(&mut end_iter, &format!("{}\n", text));
            }
            Ok(LinkItem(link_item)) => {
                draw_link(&gui, link_item);
            }
            Err(_) => println!("Something failed."),
        }
    }
    buffer
}

fn draw_link(gui: &Arc<Gui>, link_item: String) {
    match Link::from_str(&link_item) {
        Ok(Link::Http(url, label)) => {
            let button_label = if label.is_empty() {
                url.clone().to_string()
            } else {
                label
            };
            let www_label = format!("{} [WWW]", button_label);

            insert_external_button(&gui, url, &www_label);
        }
        Ok(Link::Gopher(url, label)) => {
            let button_label = if label.is_empty() {
                url.clone().to_string()
            } else {
                label
            };
            let gopher_label = format!("{} [Gopher]", button_label);
            insert_external_button(&gui, url, &gopher_label);
        }
        Ok(Link::Gemini(url, label)) => {
            insert_gemini_button(&gui, url, label);
        }
        Ok(Link::Relative(url, label)) => {
            let new_url = absolute::make(&url).unwrap();
            insert_gemini_button(&gui, new_url, label);
        }
        Ok(Link::Unknown(_, _)) => (),
        Err(_) => (),
    }
}

fn insert_gemini_button(gui: &Arc<Gui>, url: Url, label: String) {
    let content_view = gui.content_view();
    let buffer = content_view.get_buffer().unwrap();

    let button_label = if label.is_empty() {
        url.clone().to_string()
    } else {
        label
    };

    let button = gtk::Button::new_with_label(&button_label);
    button.set_tooltip_text(Some(&url.to_string()));

    button.connect_clicked(clone!(@weak gui => move |_| {
        visit_url(&gui, url.to_string());
    }));

    let mut start_iter = buffer.get_end_iter();
    let anchor = buffer.create_child_anchor(&mut start_iter).unwrap();
    content_view.add_child_at_anchor(&button, &anchor);
    let mut end_iter = buffer.get_end_iter();
    buffer.insert(&mut end_iter, "\n");
}

fn insert_external_button(gui: &Arc<Gui>, url: Url, label: &str) {
    let content_view = gui.content_view();
    let buffer = content_view.get_buffer().unwrap();

    let button = gtk::Button::new_with_label(&label);
    button.set_tooltip_text(Some(&url.to_string()));

    button.connect_clicked(move |_| {
        open::that(url.to_string()).unwrap();
    });

    let mut start_iter = buffer.get_end_iter();
    let anchor = buffer.create_child_anchor(&mut start_iter).unwrap();
    content_view.add_child_at_anchor(&button, &anchor);
    let mut end_iter = buffer.get_end_iter();
    buffer.insert(&mut end_iter, "\n");
}

fn error_dialog(gui: &Arc<Gui>, message: &str) {
    let dialog = gtk::Dialog::new_with_buttons(
        Some("Error"),
        Some(gui.window()),
        gtk::DialogFlags::MODAL,
        &[("Close", ResponseType::Close)],
    );
    dialog.set_default_response(ResponseType::Close);
    dialog.connect_response(|dialog, _| dialog.destroy());

    let content_area = dialog.get_content_area();
    let message = gtk::Label::new(Some(message));
    content_area.add(&message);

    dialog.show_all();
}

fn input_dialog(gui: &Arc<Gui>, url: Url, message: &str) {
    let dialog = gtk::Dialog::new_with_buttons(
        Some(message),
        Some(gui.window()),
        gtk::DialogFlags::MODAL,
        &[("Close", ResponseType::Close),("Send", ResponseType::Accept)],
    );

    let content_area = dialog.get_content_area();
    let entry = gtk::Entry::new();
    content_area.add(&entry);

    dialog.show_all();

    if dialog.run() == gtk::ResponseType::Accept {
        let response = entry.get_text().expect("get_text failed").to_string();
        let cleaned: &str = &url[..Position::AfterPath];
        let full_url = format!("{}?{}", cleaned.to_string(), response);
        visit_url(&gui, full_url);
    }

    dialog.destroy();
}

fn clear_buffer(view: &gtk::TextView) {
    if let Some(buffer) = view.get_buffer() {
        let (mut start, mut end) = buffer.get_bounds();
        buffer.delete(&mut start, &mut end);
    }
}
