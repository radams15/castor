extern crate regex;
use regex::Regex;
use std::str::FromStr;
use url::Url;

#[derive(Debug)]
pub enum Link {
    Gemini(Url, String),
    Gopher(Url, String),
    Http(Url, String),
    Relative(String, String),
    Unknown(Url, String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ParseError;

impl FromStr for Link {
    type Err = ParseError;

    // Parses a &str into an instance of 'Link'
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut els = line.split("\t");

        if line.starts_with('0') || line.starts_with('1') {
            let label = els.next().expect("no label");
            println!("{:?}", label);
            let path = els.next();
            println!("{:?}", path);
            let host = els.next();
            println!("{:?}", host);
            let port = els.next();
            println!("{:?}", port);

            if let Some(host) = host {
                if let Some(path) = path {
                    let mut text = String::from(label);
                    text.remove(0);
                    Ok(Link::Gopher(Url::parse(&format!("gopher://{}{}", host, path)).unwrap(), String::from(text)))
                } else {
                    Err(ParseError)
                }
            } else {
                Err(ParseError)
            }
        } else if line.starts_with('h') {
            let label = els.next();
            println!("{:?}", label);
            let url = els.next();
            println!("{:?}", url);

            if let Some(label) = label {
                if let Some(url) = url {
                    let mut text = String::from(label);
                    text.remove(0);
                    let mut url = String::from(url);
                    let url = url.split_off(4);
                    let url = Url::parse(&url).unwrap();
                    match url.scheme() {
                        "gemini" => Ok(Link::Gemini(url, String::from(text))),
                        "http" => Ok(Link::Http(url, String::from(text))),
                        "https" => Ok(Link::Http(url, String::from(text))),
                        _ => Ok(Link::Unknown(url, String::from(text))),
                    }
                } else {
                    Err(ParseError)
                }
            } else {
                Err(ParseError)
            }
        } else if line.starts_with('[') {
            let label = line;
            println!("{:?}", label);
            let mut url = String::from(line);
            let url = url.split_off(4);
            println!("{:?}", url);
            let url = Url::parse(&url);

            if let Ok(url) = url {
                println!("SCHEME {}", url.scheme());
                match url.scheme() {
                    "gemini" => Ok(Link::Gemini(url, String::from(line))),
                    "gopher" => Ok(Link::Gopher(url, String::from(line))),
                    "http" => Ok(Link::Http(url, String::from(line))),
                    "https" => Ok(Link::Http(url, String::from(line))),
                    _ => Ok(Link::Unknown(url, String::from(line))),
                }
            } else {
                Err(ParseError)
            }
        } else {
            Err(ParseError)
        }

        // match link_regexp.captures(&line) {
        //     Some(caps) => {
        //         let url_str = caps.get(1).map_or("", |m| m.as_str());
        //         let label_str = caps.get(2).map_or("", |m| m.as_str());

        //         let url = url_str.to_string();
        //         let label = if label_str.is_empty() {
        //             url_str.to_string()
        //         } else {
        //             label_str.to_string()
        //         };

        //         match make_link(url, label) {
        //             Some(link) => Ok(link),
        //             None => Err(ParseError),
        //         }
        //     }
        //     None => Err(ParseError),
        // }
    }
}

fn make_link(url: String, label: String) -> Option<Link> {
    let urlp = Url::parse(&url);
    match urlp {
        Ok(url) => match url.scheme() {
            "gemini" => Some(Link::Gemini(url, label)),
            "gopher" => Some(Link::Gopher(url, label)),
            "http" => Some(Link::Http(url, label)),
            "https" => Some(Link::Http(url, label)),
            _ => Some(Link::Unknown(url, label)),
        },
        Err(url::ParseError::RelativeUrlWithoutBase) => Some(Link::Relative(url, label)),
        _ => None,
    }
}
