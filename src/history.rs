use std::sync::Mutex;
use url::Url;

lazy_static! {
    static ref HISTORY: Mutex<Vec<Url>> = Mutex::new(vec![]);
}

pub fn append(url: &str) {
    let url = Url::parse(url).unwrap();
    HISTORY.lock().unwrap().push(url)
}

pub fn get_current_host() -> Option<String> {
    let history = HISTORY.lock().unwrap();
    match history.last() {
        Some(current_url) => match current_url.host_str() {
            Some(host) => Some(String::from(host)),
            None => None,
        },
        None => None,
    }
}

pub fn get_current_url() -> Option<String> {
    let history = HISTORY.lock().unwrap();
    match history.last() {
        Some(current_url) => {
            let current_path = current_url.join("./");
            match current_path {
                Ok(path) => Some(path.to_string()),
                Err(_) => None,
            }
        }
        None => None
    }
}

pub fn get_previous_url() -> Option<Url> {
    let mut history = HISTORY.lock().unwrap();

    if history.len() > 1 {
        history.pop(); // remove current

        if history.len() > 1 {
            history.pop() // return previous
        } else {
            history.iter().cloned().last()
        }
    } else {
        None
    }
}

#[cfg(test)]
pub(crate) fn clear() -> () {
    HISTORY.lock().unwrap().clear();
}
