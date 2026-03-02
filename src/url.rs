use crate::http::HttpResponse;
use crate::requestable::Requestable;
use native_tls::TlsConnector;
use std::collections::HashMap;
use std::io::{self, BufReader, Read, Write};
use std::net::TcpStream;
use std::str::Split;

pub struct Url {
    scheme: String,
    host: String,
    path: String,
    port: i32,
}

// Chapter 1 Exercise: Cache implementation
pub struct Cache {
    store: HashMap<String, CacheEntry>,
    max_age: u64,
}

#[derive(Clone)]
struct CacheEntry {
    content: String,
    timestamp: u64,
    headers: HashMap<String, String>,
}

impl Cache {
    pub fn new(max_age: u64) -> Self {
        Self {
            store: HashMap::new(),
            max_age,
        }
    }

    pub fn get(&self, url: &str) -> Option<String> {
        if let Some(entry) = self.store.get(url) {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();

            if now - entry.timestamp < self.max_age {
                return Some(entry.content.clone());
            }
        }
        None
    }

    pub fn set(&mut self, url: String, content: String, headers: HashMap<String, String>) {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        self.store.insert(
            url,
            CacheEntry {
                content,
                timestamp,
                headers,
            },
        );
    }
}

fn additional_headers() -> HashMap<String, String> {
    let mut headers = HashMap::new();
    headers.insert("Connection".to_string(), "close".to_string());
    headers.insert("User-Agent".to_string(), "browsa/1.0".to_string());
    headers.insert("Accept-Encoding".to_string(), "gzip".to_string());
    headers
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
            port,
        })
    }

    fn make_request(&self) -> Result<HttpResponse, io::Error> {
        let raw_response = match self.scheme.as_str() {
            "http" => self.http_request()?,
            "https" => self.https_request()?,
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::Unsupported,
                    "Unsupported scheme",
                ))
            }
        };

        let response = HttpResponse::parse(raw_response)?;
        
        println!("{}", response.body);

        // Handle redirects
        if response.status >= 300 && response.status < 400 {
            if let Some(location) = response.get_header("location") {
                let new_url = self.resolve_url(location);
                let redirected = Url::new(&new_url)
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
                return redirected.make_request();
            }
        }

        Ok(response)
    }

    fn resolve_url(&self, location: &str) -> String {
        if location.starts_with("http://") || location.starts_with("https://") {
            location.to_string()
        } else if location.starts_with("/") {
            format!("{}://{}{}", self.scheme, self.host, location)
        } else {
            let base_path = if let Some(pos) = self.path.rfind('/') {
                &self.path[..=pos]
            } else {
                "/"
            };
            format!("{}://{}{}{}", self.scheme, self.host, base_path, location)
        }
    }

    fn http_request(&self) -> Result<String, io::Error> {
        let addr = format!("{}:{}", self.host, self.port);
        let mut tcp_stream = TcpStream::connect(addr)?;

        let headers = additional_headers();
        let mut headers_str = String::new();
        for (key, value) in headers {
            headers_str.push_str(&format!("{}: {}\r\n", key, value));
        }

        let request = format!(
            "GET {} HTTP/1.1\r\nHost: {}\r\n{}\r\n",
            self.path, self.host, headers_str
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
        

        let connector = TlsConnector::new()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        let mut tls_stream = connector
            .connect(&self.host, tcp_stream)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        let headers = additional_headers();
        let mut headers_str = String::new();
        for (key, value) in headers {
            headers_str.push_str(&format!("{}: {}\r\n", key, value));
        }
        
        println!("{}", headers_str);

        let request = format!(
            "GET {} HTTP/1.1\r\nHost: {}\r\n{}\r\n",
            self.path, self.host, headers_str
        );
        let mut response = String::new();
        
        println!("{}", request);
        
        tls_stream.write_all(request.as_bytes())?;
        let mut reader = BufReader::new(tls_stream);
        reader.read_to_string(&mut response)?;
        
        println!("{}", response);

        Ok(response)
    }
}

impl Requestable for Url {
    fn request(&self) -> Result<String, io::Error> {
        let response = self.make_request()?;
        
        // Handle different content encodings
        if let Some(encoding) = response.get_header("content-encoding") {
            if encoding == "gzip" {
                // Note: For full gzip support, you'd need to add flate2 dependency
                // For now, we'll just return the body as-is
                eprintln!("Warning: gzip encoding detected but not fully supported");
            }
        }

        // Handle transfer encoding
        if let Some(transfer_encoding) = response.get_header("transfer-encoding") {
            if transfer_encoding == "chunked" {
                return Ok(self.decode_chunked(&response.body)?);
            }
        }

        Ok(response.body)
    }

    fn scheme(&self) -> String {
        String::from(&self.scheme)
    }
}

impl Url {
    fn decode_chunked(&self, body: &str) -> Result<String, io::Error> {
        let mut result = String::new();
        let mut remaining = body;

        loop {
            // Find the chunk size line
            let newline_pos = remaining.find("\r\n").ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidData, "Invalid chunked encoding")
            })?;

            let size_str = &remaining[..newline_pos];
            let size = usize::from_str_radix(size_str.trim(), 16).map_err(|_| {
                io::Error::new(io::ErrorKind::InvalidData, "Invalid chunk size")
            })?;

            if size == 0 {
                break;
            }

            remaining = &remaining[newline_pos + 2..];

            if remaining.len() < size {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Chunk size exceeds available data",
                ));
            }

            result.push_str(&remaining[..size]);
            remaining = &remaining[size + 2..]; // Skip chunk and trailing \r\n
        }

        Ok(result)
    }
}