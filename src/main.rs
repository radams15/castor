use std::fs::File;
use std::io::prelude::*;
use std::io::{BufRead, BufReader};

use gtk::Orientation::{Horizontal, Vertical};
use gtk::{ButtonExt, EntryExt, Inhibit, OrientableExt, TextBufferExt, TextViewExt, WidgetExt};
use relm::Widget;
use relm_derive::{widget, Msg};
use webkit2gtk::{WebContext, WebContextExt, WebViewExt};

extern crate regex;
use regex::Regex;

use url::Url;
mod content;

const LINK_REGEX: &str = r"^=>\s*(\S*)\s*(.*)?$";
const H1_REGEX: &str = r"^#\s+(.*)$";
const H2_REGEX: &str = r"^##\s+(.*)$";
const H3_REGEX: &str = r"^###\s+(.*)$";
const UL_REGEX: &str = r"^\s*\*\s+(.*)$";

use self::Msg::*;

pub struct Model {
    current_url: String,
    current_host: String,
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
        // self.webview.load_uri("gemini://gemini.circumlunar.space");
    }

    fn model() -> Model {
        Model {
            current_url: String::from(""),
            current_host: String::from(""),
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Back => (),
            Go(url) => {
                let new_url = Url::parse(&url).unwrap();
                let data = content::get_data(&new_url);
                self.model.current_url = new_url.to_string();
                self.model.current_host = new_url.host().unwrap().to_string();
                match data {
                    Ok((meta, new_content)) => {
                        clear_buffer(&self.webview);
                        let content_str = String::from_utf8_lossy(&new_content).to_string();
                        // let content = gemini2html(content_str, self.model.current_url.clone());
                        // self.webview.load_html(&content, None);
                        let content = parse_gemini(
                            &content_str,
                            self.model.current_url.clone(),
                            self.model.current_host.clone(),
                            self.webview.get_buffer().unwrap(),
                        );

                        let (start, end) = content.get_bounds();
                        // content.delete(&mut start, &mut end);
                        // let (start, end) = content.get_bounds();
                        content.set_text(&content.get_text(&start, &end, false).unwrap());
                    }
                    Err(_) => {
                        let content = "ERROR";
                        self.webview.get_buffer().unwrap().set_text(&content);
                    }
                }
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
                            if url.starts_with("gemini://") {
                                Go(url)
                            } else {
                                Go(format!("gemini://{}", url))
                            }
                        },
                        placeholder_text: Some("Enter a URL"),
                        width_chars: 40,
                    },
                },
                gtk::ScrolledWindow {
                    #[name="webview"]
                    gtk::TextView {
                        vexpand: true,
                        editable: false,
                    },
                }
                // webkit2gtk::WebView {
                //     vexpand: true,
                //     // decide_policy(_, policy_decision, policy_decision_type) with (open_in_new_window, relm) =>
                //     //     return WebView::decide_policy(&policy_decision, &policy_decision_type, &open_in_new_window, &relm),
                //     // permission_request(_, request) => (PermissionRequest(request.clone()), true),
                // },
            },
            delete_event(_, _) => (Quit, Inhibit(false)),
        }
    }
}

fn main() {
    // let context = WebContext::get_default().unwrap();
    // context.register_uri_scheme("gemini", visit_link);
    Win::run(()).expect("Win::run failed");
}

fn visit_link(url: &str) -> String {
    let new_url = Url::parse(url).unwrap();
    let data = content::get_data(&new_url);
    match data {
        Ok((meta, new_content)) => String::from_utf8_lossy(&new_content).to_string(),
        Err(_) => String::from("ERROR"),
    }
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

// fn gemini2html(content: String, current_url: String) -> String {
//     let link_regexp = Regex::new(LINK_REGEX).unwrap();
//     let h1_regexp = Regex::new(H1_REGEX).unwrap();
//     let h2_regexp = Regex::new(H2_REGEX).unwrap();
//     let h3_regexp = Regex::new(H3_REGEX).unwrap();
//     let ul_regexp = Regex::new(UL_REGEX).unwrap();

//     let mut html = String::from("<!DOCTYPE html><html><body>");

//     for line in content.lines() {
//         if link_regexp.is_match(line) {
//             let caps = link_regexp.captures(&line).unwrap();
//             let dest = caps.get(1).map_or("", |m| m.as_str());
//             let label = caps.get(2).map_or("", |m| m.as_str());
//             html.push_str(&format!(
//                 "<a href={}>{}</a><br/>",
//                 make_absolute(dest, &current_url),
//                 label
//             ));
//         } else if h1_regexp.is_match(line) {
//             let caps = h1_regexp.captures(&line).unwrap();
//             let header = caps.get(1).map_or("", |m| m.as_str());
//             html.push_str(&format!("<h1>{}</h1>", header));
//         } else if h2_regexp.is_match(line) {
//             let caps = h2_regexp.captures(&line).unwrap();
//             let header = caps.get(1).map_or("", |m| m.as_str());
//             html.push_str(&format!("<h2>{}</h2>", header));
//         } else if h3_regexp.is_match(line) {
//             let caps = h3_regexp.captures(&line).unwrap();
//             let header = caps.get(1).map_or("", |m| m.as_str());
//             html.push_str(&format!("<h3>{}</h3>", header));
//         } else if ul_regexp.is_match(line) {
//             let caps = ul_regexp.captures(&line).unwrap();
//             let header = caps.get(1).map_or("", |m| m.as_str());
//             html.push_str(&format!("<ul><li>{}</li></ul>", header));
//         } else if line.is_empty() {
//             html.push_str("<br/>");
//         } else {
//             html.push_str(&format!("{}<br/>", line));
//         }
//     }

//     let html_end = "</body></html>";
//     println!("{}{}", html, html_end);
//     format!("{}{}", html, html_end)
// }

fn parse_gemini(
    content: &String,
    current_url: String,
    current_host: String,
    buffer: gtk::TextBuffer,
) -> gtk::TextBuffer {
    let link_regexp = Regex::new(LINK_REGEX).unwrap();
    let h1_regexp = Regex::new(H1_REGEX).unwrap();
    let h2_regexp = Regex::new(H2_REGEX).unwrap();
    let h3_regexp = Regex::new(H3_REGEX).unwrap();
    let ul_regexp = Regex::new(UL_REGEX).unwrap();

    for line in content.lines() {
        if link_regexp.is_match(line) {
            let caps = link_regexp.captures(&line).unwrap();
            let dest = caps.get(1).map_or("", |m| m.as_str());
            let label = caps.get(2).map_or("", |m| m.as_str());
            let mut end_iter = buffer.get_end_iter();
            buffer.insert(
                &mut end_iter,
                &format!(
                    "Link: {} -> {}\n",
                    label,
                    make_absolute(dest, &current_url, &current_host)
                ),
            );
        } else if h1_regexp.is_match(line) {
            let caps = h1_regexp.captures(&line).unwrap();
            let header = caps.get(1).map_or("", |m| m.as_str());
            let mut end_iter = buffer.get_end_iter();
            buffer.insert(&mut end_iter, &format!("Header 1: {}\n", header));
        } else if h2_regexp.is_match(line) {
            let caps = h2_regexp.captures(&line).unwrap();
            let header = caps.get(1).map_or("", |m| m.as_str());
            let mut end_iter = buffer.get_end_iter();
            buffer.insert(&mut end_iter, &format!("Header 2: {}\n", header));
        } else if h3_regexp.is_match(line) {
            let caps = h3_regexp.captures(&line).unwrap();
            let header = caps.get(1).map_or("", |m| m.as_str());
            let mut end_iter = buffer.get_end_iter();
            buffer.insert(&mut end_iter, &format!("Header 3: {}\n", header));
        } else if ul_regexp.is_match(line) {
            let caps = ul_regexp.captures(&line).unwrap();
            let header = caps.get(1).map_or("", |m| m.as_str());
            let mut end_iter = buffer.get_end_iter();
            buffer.insert(&mut end_iter, &format!("List item: {}\n", header));
        } else if line.is_empty() {
            let mut end_iter = buffer.get_end_iter();
            buffer.insert(&mut end_iter, "\n");
        } else {
            let mut end_iter = buffer.get_end_iter();
            buffer.insert(&mut end_iter, &format!("{}\n", line));
        }
    }
    buffer
}

fn make_absolute(url: &str, current_host: &str, current_url: &str) -> String {
    if url.starts_with("gopher://") {
        String::from(url)
    } else if url.starts_with("http://") {
        String::from(url)
    } else if url.starts_with("https://") {
        String::from(url)
    } else if !current_host.is_empty() {
        if url.starts_with("gemini://") {
            String::from(url)
        } else if url.starts_with("//") {
            format!("gemini:{}", url)
        } else if url.starts_with('/') {
            format!("{}{}", current_host, url)
        } else {
            format!("{}{}", current_url, url)
        }
    } else {
        if url.starts_with("gemini://") {
            String::from(url)
        } else if url.starts_with("//") {
            format!("gemini:{}", url)
        } else {
            format!("gemini://{}", url)
        }
    }
}
