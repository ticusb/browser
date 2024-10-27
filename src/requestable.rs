use std::collections::HashMap;
use std::io;

pub trait Requestable {
    fn request(&self) -> Result<String, io::Error>;
    fn scheme(&self) -> String;
    
    fn additional_headers(&self) -> String {
        let mut headers = HashMap::new();
        headers.insert("Connection".to_string(), "close".to_string());
        headers.insert("User-Agent".to_string(), "browsa/1.0".to_string());

        let mut headers_str = String::new();
        for (key, value) in headers {
            headers_str.push_str(&format!("{}: {}\r\n", key, value));
        }

        headers_str
    }
}
