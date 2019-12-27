 use std::str::FromStr;


#[derive(Debug)]
pub enum TextElement {
    ExternalLinkItem(String),
    LinkItem(String),
    Image(String),
    Text(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ParseError;

impl FromStr for TextElement {
    type Err = ParseError;

    // Parses a &str into an instance of 'TextElement'
    fn from_str(line: &str) -> Result<TextElement, ParseError> {
        if line.starts_with('0') {
            Ok(TextElement::LinkItem(String::from(line)))
        } else if line.starts_with('1') {
            Ok(TextElement::LinkItem(String::from(line)))
        } else if line.starts_with('i') {
            let mut els = line.split("\t");
            let text = els.next().expect("");
            let mut text = String::from(text);
            text.remove(0);
            Ok(TextElement::Text(String::from(text)))
        } else if line.starts_with('h') {
            Ok(TextElement::ExternalLinkItem(String::from(line)))
        } else if line.starts_with('I') {
            let mut els = line.split("\t");
            let text = els.next().expect("");
            let mut text = String::from(text);
            text.remove(0);
            Ok(TextElement::Image(String::from(text)))
        } else {
            Ok(TextElement::Text(String::from(line)))
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
