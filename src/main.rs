use std::io::{BufRead, Write};

fn main() {
    let listener = std::net::TcpListener::bind("172.25.39.143:8080").unwrap();
    for mut stream in listener.incoming().flatten() {
        let mut rdr = std::io::BufReader::new(&mut stream);
        let mut l = String::new();
        rdr.read_line(&mut l).unwrap();
        match l.trim().split(' ').collect::<Vec<_>>().as_slice() {
            ["GET" , resource, "HTTP/1.1"] => {
                loop {
                    let mut l = String::new();
                    rdr.read_line(&mut l).unwrap();
                    if l.trim().is_empty() {
                        break;
                    }
                }
                let mut p = std::path::PathBuf::new();
                // p.push("web");
                p.push(resource.trim_start_matches('/'));
                if resource.ends_with('/') {
                    p.push("index.html");
                }
                stream.write_all(b"HTTP/1.1 200 OK\r\n\r\n").unwrap();
                stream.write_all(&std::fs::read(p).unwrap()).unwrap();
            }
            _ => {
                stream.write_all(b"HTTP/1.1 400 Bad Request\r\n\r\n").unwrap();
            }
        }
    }
}
