use glib::clone;
use gtk::prelude::*;
use gtk::TextBuffer;
use std::str::FromStr;
use std::sync::Arc;
use url::Url;

use crate::absolute_url::AbsoluteUrl;
use crate::gemini::link::Link as GeminiLink;
use crate::gopher::link::Link as GopherLink;
use crate::gui::Gui;
use crate::protocols::{Finger, Gemini, Gopher};

pub fn gemini_content(
    gui: &Arc<Gui>,
    content: Vec<Result<crate::gemini::parser::TextElement, crate::gemini::parser::ParseError>>,
) -> TextBuffer {
    let content_view = gui.content_view();
    let buffer = content_view.get_buffer().unwrap();

    let mut mono_toggle = false;
    let font_family = if crate::settings::gemini_monospace() {
        "monospace"
    } else {
        "sans"
    };

    for el in content {
        match el {
            Ok(crate::gemini::parser::TextElement::H1(header)) => {
                let mut end_iter = buffer.get_end_iter();
                buffer.insert_markup(
                    &mut end_iter,
                    &format!(
                        "<span foreground=\"{}\" size=\"x-large\" font_family=\"{}\">{}{}</span>\n",
                        crate::settings::h1_color(),
                        font_family,
                        crate::settings::h1_character(),
                        escape_text(&header)
                    ),
                );
            }
            Ok(crate::gemini::parser::TextElement::H2(header)) => {
                let mut end_iter = buffer.get_end_iter();
                buffer.insert_markup(
                    &mut end_iter,
                    &format!(
                        "<span foreground=\"{}\" size=\"large\" font_family=\"{}\">{}{}</span>\n",
                        crate::settings::h2_color(),
                        font_family,
                        crate::settings::h2_character(),
                        escape_text(&header)
                    ),
                );
            }
            Ok(crate::gemini::parser::TextElement::H3(header)) => {
                let mut end_iter = buffer.get_end_iter();
                buffer.insert_markup(
                    &mut end_iter,
                    &format!(
                        "<span foreground=\"{}\" size=\"medium\" font_family=\"{}\">{}{}</span>\n",
                        crate::settings::h3_color(),
                        font_family,
                        crate::settings::h3_character(),
                        escape_text(&header)
                    ),
                );
            }
            Ok(crate::gemini::parser::TextElement::ListItem(item)) => {
                let mut end_iter = buffer.get_end_iter();
                buffer.insert_markup(
                    &mut end_iter,
                    &format!(
                        "<span foreground=\"{}\" font_family=\"{}\">{} {}</span>\n",
                        crate::settings::list_color(),
                        font_family,
                        crate::settings::list_character(),
                        escape_text(&item)
                    ),
                );
            }
            Ok(crate::gemini::parser::TextElement::MonoText(_text)) => {
                mono_toggle = !mono_toggle;
            }
            Ok(crate::gemini::parser::TextElement::Text(text)) => {
                let mut end_iter = buffer.get_end_iter();
                let text = if text.contains("<span") {
                    text
                } else {
                    escape_text(&text)
                };

                if mono_toggle {
                    buffer.insert_markup(
                        &mut end_iter,
                        &format!(
                            "<span foreground=\"{}\" font_family=\"monospace\">{}</span>\n",
                            crate::settings::text_color(),
                            text
                        ),
                    );
                } else {
                    buffer.insert_markup(
                        &mut end_iter,
                        &format!(
                            "<span foreground=\"{}\" font_family=\"{}\">{}</span>\n",
                            crate::settings::text_color(),
                            font_family,
                            text
                        ),
                    );
                }
            }
            Ok(crate::gemini::parser::TextElement::LinkItem(link_item)) => {
                gemini_link(&gui, link_item);
            }
            Err(_) => println!("Something failed."),
        }
    }
    buffer
}

pub fn gemini_text_content(
    gui: &Arc<Gui>,
    content: Vec<Result<crate::gemini::parser::TextElement, crate::gemini::parser::ParseError>>,
) -> TextBuffer {
    let content_view = gui.content_view();
    let buffer = content_view.get_buffer().unwrap();

    for el in content {
        match el {
            Ok(crate::gemini::parser::TextElement::Text(text)) => {
                let mut end_iter = buffer.get_end_iter();
                buffer.insert_markup(
                    &mut end_iter,
                    &format!(
                        "<span foreground=\"{}\" font_family=\"monospace\">{}</span>\n",
                        crate::settings::text_color(),
                        escape_text(&text)
                    ),
                );
            }
            Ok(_) => (),
            Err(_) => println!("Something failed."),
        }
    }
    buffer
}

pub fn gopher_content(
    gui: &Arc<Gui>,
    content: Vec<Result<crate::gopher::parser::TextElement, crate::gopher::parser::ParseError>>,
) -> TextBuffer {
    let content_view = gui.content_view();
    let buffer = content_view.get_buffer().unwrap();

    for el in content {
        match el {
            Ok(crate::gopher::parser::TextElement::Text(text)) => {
                let mut end_iter = buffer.get_end_iter();
                let font_family = if crate::settings::gopher_monospace() {
                    "font_family=\"monospace\""
                } else {
                    "font_family=\"serif\""
                };

                buffer.insert_markup(
                    &mut end_iter,
                    &format!(
                        "<span foreground=\"{}\" {}>{}</span>\n",
                        crate::settings::text_color(),
                        font_family,
                        escape_text(&text)
                    ),
                );
            }
            Ok(crate::gopher::parser::TextElement::LinkItem(link_item)) => {
                gopher_link(&gui, link_item);
            }
            Ok(crate::gopher::parser::TextElement::ExternalLinkItem(link_item)) => {
                gopher_link(&gui, link_item);
            }
            Ok(crate::gopher::parser::TextElement::Image(link_item)) => {
                gopher_link(&gui, link_item);
            }
            Err(_) => println!("Something failed."),
        }
    }
    buffer
}

pub fn finger_content(
    gui: &Arc<Gui>,
    content: Vec<Result<crate::finger::parser::TextElement, crate::finger::parser::ParseError>>,
) -> TextBuffer {
    let content_view = gui.content_view();
    let buffer = content_view.get_buffer().unwrap();

    for el in content {
        match el {
            Ok(crate::finger::parser::TextElement::Text(text)) => {
                let mut end_iter = buffer.get_end_iter();
                let font_family = if crate::settings::finger_monospace() {
                    "font_family=\"monospace\""
                } else {
                    "font_family=\"serif\""
                };

                buffer.insert_markup(
                    &mut end_iter,
                    &format!(
                        "<span foreground=\"{}\" {}>{}</span>\n",
                        crate::settings::text_color(),
                        font_family,
                        escape_text(&text)
                    ),
                );
            }
            Err(_) => println!("Something failed."),
        }
    }
    buffer
}

pub fn gemini_link(gui: &Arc<Gui>, link_item: String) {
    match GeminiLink::from_str(&link_item) {
        Ok(GeminiLink::Finger(url, label)) => {
            let button_label = if label.is_empty() {
                url.clone().to_string()
            } else {
                label
            };
            let finger_label = format!("{} [Finger]", button_label);
            insert_button(&gui, url, finger_label);
        }
        Ok(GeminiLink::Gemini(url, label)) => {
            insert_button(&gui, url, label);
        }
        Ok(GeminiLink::Gopher(url, label)) => {
            let button_label = if label.is_empty() {
                url.clone().to_string()
            } else {
                label
            };
            let gopher_label = format!("{} [Gopher]", button_label);
            insert_button(&gui, url, gopher_label);
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
            insert_button(&gui, new_url, label);
        }
        Ok(GeminiLink::Unknown(_, _)) => (),
        Err(_) => (),
    }
}

pub fn gopher_link(gui: &Arc<Gui>, link_item: String) {
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
            insert_button(&gui, url, gopher_label);
        }
        Ok(GopherLink::Image(url, label)) => {
            let button_label = if label.is_empty() {
                url.clone().to_string()
            } else {
                label
            };
            let image_label = format!("{} [Image]", button_label);
            insert_gopher_file_button(&gui, url, image_label);
        }
        Ok(GopherLink::Gemini(url, label)) => {
            insert_button(&gui, url, label);
        }
        Ok(GopherLink::Relative(url, label)) => {
            let new_url = Gopher { source: url }.to_absolute_url().unwrap();
            insert_button(&gui, new_url, label);
        }
        Ok(GopherLink::Unknown(_, _)) => (),
        Err(_) => (),
    }
}

pub fn insert_button(gui: &Arc<Gui>, url: Url, label: String) {
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
            "finger" => crate::visit_url(&gui, Finger { source: url.to_string() }),
            "gemini" => crate::visit_url(&gui, Gemini { source: url.to_string() }),
            "gopher" => crate::visit_url(&gui, Gopher { source: url.to_string() }),
            _ => ()
        }
    }));

    let mut start_iter = buffer.get_end_iter();
    let anchor = buffer.create_child_anchor(&mut start_iter).unwrap();
    content_view.add_child_at_anchor(&button, &anchor);
    let mut end_iter = buffer.get_end_iter();
    buffer.insert(&mut end_iter, "\n");
}

pub fn insert_gopher_file_button(gui: &Arc<Gui>, url: Url, label: String) {
    let content_view = gui.content_view();
    let buffer = content_view.get_buffer().unwrap();

    let button_label = if label.is_empty() {
        url.clone().to_string()
    } else {
        label
    };

    let button = gtk::Button::new_with_label(&button_label);
    button.set_tooltip_text(Some(&url.to_string()));

    button.connect_clicked(move |_| {
        let (_meta, content) = crate::gopher::client::get_data(Gopher {
            source: url.to_string(),
        })
        .unwrap();
        crate::client::download(content);
    });

    let mut start_iter = buffer.get_end_iter();
    let anchor = buffer.create_child_anchor(&mut start_iter).unwrap();
    content_view.add_child_at_anchor(&button, &anchor);
    let mut end_iter = buffer.get_end_iter();
    buffer.insert(&mut end_iter, "\n");
}

pub fn insert_external_button(gui: &Arc<Gui>, url: Url, label: &str) {
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

fn escape_text(str: &str) -> String {
    String::from(glib::markup_escape_text(&str).as_str())
}
