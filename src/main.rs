#![allow(unused_variables, unused_mut)]

extern crate gio;
extern crate glib;
extern crate gtk;
#[macro_use]
extern crate lazy_static;

use std::sync::Arc;

use glib::clone;
use gtk::prelude::*;
use gtk::{TextBuffer, TextBufferExt};

extern crate regex;
use regex::Regex;

mod gui;
use gui::Gui;
mod absolute;
mod content;
mod history;
mod link;
mod tags;

const LINK_REGEX: &str = r"^=>\s*(\S*)\s*(.*)?$";
const H1_REGEX: &str = r"^#\s+(.*)$";
const H2_REGEX: &str = r"^##\s+(.*)$";
const H3_REGEX: &str = r"^###\s+(.*)$";
const UL_REGEX: &str = r"^\s*\*\s+(.*)$";


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
        button.connect_clicked(clone!(@weak content_view => move |_| {
            go_back(&gui);
        }));
    }

    // Bind URL bar
    {
        let gui2 = gui.clone();
        let url_bar = gui.url_bar();
        url_bar.connect_activate(clone!(@weak content_view => move |bar| {
            let url = bar.get_text().expect("get_text failed").to_string();
            let full_url = if url.starts_with("gemini://") {
                url
            } else {
                format!("gemini://{}", url)
            };

            let new_content = visit_url(&gui2, full_url);
        }));
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

fn update_url_field(gui: &Arc<Gui>, url: &str) -> () {
    let url_bar = gui.url_bar();
    url_bar.get_buffer().set_text(url);
}

fn visit_url(gui: &Arc<Gui>, url: String) {
    {
        let content_view = gui.content_view();

        match absolute::make(url.as_str()) {
            Ok(url) => match content::get_data(&url) {
                Ok((_meta, new_content)) => {
                    history::append(url.as_str());
                    update_url_field(&gui, url.as_str());
                    let content_str = String::from_utf8_lossy(&new_content).to_string();

                    clear_buffer(&content_view);

                    parse_gemini(&gui, content_str);
                    content_view.show_all();
                }
                Err(_) => {
                    let buffer = content_view.get_buffer().unwrap();
                    let mut end_iter = buffer.get_end_iter();

                    clear_buffer(&content_view);

                    buffer.insert_markup(
                        &mut end_iter,
                        "<span foreground=\"red\" size=\"x-large\">ERROR</span>\n",
                    );
                }
            },
            Err(_) => {
                println!("Could not parse {}", url.as_str());
            }
        }
    }
}

fn parse_gemini(gui: &Arc<Gui>, content: String) -> TextBuffer {
    let link_regexp = Regex::new(LINK_REGEX).unwrap();
    let h1_regexp = Regex::new(H1_REGEX).unwrap();
    let h2_regexp = Regex::new(H2_REGEX).unwrap();
    let h3_regexp = Regex::new(H3_REGEX).unwrap();
    let ul_regexp = Regex::new(UL_REGEX).unwrap();
    let content_view = gui.content_view();
    let buffer = content_view.get_buffer().unwrap();
    let mut i = 0;

    for line in content.lines() {
        if link_regexp.is_match(line) {
            let caps = link_regexp.captures(&line).unwrap();
            let dest = String::from(caps.get(1).map_or("", |m| m.as_str()));
            let label = String::from(caps.get(2).map_or("", |m| m.as_str()));

            let button_label = if label.is_empty() {
                dest.clone()
            } else {
                label
            };

            let button = gtk::Button::new_with_label(&button_label);
            button.set_tooltip_text(Some(&dest));

            button.connect_clicked(clone!(@weak gui => move |button| {
                let new_url = absolute::make(&dest.clone()).unwrap().to_string();
                visit_url(&gui, new_url);
            }));

            let mut start_iter = buffer.get_iter_at_line(i);
            let anchor = buffer.create_child_anchor(&mut start_iter).unwrap();
            content_view.add_child_at_anchor(&button, &anchor);
            let mut end_iter = buffer.get_end_iter();
            buffer.insert(&mut end_iter, "\n");
        } else if h1_regexp.is_match(line) {
            let caps = h1_regexp.captures(&line).unwrap();
            let header = caps.get(1).map_or("", |m| m.as_str());
            let mut end_iter = buffer.get_end_iter();
            buffer.insert_markup(
                &mut end_iter,
                &format!(
                    "<span foreground=\"#9932CC\" size=\"x-large\">{}</span>\n",
                    header
                ),
            );
        } else if h2_regexp.is_match(line) {
            let caps = h2_regexp.captures(&line).unwrap();
            let header = caps.get(1).map_or("", |m| m.as_str());
            let mut end_iter = buffer.get_end_iter();
            buffer.insert_markup(
                &mut end_iter,
                &format!(
                    "<span foreground=\"#FF1493\" size=\"large\">{}</span>\n",
                    header
                ),
            );
        } else if h3_regexp.is_match(line) {
            let caps = h3_regexp.captures(&line).unwrap();
            let header = caps.get(1).map_or("", |m| m.as_str());
            let mut end_iter = buffer.get_end_iter();
            buffer.insert_markup(
                &mut end_iter,
                &format!(
                    "<span foreground=\"#87CEFA\" size=\"medium\">{}</span>\n",
                    header
                ),
            );
        } else if ul_regexp.is_match(line) {
            let caps = ul_regexp.captures(&line).unwrap();
            let header = caps.get(1).map_or("", |m| m.as_str());
            let mut end_iter = buffer.get_end_iter();
            buffer.insert_markup(
                &mut end_iter,
                &format!("<span foreground=\"green\">■ {}</span>\n", header),
            );
        } else if line.is_empty() {
            let mut end_iter = buffer.get_end_iter();
            buffer.insert(&mut end_iter, "\n");
        } else {
            let mut end_iter = buffer.get_end_iter();
            buffer.insert(&mut end_iter, &format!("{}\n", line));
        }
        i += 1;
    }
    buffer
}

fn clear_buffer(view: &gtk::TextView) {
    match view.get_buffer() {
        Some(buffer) => {
            let (mut start, mut end) = buffer.get_bounds();
            buffer.delete(&mut start, &mut end);
        }
        None => (),
    }
}
