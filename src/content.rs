use std::io::{Read, Write};
use tempfile::NamedTempFile;

use native_tls::TlsConnector;
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

pub fn get_data(url: &url::Url) -> Result<(Vec<u8>, Vec<u8>), String> {
    let host = url.host_str().unwrap();
    let urlf = format!("{}:1965", host);

    let mut builder = TlsConnector::builder();
    builder.danger_accept_invalid_hostnames(true);
    builder.danger_accept_invalid_certs(true);
    let connector = builder.build().unwrap();

    match urlf.to_socket_addrs() {
        Ok(mut addrs_iter) => match addrs_iter.next() {
            Some(socket_addr) => {
                let stream = TcpStream::connect_timeout(&socket_addr, Duration::new(5, 0));

                match stream {
                    Ok(stream) => {
                        let mstream = connector.connect(&host, stream);

                        match mstream {
                            Ok(mut stream) => {
                                let url = format!("{}\r\n", url);
                                stream.write_all(url.as_bytes()).unwrap();
                                let mut res = vec![];
                                stream.read_to_end(&mut res).unwrap();

                                let clrf_idx = find_clrf(&res);
                                let content = res.split_off(clrf_idx.unwrap() + 2);

                                Ok((res, content))
                            }
                            Err(e) => Err(format!("Could not connect to {}\n{}", urlf, e)),
                        }
                    }
                    Err(e) => Err(format!("Could not connect to {}\n{}", urlf, e)),
                }
            }
            None => Err(format!("Could not connect to {}", urlf)),
        },
        Err(e) => Err(format!("Could not connect to {}\n{}", urlf, e)),
    }
}

pub fn download(content: Vec<u8>) {
    let path = write_tmp_file(content);
    open::that(path).unwrap();
}

fn write_tmp_file(content: Vec<u8>) -> std::path::PathBuf {
    let mut tmp_file = NamedTempFile::new().unwrap();
    tmp_file.write_all(&content).unwrap();
    let (_file, path) = tmp_file.keep().unwrap();
    path
}

fn find_clrf(data: &[u8]) -> Option<usize> {
    let clrf = b"\r\n";
    data.windows(clrf.len()).position(|window| window == clrf)
}
