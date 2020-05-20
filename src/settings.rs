extern crate dirs;

use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;

use serde_derive::Deserialize;

#[derive(Deserialize)]
struct Settings {
    general: Option<General>,
    colors: Option<Color>,
    characters: Option<Character>,
    fonts: Option<Font>,
}

#[derive(Deserialize)]
struct General {
    start_url: Option<String>,
}

#[derive(Deserialize)]
struct Color {
    h1: Option<String>,
    h2: Option<String>,
    h3: Option<String>,
    list: Option<String>,
    text: Option<String>,
    background: Option<String>,
}

#[derive(Deserialize)]
struct Character {
    h1: Option<String>,
    h2: Option<String>,
    h3: Option<String>,
    list: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Font {
    finger: Option<FontAttr>,
    gemini: Option<GeminiFontAttr>,
    gopher: Option<FontAttr>,
}

#[derive(Debug, Deserialize)]
struct FontAttr {
    family: Option<String>,
    style: Option<String>,
    size: Option<i32>,
}

#[derive(Debug, Deserialize)]
struct GeminiFontAttr {
    text: Option<FontAttr>,
    h1: Option<FontAttr>,
    h2: Option<FontAttr>,
    h3: Option<FontAttr>,
    list: Option<FontAttr>,
}

pub fn start_url() -> Option<String> {
    match read().general {
        Some(general) => general.start_url,
        None => None,
    }
}

const DEFAULT_FONT: &str = "serif";
const DEFAULT_FONT_STYLE: &str = "normal";
const DEFAULT_FONT_SIZE: i32 = 11 * pango_sys::PANGO_SCALE;
const DEFAULT_H1_FONT_SIZE: i32 = 16 * pango_sys::PANGO_SCALE;
const DEFAULT_H2_FONT_SIZE: i32 = 13 * pango_sys::PANGO_SCALE;
const DEFAULT_H3_FONT_SIZE: i32 = 12 * pango_sys::PANGO_SCALE;

fn finger_font_family() -> Option<String> {
    read().fonts?.finger?.family
}

fn finger_font_size() -> Option<i32> {
    read().fonts?.finger?.size.or(Some(DEFAULT_FONT_SIZE))
}

pub fn get_finger_font_family() -> String {
    match finger_font_family() {
        Some(family) => family,
        None => String::from(DEFAULT_FONT),
    }
}

pub fn get_finger_font_size() -> i32 {
    match finger_font_size() {
        Some(size) => size * pango_sys::PANGO_SCALE,
        None => DEFAULT_FONT_SIZE,
    }
}

fn gemini_text_font_family() -> Option<String> {
    read().fonts?.gemini?.text?.family
}

pub fn get_gemini_text_font_family() -> String {
    match gemini_text_font_family() {
        Some(family) => family,
        None => String::from(DEFAULT_FONT),
    }
}

fn gemini_text_font_size() -> Option<i32> {
    read().fonts?.gemini?.text?.size
}

pub fn get_gemini_text_font_size() -> i32 {
    match gemini_text_font_size() {
        Some(size) => size * pango_sys::PANGO_SCALE,
        None => DEFAULT_FONT_SIZE,
    }
}

fn gemini_h1_font_family() -> Option<String> {
    read().fonts?.gemini?.h1?.family
}

fn gemini_h1_font_size() -> Option<i32> {
    read().fonts?.gemini?.h1?.size
}

fn gemini_h1_font_style() -> Option<String> {
    read().fonts?.gemini?.h1?.style
}

pub fn get_gemini_h1_font_size() -> i32 {
    match gemini_h1_font_size() {
        Some(size) => size * pango_sys::PANGO_SCALE,
        None => DEFAULT_H1_FONT_SIZE,
    }
}

pub fn get_gemini_h1_font_family() -> String {
    match gemini_h1_font_family() {
        Some(family) => family,
        None => String::from(DEFAULT_FONT),
    }
}

pub fn get_gemini_h1_font_style() -> String {
    match gemini_h1_font_style() {
        Some(style) => style,
        None => String::from(DEFAULT_FONT_STYLE),
    }
}

fn gemini_h2_font_family() -> Option<String> {
    read().fonts?.gemini?.h2?.family
}

fn gemini_h2_font_size() -> Option<i32> {
    read().fonts?.gemini?.h2?.size
}

fn gemini_h2_font_style() -> Option<String> {
    read().fonts?.gemini?.h2?.style
}

pub fn get_gemini_h2_font_size() -> i32 {
    match gemini_h2_font_size() {
        Some(size) => size * pango_sys::PANGO_SCALE,
        None => DEFAULT_H2_FONT_SIZE,
    }
}

pub fn get_gemini_h2_font_family() -> String {
    match gemini_h2_font_family() {
        Some(family) => family,
        None => String::from(DEFAULT_FONT),
    }
}

pub fn get_gemini_h2_font_style() -> String {
    match gemini_h2_font_style() {
        Some(style) => style,
        None => String::from(DEFAULT_FONT_STYLE),
    }
}

fn gemini_h3_font_family() -> Option<String> {
    read().fonts?.gemini?.h3?.family
}

fn gemini_h3_font_size() -> Option<i32> {
    read().fonts?.gemini?.h3?.size
}

fn gemini_h3_font_style() -> Option<String> {
    read().fonts?.gemini?.h3?.style
}

pub fn get_gemini_h3_font_size() -> i32 {
    match gemini_h3_font_size() {
        Some(size) => size * pango_sys::PANGO_SCALE,
        None => DEFAULT_H3_FONT_SIZE,
    }
}

pub fn get_gemini_h3_font_family() -> String {
    match gemini_h3_font_family() {
        Some(family) => family,
        None => String::from(DEFAULT_FONT),
    }
}

pub fn get_gemini_h3_font_style() -> String {
    match gemini_h3_font_style() {
        Some(style) => style,
        None => String::from(DEFAULT_FONT_STYLE),
    }
}

fn gemini_list_font_family() -> Option<String> {
    read().fonts?.gemini?.list?.family
}

fn gemini_list_font_size() -> Option<i32> {
    read().fonts?.gemini?.list?.size
}

fn gemini_list_font_style() -> Option<String> {
    read().fonts?.gemini?.list?.style
}

pub fn get_gemini_list_font_size() -> i32 {
    match gemini_list_font_size() {
        Some(size) => size * pango_sys::PANGO_SCALE,
        None => DEFAULT_FONT_SIZE,
    }
}

pub fn get_gemini_list_font_family() -> String {
    match gemini_list_font_family() {
        Some(family) => family,
        None => String::from(DEFAULT_FONT),
    }
}

pub fn get_gemini_list_font_style() -> String {
    match gemini_list_font_style() {
        Some(style) => style,
        None => String::from(DEFAULT_FONT_STYLE),
    }
}

fn gopher_font_family() -> Option<String> {
    read().fonts?.gopher?.family
}

fn gopher_font_size() -> Option<i32> {
    read().fonts?.gopher?.size
}

pub fn get_gopher_font_family() -> String {
    match gopher_font_family() {
        Some(family) => family,
        None => String::from(DEFAULT_FONT),
    }
}

pub fn get_gopher_font_size() -> i32 {
    match gopher_font_size() {
        Some(size) => size * pango_sys::PANGO_SCALE,
        None => DEFAULT_FONT_SIZE,
    }
}

fn h1_color() -> Option<String> {
    read().colors?.h1
}

pub fn get_h1_color() -> String {
    match h1_color() {
        Some(color) => color,
        None => String::from("#9932CC"),
    }
}

fn h2_color() -> Option<String> {
    read().colors?.h2
}

pub fn get_h2_color() -> String {
    match h2_color() {
        Some(color) => color,
        None => String::from("#FF1493"),
    }
}

fn h3_color() -> Option<String> {
    read().colors?.h3
}

pub fn get_h3_color() -> String {
    match h3_color() {
        Some(color) => color,
        None => String::from("#87CEFA"),
    }
}

fn list_color() -> Option<String> {
    read().colors?.list
}

pub fn get_list_color() -> String {
    match list_color() {
        Some(color) => color,
        None => String::from("green"),
    }
}

fn text_color() -> Option<String> {
    read().colors?.text
}

pub fn get_text_color() -> String {
    match text_color() {
        Some(color) => color,
        None => String::from("black"),
    }
}

pub fn background_color() -> Option<String> {
    read().colors?.background
}

fn h1_character() -> Option<String> {
    read().characters?.h1
}

pub fn get_h1_character() -> String {
    match h1_character() {
        Some(char) => char,
        None => String::new(),
    }
}

fn h2_character() -> Option<String> {
    read().characters?.h2
}

pub fn get_h2_character() -> String {
    match h2_character() {
        Some(char) => char,
        None => String::new(),
    }
}

fn h3_character() -> Option<String> {
    read().characters?.h3
}

pub fn get_h3_character() -> String {
    match h3_character() {
        Some(char) => char,
        None => String::new(),
    }
}

fn list_character() -> Option<String> {
    read().characters?.list
}

pub fn get_list_character() -> String {
    match list_character() {
        Some(char) => char,
        None => String::from("■"),
    }
}

fn read() -> Settings {
    let mut file = settings_file();
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Unable to read file");

    let settings: Settings = toml::from_str(&content).unwrap();
    settings
}

fn settings_file() -> File {
    let mut settings = dirs::config_dir().unwrap();
    settings.push("castor");
    fs::create_dir_all(&settings).unwrap();
    settings.push("settings.toml");
    let file_path = settings.into_os_string();

    OpenOptions::new()
        .create(true)
        .append(true)
        .read(true)
        .open(file_path)
        .expect("file not found")
}
