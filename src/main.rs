use native_tls::TlsConnector;
use std::env;
use std::io::{self, BufReader, Read, Write};
use std::net::TcpStream;
use std::str::Split;
use std::collections::HashMap;


struct URL {
    scheme: String,
    host: String,
    path: String,
	port: i32
}

impl URL {
    fn new(url: &str) -> Result<Self, &'static str> {
        // Split the scheme
        let mut parts: Split<&str> = url.split("://");
        let scheme = parts.next().ok_or("Invalid URL")?;
        if scheme != "http" && scheme != "https" {
            return Err("Unsupported scheme");
        }

        // Get the rest of the URL
        let remainder = parts.next().ok_or("Invalid URL")?;
        let (host, path) = if let Some(pos) = remainder.find('/') {
            (&remainder[..pos], &remainder[pos..])
        } else {
            (remainder, "/")
        };
		
		let mut host_parts = host.split(':');
		let host = host_parts.next().unwrap_or("");
		let mut port = host_parts.next().and_then(|p| p.parse().ok()).unwrap_or(-1);
        if port == -1 {
            port = if scheme == "https" { 443 } else { 80 };
        }

        Ok(Self {
            scheme: scheme.to_string(),
            host: host.to_string(),
            path: path.to_string(),
			port: port,
        })
    }

    fn request(&self) -> Result<String, io::Error> {
        let addr = format!("{}:{}", self.host, self.port);
		let mut tcp_stream = TcpStream::connect(addr)?;
		let request = format!("GET {} HTTP/1.1\r\nHost: {}\r\n{}\r\n", self.path, self.host, additional_headers());
		let mut response = String::new();

		if self.scheme == "https" {
			let connector = TlsConnector::new().unwrap();
			let mut tls_stream = connector.connect(&self.host, tcp_stream).unwrap();

			tls_stream.write_all(request.as_bytes())?;
			let mut reader = BufReader::new(tls_stream);
			reader.read_to_string(&mut response)?;
		}
		else {
			tcp_stream.write_all(request.as_bytes())?;
			let mut reader = BufReader::new(tcp_stream);
			reader.read_to_string(&mut response)?;
		}

        let mut headers_body_split = response.split("\r\n\r\n");
        let _headers = headers_body_split.next().unwrap(); // Headers as &str
        let body = headers_body_split.next().unwrap(); // Body as &str

        Ok(body.to_string())
    }
}

fn load(url: &URL) {
    match url.request() {
        Ok(body) => show(body),
        Err(e) => eprintln!("Error: {}", e),
    }
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

fn additional_headers() -> String {
    let mut headers = HashMap::new();
    headers.insert("Connection".to_string(), "close".to_string());
    headers.insert("User-Agent".to_string(), "browsa/1.0".to_string());
    
    let mut headers_str = String::new();
    for (key, value) in headers {
        headers_str.push_str(&format!("{}: {}\r\n", key, value));
    }
    
    headers_str
}

fn main() {
    let url_str = "http://browser.engineering/examples/example1-simple.html";
    let mut url = URL::new(url_str);
    match url {
        Ok(ref url) => {
            println!("Scheme: {}", url.scheme);
            println!("Host: {}", url.host);
            println!("Path: {}", url.path);
            println!("Port: {}\r\n", url.port);
        },
        Err(e) => eprintln!("Failed to parse URL: {}", e),
    }
    
    let urll = url.unwrap();
    //load(&urll);
    println!("GET {} HTTP/1.1\r\nHost: {}\r\n{}\r\n", urll.path, urll.host, additional_headers());
}
