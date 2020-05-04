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
    monospace: Option<Monospace>,
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

#[derive(Deserialize)]
struct Monospace {
    finger: Option<bool>,
    gemini: Option<bool>,
    gopher: Option<bool>,
}

pub fn start_url() -> Option<String> {
    match read().general {
        Some(general) => general.start_url,
        None => None,
    }
}

pub fn h1_color() -> String {
    match read().colors {
        Some(colors) => match colors.h1 {
            Some(color) => color,
            None => String::from("#9932CC"),
        },
        None => String::from("#9932CC"),
    }
}

pub fn h2_color() -> String {
    match read().colors {
        Some(colors) => match colors.h2 {
            Some(color) => color,
            None => String::from("#FF1493"),
        },
        None => String::from("#FF1493"),
    }
}

pub fn h3_color() -> String {
    match read().colors {
        Some(colors) => match colors.h3 {
            Some(color) => color,
            None => String::from("#87CEFA"),
        },
        None => String::from("#87CEFA"),
    }
}

pub fn list_color() -> String {
    match read().colors {
        Some(colors) => match colors.list {
            Some(color) => color,
            None => String::from("green"),
        },
        None => String::from("green"),
    }
}

pub fn text_color() -> String {
    match read().colors {
        Some(colors) => match colors.text {
            Some(color) => color,
            None => String::from("black"),
        },
        None => String::from("black"),
    }
}

pub fn background_color() -> Option<String> {
    match read().colors {
        Some(colors) => colors.background,
        None => None,
    }
}

pub fn h1_character() -> String {
    match read().characters {
        Some(characters) => match characters.h1 {
            Some(character) => character,
            None => String::new(),
        },
        None => String::new(),
    }
}

pub fn h2_character() -> String {
    match read().characters {
        Some(characters) => match characters.h2 {
            Some(character) => character,
            None => String::new(),
        },
        None => String::new(),
    }
}

pub fn h3_character() -> String {
    match read().characters {
        Some(characters) => match characters.h3 {
            Some(character) => character,
            None => String::new(),
        },
        None => String::new(),
    }
}

pub fn list_character() -> String {
    match read().characters {
        Some(characters) => match characters.list {
            Some(character) => character,
            None => String::from("■"),
        },
        None => String::from("■"),
    }
}

pub fn finger_monospace() -> bool {
    match read().monospace {
        Some(monospace) => match monospace.finger {
            Some(setting) => setting,
            None => true,
        },
        None => true,
    }
}

pub fn gemini_monospace() -> bool {
    match read().monospace {
        Some(monospace) => match monospace.gemini {
            Some(setting) => setting,
            None => true,
        },
        None => true,
    }
}

pub fn gopher_monospace() -> bool {
    match read().monospace {
        Some(monospace) => match monospace.gopher {
            Some(setting) => setting,
            None => true,
        },
        None => true,
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
