use std::collections::HashMap;
use std::io;

#[derive(Debug)]
pub struct HttpResponse {
    pub version: String,
    pub status: u16,
    pub explanation: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl HttpResponse {
    pub fn parse(response: String) -> Result<Self, io::Error> {
        // Split headers and body
        let parts: Vec<&str> = response.splitn(2, "\r\n\r\n").collect();
        
        if parts.len() < 2 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid HTTP response format",
            ));
        }

        let header_section = parts[0];
        let body = parts[1].to_string();

        // Parse headers
        let mut lines = header_section.lines();
        
        // Parse status line
        let status_line = lines.next().ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidData, "Missing status line")
        })?;

        let status_parts: Vec<&str> = status_line.splitn(3, ' ').collect();
        if status_parts.len() < 3 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid status line",
            ));
        }

        let version = status_parts[0].to_string();
        let status = status_parts[1].parse::<u16>().map_err(|_| {
            io::Error::new(io::ErrorKind::InvalidData, "Invalid status code")
        })?;
        let explanation = status_parts[2].to_string();

        // Parse headers
        let mut headers = HashMap::new();
        for line in lines {
            if let Some(pos) = line.find(':') {
                let key = line[..pos].trim().to_lowercase();
                let value = line[pos + 1..].trim().to_string();
                headers.insert(key, value);
            }
        }

        Ok(HttpResponse {
            version,
            status,
            explanation,
            headers,
            body,
        })
    }

    pub fn get_header(&self, key: &str) -> Option<&String> {
        self.headers.get(&key.to_lowercase())
    }
}