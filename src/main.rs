extern crate gio;
extern crate glib;
extern crate gtk;
#[macro_use]
extern crate lazy_static;

use std::env;
use std::str::FromStr;
use std::sync::Arc;

use glib::clone;
use gtk::prelude::*;
use gtk::{ResponseType, TextBuffer};

use url::{Position, Url};

mod gui;
use gui::Gui;
mod absolute_url;
use absolute_url::AbsoluteUrl;
mod bookmarks;
mod client;
mod colors;
mod finger;
mod gemini;
mod gopher;
mod history;
use gemini::link::Link as GeminiLink;
use gopher::link::Link as GopherLink;
mod protocols;
use protocols::{Finger, Gemini, Gopher, Protocol, Scheme};
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

    // Bind add_bookmark button
    {
        let button = gui.add_bookmark_button();
        let gui = gui.clone();
        button.connect_clicked(move |_| {
            add_bookmark(&gui);
        });
    }

    // Bind show_bookmarks button
    {
        let button = gui.show_bookmarks_button();
        let gui = gui.clone();
        button.connect_clicked(move |_| {
            show_bookmarks(&gui);
        });
    }

    // Bind URL bar
    {
        let gui_clone = gui.clone();
        let url_bar = gui.url_bar();
        url_bar.connect_activate(move |b| {
            let url = b.get_text().expect("get_text failed").to_string();
            route_url(&gui_clone, url)
        });
    }

    // Create Pango tags
    tags::apply_tags(&content_view.get_buffer().unwrap());

    // Visit URL if one was provided
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let url = String::from(&args[1]);
        route_url(&gui, url)
    }

    gui.start();
    gtk::main();
}

fn route_url(gui: &Arc<Gui>, url: String) {
    if url.starts_with("gemini://") {
        visit_url(&gui, Gemini { source: url })
    } else if url.starts_with("gopher://") {
        visit_url(&gui, Gopher { source: url })
    } else if url.starts_with("finger://") {
        visit_url(&gui, Finger { source: url })
    } else {
        visit_url(
            &gui,
            Gemini {
                source: format!("gemini://{}", url),
            },
        )
    };
}

fn go_back(gui: &Arc<Gui>) {
    let previous = history::get_previous_url();
    if let Some(url) = previous {
        match url.scheme() {
            "finger" => visit_url(
                gui,
                Finger {
                    source: url.to_string(),
                },
            ),
            "gemini" => visit_url(
                gui,
                Gemini {
                    source: url.to_string(),
                },
            ),
            "gopher" => visit_url(
                gui,
                Gopher {
                    source: url.to_string(),
                },
            ),
            _ => (),
        }
    }
}

fn update_url_field(gui: &Arc<Gui>, url: &str) {
    let url_bar = gui.url_bar();
    url_bar.get_buffer().set_text(url);
}

fn add_bookmark(gui: &Arc<Gui>) {
    let url_bar = gui.url_bar();
    let current_url = url_bar.get_text();
    if let Some(url) = current_url {
        bookmarks::add(&url);
        info_dialog(&gui, "Bookmark added.");
    }
}

fn show_bookmarks(gui: &Arc<Gui>) {
    let content_view = gui.content_view();

    let bookmarks_list = format!("# Bookmarks\n\n{}", bookmarks::content());
    let parsed_content = gemini::parser::parse(bookmarks_list);

    clear_buffer(&content_view);
    draw_gemini_content(&gui, parsed_content);

    update_url_field(&gui, "::bookmarks");

    content_view.show_all();
}

fn visit_url<T: AbsoluteUrl + Protocol>(gui: &Arc<Gui>, url: T) {
    if url.get_source_str() == "gemini://::bookmarks" {
        show_bookmarks(&gui);
        return;
    }

    let content_view = gui.content_view();

    match url.get_scheme() {
        Scheme::Gemini => {
            let absolute_url = url.to_absolute_url();

            match absolute_url {
                Ok(absolute_url) => match gemini::client::get_data(url) {
                    Ok((meta, new_content)) => {
                        let meta_str = String::from_utf8_lossy(&meta.unwrap()).to_string();
                        if let Ok(status) = Status::from_str(&meta_str) {
                            match status {
                                Status::Success(meta) => {
                                    if meta.starts_with("text/") {
                                        // display text files.
                                        history::append(absolute_url.as_str());
                                        update_url_field(&gui, absolute_url.as_str());
                                        let content_str =
                                            String::from_utf8_lossy(&new_content).to_string();

                                        let parsed_content = gemini::parser::parse(content_str);
                                        clear_buffer(&content_view);
                                        draw_gemini_content(&gui, parsed_content);

                                        content_view.show_all();
                                    } else {
                                        // download and try to open the rest.
                                        client::download(new_content);
                                    }
                                }
                                Status::Gone(_meta) => {
                                    error_dialog(&gui, "\nSorry page is gone.\n");
                                }
                                Status::RedirectTemporary(new_url)
                                | Status::RedirectPermanent(new_url) => {
                                    visit_url(&gui, Gemini { source: new_url });
                                }
                                Status::TransientCertificateRequired(_meta)
                                | Status::AuthorisedCertificatedRequired(_meta) => {
                                    error_dialog(
                                        &gui,
                                        "\nYou need a valid certificate to access this page.\n",
                                    );
                                }
                                Status::Input(message) => {
                                    input_dialog(&gui, absolute_url, &message);
                                }
                                _ => (),
                            }
                        }
                    }
                    Err(e) => {
                        error_dialog(&gui, &format!("\n{}\n", e));
                    }
                },
                Err(e) => {
                    error_dialog(&gui, &format!("\n{}\n", e));
                }
            }
        }
        Scheme::Gopher => {
            let absolute_url = url.to_absolute_url();
            match absolute_url {
                Ok(abs_url) => match gopher::client::get_data(url) {
                    Ok((_meta, new_content)) => {
                        history::append(abs_url.as_str());
                        update_url_field(&gui, abs_url.as_str());
                        let content_str = String::from_utf8_lossy(&new_content).to_string();

                        let parsed_content = gopher::parser::parse(content_str);
                        clear_buffer(&content_view);
                        draw_gopher_content(&gui, parsed_content);

                        content_view.show_all();
                    }
                    Err(e) => {
                        error_dialog(&gui, &format!("\n{}\n", e));
                    }
                },
                Err(e) => {
                    error_dialog(&gui, &format!("\n{}\n", e));
                }
            }
        }
        Scheme::Finger => {
            let absolute_url = url.to_absolute_url();
            match absolute_url {
                Ok(abs_url) => match finger::client::get_data(url) {
                    Ok((_meta, new_content)) => {
                        history::append(abs_url.as_str());
                        update_url_field(&gui, abs_url.as_str());
                        let content_str = String::from_utf8_lossy(&new_content).to_string();

                        let parsed_content = gopher::parser::parse(content_str);
                        clear_buffer(&content_view);
                        draw_gopher_content(&gui, parsed_content);

                        content_view.show_all();
                    }
                    Err(e) => {
                        error_dialog(&gui, &format!("\n{}\n", e));
                    }
                },
                Err(e) => {
                    error_dialog(&gui, &format!("\n{}\n", e));
                }
            }
        }
    }
}

fn draw_gemini_content(
    gui: &Arc<Gui>,
    content: Vec<Result<gemini::parser::TextElement, gemini::parser::ParseError>>,
) -> TextBuffer {
    let content_view = gui.content_view();
    let buffer = content_view.get_buffer().unwrap();

    for el in content {
        match el {
            Ok(gemini::parser::TextElement::H1(header)) => {
                let mut end_iter = buffer.get_end_iter();
                buffer.insert_markup(
                    &mut end_iter,
                    &format!(
                        "<span foreground=\"#9932CC\" size=\"x-large\">{}</span>\n",
                        header
                    ),
                );
            }
            Ok(gemini::parser::TextElement::H2(header)) => {
                let mut end_iter = buffer.get_end_iter();
                buffer.insert_markup(
                    &mut end_iter,
                    &format!(
                        "<span foreground=\"#FF1493\" size=\"large\">{}</span>\n",
                        header
                    ),
                );
            }
            Ok(gemini::parser::TextElement::H3(header)) => {
                let mut end_iter = buffer.get_end_iter();
                buffer.insert_markup(
                    &mut end_iter,
                    &format!(
                        "<span foreground=\"#87CEFA\" size=\"medium\">{}</span>\n",
                        header
                    ),
                );
            }
            Ok(gemini::parser::TextElement::ListItem(item)) => {
                let mut end_iter = buffer.get_end_iter();
                buffer.insert_markup(
                    &mut end_iter,
                    &format!("<span foreground=\"green\">■ {}</span>\n", item),
                );
            }
            Ok(gemini::parser::TextElement::Text(text)) => {
                let mut end_iter = buffer.get_end_iter();
                buffer.insert_markup(&mut end_iter, &format!("{}\n", text));
            }
            Ok(gemini::parser::TextElement::LinkItem(link_item)) => {
                draw_gemini_link(&gui, link_item);
            }
            Err(_) => println!("Something failed."),
        }
    }
    buffer
}

fn draw_gopher_content(
    gui: &Arc<Gui>,
    content: Vec<Result<gopher::parser::TextElement, gopher::parser::ParseError>>,
) -> TextBuffer {
    let content_view = gui.content_view();
    let buffer = content_view.get_buffer().unwrap();

    for el in content {
        match el {
            Ok(gopher::parser::TextElement::Text(text)) => {
                let mut end_iter = buffer.get_end_iter();
                buffer.insert_markup(&mut end_iter, &format!("{}\n", text));
            }
            Ok(gopher::parser::TextElement::LinkItem(link_item)) => {
                draw_gopher_link(&gui, link_item);
            }
            Ok(gopher::parser::TextElement::ExternalLinkItem(link_item)) => {
                draw_gopher_link(&gui, link_item);
            }
            Ok(gopher::parser::TextElement::Image(_link_item)) => {
                // draw_gopher_link(&gui, link_item);
                ();
            }
            Err(_) => println!("Something failed."),
        }
    }
    buffer
}

fn draw_gemini_link(gui: &Arc<Gui>, link_item: String) {
    match GeminiLink::from_str(&link_item) {
        Ok(GeminiLink::Finger(url, label)) => {
            let button_label = if label.is_empty() {
                url.clone().to_string()
            } else {
                label
            };
            let finger_label = format!("{} [Finger]", button_label);
            insert_gemini_button(&gui, url, finger_label);
        }
        Ok(GeminiLink::Gemini(url, label)) => {
            insert_gemini_button(&gui, url, label);
        }
        Ok(GeminiLink::Gopher(url, label)) => {
            let button_label = if label.is_empty() {
                url.clone().to_string()
            } else {
                label
            };
            let gopher_label = format!("{} [Gopher]", button_label);
            insert_gemini_button(&gui, url, gopher_label);
        }
        Ok(GeminiLink::Http(url, label)) => {
            let button_label = if label.is_empty() {
                url.clone().to_string()
            } else {
                label
            };
            let www_label = format!("{} [WWW]", button_label);

            insert_external_button(&gui, url, &www_label);
        }
        Ok(GeminiLink::Relative(url, label)) => {
            let new_url = Gemini { source: url }.to_absolute_url().unwrap();
            insert_gemini_button(&gui, new_url, label);
        }
        Ok(GeminiLink::Unknown(_, _)) => (),
        Err(_) => (),
    }
}

fn draw_gopher_link(gui: &Arc<Gui>, link_item: String) {
    match GopherLink::from_str(&link_item) {
        Ok(GopherLink::Http(url, label)) => {
            let button_label = if label.is_empty() {
                url.clone().to_string()
            } else {
                label
            };
            let www_label = format!("{} [WWW]", button_label);

            insert_external_button(&gui, url, &www_label);
        }
        Ok(GopherLink::Gopher(url, label)) => {
            let button_label = if label.is_empty() {
                url.clone().to_string()
            } else {
                label
            };
            let gopher_label = format!("{} [Gopher]", button_label);
            insert_gemini_button(&gui, url, gopher_label);
        }
        Ok(GopherLink::Gemini(url, label)) => {
            insert_gemini_button(&gui, url, label);
        }
        Ok(GopherLink::Relative(url, label)) => {
            let new_url = Gopher { source: url }.to_absolute_url().unwrap();
            insert_gemini_button(&gui, new_url, label);
        }
        Ok(GopherLink::Unknown(_, _)) => (),
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
        match url.scheme() {
            "finger" => visit_url(&gui, Finger { source: url.to_string() }),
            "gemini" => visit_url(&gui, Gemini { source: url.to_string() }),
            "gopher" => visit_url(&gui, Gopher { source: url.to_string() }),
            _ => ()
        }
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

fn info_dialog(gui: &Arc<Gui>, message: &str) {
    let dialog = gtk::Dialog::new_with_buttons(
        Some("Info"),
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
        &[
            ("Close", ResponseType::Close),
            ("Send", ResponseType::Accept),
        ],
    );

    let content_area = dialog.get_content_area();
    let entry = gtk::Entry::new();
    content_area.add(&entry);

    dialog.show_all();

    if dialog.run() == gtk::ResponseType::Accept {
        let response = entry.get_text().expect("get_text failed").to_string();
        let cleaned: &str = &url[..Position::AfterPath];
        let full_url = format!("{}?{}", cleaned.to_string(), response);

        visit_url(&gui, Gemini { source: full_url });
    }

    dialog.destroy();
}

fn clear_buffer(view: &gtk::TextView) {
    if let Some(buffer) = view.get_buffer() {
        let (mut start, mut end) = buffer.get_bounds();
        buffer.delete(&mut start, &mut end);
    }
}
