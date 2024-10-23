use native_tls::TlsConnector;
use std::env;
use std::io::{self, BufReader, Read, Write};
use std::net::TcpStream;
use std::str::Split;

struct URL {
    scheme: String,
    host: String,
    path: String,
}

impl URL {
    fn new(url: &str) -> Result<Self, &'static str> {
        // Split the scheme
        let mut parts: Split<&str> = url.split("://");
        let scheme = parts.next().ok_or("Invalid URL")?;
        if scheme != "http" {
            return Err("Unsupported scheme");
        }

        // Get the rest of the URL
        let remainder = parts.next().ok_or("Invalid URL")?;
        let (host, path) = if let Some(pos) = remainder.find('/') {
            (&remainder[..pos], &remainder[pos..])
        } else {
            (remainder, "/")
        };

        Ok(Self {
            scheme: scheme.to_string(),
            host: host.to_string(),
            path: path.to_string(),
        })
    }

    fn request(&self) -> Result<String, io::Error> {
        let addr = format!("{}:80", self.host);
        let mut stream = TcpStream::connect(addr)?;

        if self.scheme == "https" {
            let connector = TlsConnector::new().unwrap();
            let stream = connector.connect(&self.host, stream).unwrap();
            // Use `stream` for further communication
        }

        let request = format!("GET {} HTTP/1.0\r\nHost: {}\r\n\r\n", self.path, self.host);

        stream.write_all(request.as_bytes())?;

        let mut reader = BufReader::new(stream);
        let mut response = String::new();

        reader.read_to_string(&mut response)?;

        let mut headers_body_split = response.split("\r\n\r\n");
        let _headers = headers_body_split.next().unwrap(); // Headers as &str
        let body = headers_body_split.next().unwrap(); // Body as &str

        Ok(body.to_string())
    }
}

fn load(url: &URL) {
    let body = url.request().unwrap();
    show(body);
}

fn show(body: String) {
    let mut in_tag = false;

    for c in body.chars() {
        if c == '<' {
            in_tag = true;
        } else if c == '>' {
            in_tag = false;
        } else if in_tag == false {
            print!("{0}", c);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <url>", args[0]);
        std::process::exit(1);
    }

    let url_str = &args[1];
    let url = URL::new(url_str).unwrap();
    load(&url);
}
