use crate::requestable::Requestable;
use native_tls::TlsConnector;
use std::io::{self, BufReader, Read, Write};
use std::net::TcpStream;
use std::str::Split;

pub struct Url {
    scheme: String,
    host: String,
    path: String,
    port: i32,
}

impl Url {
    pub fn new(url: &str) -> Result<Self, &'static str> {
        let mut parts: Split<&str> = url.split("://");
        let scheme = parts.next().ok_or("Invalid url")?;

        if scheme != "http" && scheme != "https" {
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
            port = if scheme == "https" { 443 } else { 80 };
        }

        Ok(Self {
            scheme: scheme.to_string(),
            host: host.to_string(),
            path: path.to_string(),
            port
        })
    }
    
    
    fn http_request(&self) -> Result<String, io::Error> {
        let addr = format!("{}:{}", self.host, self.port);
        let mut tcp_stream = TcpStream::connect(addr)?;

        let request = format!(
            "GET {} HTTP/1.1\r\nHost: {}\r\n{}\r\n",
            self.path,
            self.host,
            self.additional_headers()
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
            self.additional_headers()
        );
        let mut response = String::new();

        tls_stream.write_all(request.as_bytes())?;
        let mut reader = BufReader::new(tls_stream);
        reader.read_to_string(&mut response)?;

        Ok(response)
    }
    
    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

impl Requestable for Url {
    fn request(&self) -> Result<String, io::Error> {
        match self.scheme.as_str() {
            "http" => self.http_request(),
            "https" => self.https_request(),
            _ => panic!("Unsupported scheme"),
        }
    }

    fn scheme(&self) -> String {
        String::from(&self.scheme)
    }
}
