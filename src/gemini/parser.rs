extern crate regex;
use regex::Regex;

use crate::colors::*;

use std::str::FromStr;

#[derive(Debug)]
pub enum TextElement {
    H1(String),
    H2(String),
    H3(String),
    ListItem(String),
    LinkItem(String),
    Text(String),
    MonoText(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ParseError;

const H1_REGEX: &str = r"^#\s+(.*)$";
const H2_REGEX: &str = r"^##\s+(.*)$";
const H3_REGEX: &str = r"^###\s+(.*)$";
const LIST_ITEM_REGEX: &str = r"^\*\s+([^*]*)$";
const LINK_ITEM_REGEX: &str = r"^=>\s*(\S*)\s*(.*)?$";

impl FromStr for TextElement {
    type Err = ParseError;

    // Parses a &str into an instance of 'TextElement'
    fn from_str(line: &str) -> Result<TextElement, ParseError> {
        let h1_regexp = Regex::new(H1_REGEX).unwrap();
        let h2_regexp = Regex::new(H2_REGEX).unwrap();
        let h3_regexp = Regex::new(H3_REGEX).unwrap();
        let list_item_regexp = Regex::new(LIST_ITEM_REGEX).unwrap();
        let link_item_regexp = Regex::new(LINK_ITEM_REGEX).unwrap();

        if h1_regexp.is_match(&line) {
            let caps = h1_regexp.captures(&line).unwrap();
            let header = caps.get(1).map_or("", |m| m.as_str());
            Ok(TextElement::H1(String::from(header)))
        } else if h2_regexp.is_match(&line) {
            let caps = h2_regexp.captures(&line).unwrap();
            let header = caps.get(1).map_or("", |m| m.as_str());
            Ok(TextElement::H2(String::from(header)))
        } else if h3_regexp.is_match(&line) {
            let caps = h3_regexp.captures(&line).unwrap();
            let header = caps.get(1).map_or("", |m| m.as_str());
            Ok(TextElement::H3(String::from(header)))
        } else if list_item_regexp.is_match(&line) {
            let caps = list_item_regexp.captures(&line).unwrap();
            let header = caps.get(1).map_or("", |m| m.as_str());
            Ok(TextElement::ListItem(String::from(header)))
        } else if link_item_regexp.is_match(&line) {
            Ok(TextElement::LinkItem(String::from(line)))
        } else if line.starts_with("```") {
            Ok(TextElement::MonoText(String::from(line)))
        } else {
            Ok(TextElement::Text(colors::colorize(line)))
        }
    }
}

pub fn parse(content: String) -> Vec<Result<TextElement, ParseError>> {
    let mut parsed = Vec::new();

    for line in content.lines() {
        parsed.push(TextElement::from_str(line));
    }
    parsed
}
