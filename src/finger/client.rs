use std::io::{Read, Write};
use std::net::TcpStream;
use std::net::{ToSocketAddrs};
use std::time::Duration;

use crate::Protocol;

pub fn get_data<T: Protocol>(url: T) -> Result<(Option<Vec<u8>>, Vec<u8>), String> {
    let url = url.get_source_url();
    let host = url.host_str().unwrap().to_string();
    let username = url.username();
    let urlf = format!("{}:79", host);
    let socket = match urlf.to_socket_addrs() {
        Ok(mut iter) => iter.next(),
        Err(_) => None,
    };

    match socket {
        Some(socket) => match TcpStream::connect_timeout(&socket, Duration::new(5, 0)) {
            Ok(mut stream) => {
                let request = format!("{}\r\n", username);
                stream.write_all(request.as_bytes()).unwrap();
                let mut res = vec![];
                stream.read_to_end(&mut res).unwrap();

                Ok((None, res))
            },
            Err(e) => Err(format!("Could not connect to {}\n{}", urlf, e)),
        },
        None => Err(format!("Could not connect to {}\n", urlf))
    }
}
