use std::io::{Read, Write};
use tempfile::NamedTempFile;
use std::net::{TcpStream};


pub fn get(url: &url::Url) -> Result<(Option<Vec<u8>>, Vec<u8>), String> {
    let host = url.host_str().unwrap();
    let urlf = format!("{}:70", host);
    println!("{:?}", url.path());

    match TcpStream::connect(&urlf) {
        Ok(mut stream) => {
            let mut url = format!("{}\r\n", url.path());
            let url = if url.starts_with("/0/") || url.starts_with("/1/") {
                url.split_off(2)
            } else {
                url
            };
            stream.write_all(url.as_bytes()).unwrap();
            let mut res = vec![];
            stream.read_to_end(&mut res).unwrap();

            Ok((None, res))
        }
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
