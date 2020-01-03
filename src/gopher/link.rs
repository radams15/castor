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
        let mut els = line.split('\t');

        if line.starts_with('0') || line.starts_with('1') {
            let label = els.next().expect("no label");
            let path = els.next();
            let host = els.next();
            let port = els.next();

            if let Some(host) = host {
                if let Some(p) = path {
                    let mut text = String::from(label);
                    text.remove(0);

                    let path = if p.starts_with('/') {
                        p.to_string()
                    } else {
                        format!("/{}", p)
                    };

                    if let Some(port) = port {
                      match Url::parse(&format!("gopher://{}:{}{}", host, port, path)) {
                          Ok(url) => Ok(Link::Gopher(url, text)),
                          Err(e) => {
                              println!("ERR {:?}", e);
                              Err(ParseError)
                          }
                      }
                    } else {
                        Err(ParseError)
                    }
                } else {
                    Err(ParseError)
                }
            } else {
                Err(ParseError)
            }
        } else if line.starts_with('h') {
            let label = els.next();
            let url = els.next();

            if let Some(label) = label {
                if let Some(url) = url {
                    let mut label = String::from(label);
                    label.remove(0);
                    let url = String::from(url);
                    match make_link(url, label) {
                        Some(link) => Ok(link),
                        None => Err(ParseError),
                    }
                } else {
                    Err(ParseError)
                }
            } else {
                Err(ParseError)
            }
        } else if line.starts_with('[') {
            let mut url = String::from(line);
            let url = url.split_off(4);
            let label = String::from(line);

            match make_link(url, label) {
                Some(link) => Ok(link),
                None => Err(ParseError),
            }
        } else {
            Err(ParseError)
        }
    }
}

fn make_link(url: String, label: String) -> Option<Link> {
    match Url::parse(&url) {
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
