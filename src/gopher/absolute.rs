use url::Url;

pub fn make(url: &str) -> Result<url::Url, url::ParseError> {
    // Creates an absolute link if needed
    match crate::history::get_current_host() {
        Some(host) => {
            if url.starts_with("gopher://") {
                Url::parse(url)
            } else if url.starts_with("//") {
                Url::parse(&format!("gopher:{}", url))
            } else if url.starts_with('/') {
                Url::parse(&format!("gopher://{}{}", host, url))
            } else {
                let current_host_path = crate::history::get_current_url().unwrap();
                Url::parse(&format!("{}{}", current_host_path, url))
            }
        }
        None => {
            if url.starts_with("gopher://") {
                Url::parse(url)
            } else if url.starts_with("//") {
                Url::parse(&format!("gopher:{}", url))
            } else {
                Url::parse(&format!("gopher://{}", url))
            }
        }
    }
}

#[test]
fn test_make_absolute_full_url() {
    crate::history::append("gopher://typed-hole.org");
    let url = "gopher://typed-hole.org/foo";
    let expected_url = Url::parse("gopher://typed-hole.org/foo").unwrap();
    let absolute_url = make(&url).unwrap();
    assert_eq!(expected_url, absolute_url);
}
#[test]
fn test_make_absolute_full_url_no_protocol() {
    crate::history::append("gopher://typed-hole.org");
    let url = "//typed-hole.org/foo";
    let expected_url = Url::parse("gopher://typed-hole.org/foo").unwrap();
    let absolute_url = make(&url).unwrap();
    assert_eq!(expected_url, absolute_url);
}
#[test]
fn test_make_absolute_slash_path() {
    crate::history::append("gopher://typed-hole.org");
    let url = "/foo";
    let expected_url = Url::parse("gopher://typed-hole.org/foo").unwrap();
    let absolute_url = make(&url).unwrap();
    assert_eq!(expected_url, absolute_url);
}
#[test]
fn test_make_absolute_just_path() {
    crate::history::append("gopher://typed-hole.org");
    let url = "foo";
    let expected_url = Url::parse("gopher://typed-hole.org/foo").unwrap();
    let absolute_url = make(&url).unwrap();
    assert_eq!(expected_url, absolute_url);
}
#[test]
fn test_make_absolute_full_url_no_current_host() {
    let url = "gopher://typed-hole.org/foo";
    let expected_url = Url::parse("gopher://typed-hole.org/foo").unwrap();
    let absolute_url = make(&url).unwrap();
    assert_eq!(expected_url, absolute_url);
}
#[test]
fn test_make_absolute_full_url_no_protocol_no_current_host() {
    let url = "//typed-hole.org/foo";
    let expected_url = Url::parse("gopher://typed-hole.org/foo").unwrap();
    let absolute_url = make(&url).unwrap();
    assert_eq!(expected_url, absolute_url);
}
#[test]
fn test_make_absolute_slash_path_no_current_host() {
    let url = "/foo";
    let expected_url = Url::parse("gopher://typed-hole.org/foo").unwrap();
    let absolute_url = make(&url).unwrap();
    assert_eq!(expected_url, absolute_url);
}
#[test]
fn test_make_absolute_just_path_no_current_host() {
    let url = "foo";
    let expected_url = Url::parse("gopher://typed-hole.org/foo").unwrap();
    let absolute_url = make(&url).unwrap();
    assert_eq!(expected_url, absolute_url);
}
