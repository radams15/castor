#![allow(unused_variables, unused_mut)]

extern crate gio;
extern crate glib;
extern crate gtk;
#[macro_use]
extern crate lazy_static;

use gio::prelude::*;
use glib::clone;
use gtk::prelude::*;
use gtk::{ApplicationWindow, Builder, Button, Entry, TextBuffer, TextBufferExt, TextView};

use std::env::args;

extern crate regex;
use regex::Regex;

mod content;
mod absolute;
mod history;
mod tags;
mod link;
use link::Link;

const LINK_REGEX: &str = r"^=>\s*(\S*)\s*(.*)?$";
const H1_REGEX: &str = r"^#\s+(.*)$";
const H2_REGEX: &str = r"^##\s+(.*)$";
const H3_REGEX: &str = r"^###\s+(.*)$";
const UL_REGEX: &str = r"^\s*\*\s+(.*)$";

fn build_ui(application: &gtk::Application) {
    let glade_src = include_str!("castor.glade");
    let builder = Builder::new_from_string(glade_src);

    let window: ApplicationWindow = builder.get_object("window").expect("Couldn't get window");
    window.set_application(Some(application));
    let url_bar: Entry = builder.get_object("url_bar").expect("Couldn't get url_bar");
    let content_view: TextView = builder.get_object("content_view").expect("Couldn't get content_view");
    let back_button: Button = builder.get_object("back_button").expect("Couldn't get back_button");

    tags::apply_tags(&content_view.get_buffer().unwrap());

    url_bar.connect_activate(clone!(@weak content_view => move |bar| {
        let url = bar.get_text().expect("get_text failed").to_string();
        let full_url = if url.starts_with("gemini://") {
            url
        } else {
            format!("gemini://{}", url)
        };

        let new_content = visit_url(full_url, &content_view);
    }));

    back_button.connect_clicked(clone!(@weak content_view => move |_| {
        go_back(&content_view);
    }));

    window.show_all();
}

fn main() {
    let application =
        gtk::Application::new(Some("org.typed-hole.castor"), Default::default())
            .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}

fn go_back(view: &TextView) {
    let previous = history::get_previous_url();
    if let Some(url) = previous {
        visit_url(url.to_string(), view)
    }
}

fn visit_url(url: String, view: &TextView) {
    {
        println!("{:?}", url);
        match absolute::make(url.as_str()) {
            Ok(url) => match content::get_data(&url) {
                Ok((_meta, new_content)) => {
                    history::append(url.as_str());
                    let content_str = String::from_utf8_lossy(&new_content).to_string();
                    clear_buffer(&view);
                    parse_gemini(content_str, &view);
                    view.show_all();
                }
                Err(_) => {
                    let buffer = view.get_buffer().unwrap();
                    let mut end_iter = buffer.get_end_iter();

                    clear_buffer(&view);

                    buffer.insert_markup(
                        &mut end_iter,
                        "<span foreground=\"red\" size=\"x-large\">ERROR</span>\n",
                    );
                }
            }
            Err(_) => {
                println!("Could not parse {}", url.as_str());
            }
        }
    }
}

fn parse_gemini(content: String, view: &TextView) -> TextBuffer {
    let link_regexp = Regex::new(LINK_REGEX).unwrap();
    let h1_regexp = Regex::new(H1_REGEX).unwrap();
    let h2_regexp = Regex::new(H2_REGEX).unwrap();
    let h3_regexp = Regex::new(H3_REGEX).unwrap();
    let ul_regexp = Regex::new(UL_REGEX).unwrap();
    let buffer = view.get_buffer().unwrap();
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

            button.connect_clicked(clone!(@weak view => move |button| {
                let new_url = absolute::make(&dest.clone()).unwrap().to_string();
                visit_url(new_url, &view);
            }));

            let mut start_iter = buffer.get_iter_at_line(i);
            let anchor = buffer.create_child_anchor(&mut start_iter).unwrap();
            view.add_child_at_anchor(&button, &anchor);
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
            buffer.insert(&mut end_iter, &format!("Header 3: {}\n", header));
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
