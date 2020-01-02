use std::io::{Read, Write};
use std::net::TcpStream;
use percent_encoding::percent_decode;

use crate::Protocol;

pub fn get_data<T: Protocol>(url: T) -> Result<(Option<Vec<u8>>, Vec<u8>), String> {
    let url = url.get_source_url();
    let host = url.host_str().unwrap().to_string();
    let urlf = format!("{}:70", host);

    match TcpStream::connect(&urlf) {
        Ok(mut stream) => {
            let mut url = match url.query() {
                Some(query) => format!("{}?{}\r\n", url.path(), query),
                None => format!("{}\r\n", url.path())
            };

            let url = if url.starts_with("/0/") || url.starts_with("/1/") {
                url.split_off(2)
            } else {
                url
            };

            let url = percent_decode(url.as_bytes()).decode_utf8().unwrap();
            stream.write_all(url.as_bytes()).unwrap();
            let mut res = vec![];
            stream.read_to_end(&mut res).unwrap();

            Ok((None, res))
        }
        Err(e) => Err(format!("Could not connect to {}\n{}", urlf, e)),
    }
}
