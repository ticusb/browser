use crate::requestable::Requestable;
use native_tls::TlsConnector;
use std::collections::HashMap;
use std::io::{self, BufReader, Read, Write};
use std::net::TcpStream;
use std::str::Split;

pub struct ViewSource {
    scheme: String,
    protocol: String,
    host: String,
    path: String,
    port: i32,
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

impl ViewSource {
    pub fn new(view_source: &str) -> Result<Self, &'static str> {
        let mut parts: Split<&str> = view_source.split(":");
        let scheme = parts.next().ok_or("Invalid ViewSource")?;

        if scheme != "view-source" {
            return Err("Unsupported scheme");
        }

        let url = view_source.replace("view-source:", "");
        parts = url.split("://");
        let protocol = parts.next().ok_or("Invalid url")?;
        if protocol != "http" && protocol != "https" {
            return Err("Unsupported scheme");
        }

        let remainder = parts.next().ok_or("Invalid url")?;
        let (host, path) = if let Some(pos) = remainder.find('/') {
            (&remainder[..pos], &remainder[pos..])
        } else {
            (remainder, "/")
        };

        let mut host_parts = host.split(':');
        let host = host_parts.next().unwrap_or("");
        let mut port = host_parts.next().and_then(|p| p.parse().ok()).unwrap_or(-1);
        if port == -1 {
            port = if protocol == "https" { 443 } else { 80 };
        }

        Ok(Self {
            scheme: scheme.to_string(),
            protocol: protocol.to_string(),
            host: host.to_string(),
            path: path.to_string(),
            port,
        })
    }

    fn http_request(&self) -> Result<String, io::Error> {
        let addr = format!("{}:{}", self.host, self.port);
        let mut tcp_stream = TcpStream::connect(addr)?;

        let request = format!(
            "GET {} HTTP/1.1\r\nHost: {}\r\n{}\r\n",
            self.path,
            self.host,
            additional_headers()
        );
        let mut response = String::new();

        tcp_stream.write_all(request.as_bytes())?;
        let mut reader = BufReader::new(tcp_stream);
        reader.read_to_string(&mut response)?;

        Ok(response)
    }

    fn https_request(&self) -> Result<String, io::Error> {
        let addr = format!("{}:{}", self.host, self.port);
        let tcp_stream = TcpStream::connect(addr)?;

        let connector = TlsConnector::new().unwrap();
        let mut tls_stream = connector.connect(&self.host, tcp_stream).unwrap();

        let request = format!(
            "GET {} HTTP/1.1\r\nHost: {}\r\n{}\r\n",
            self.path,
            self.host,
            additional_headers()
        );
        let mut response = String::new();

        tls_stream.write_all(request.as_bytes())?;
        let mut reader = BufReader::new(tls_stream);
        reader.read_to_string(&mut response)?;

        Ok(response)
    }
}

impl Requestable for ViewSource {
    fn request(&self) -> Result<String, io::Error> {
        match self.protocol.as_str() {
            "http" => self.http_request(),
            "https" => self.https_request(),
            _ => panic!("Unsupported scheme"),
        }
    }

    fn scheme(&self) -> String {
        String::from(&self.scheme)
    }
}
