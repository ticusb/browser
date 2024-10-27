use crate::requestable::Requestable;
use std::io;
use std::io::Read;
use std::str::Split;

pub struct File {
    scheme: String,
    path: String,
}

impl File {
    pub fn new(file: &str) -> Result<Self, &'static str> {
        let mut parts: Split<&str> = file.split("://");
        let scheme = parts.next().ok_or("Invalid file")?;
        if scheme != "file" {
            return Err("Unsupported scheme");
        }

        let path = parts.next().ok_or("Invalid file path")?;

        Ok(Self {
            scheme: scheme.to_string(),
            path: path.to_string(),
        })
    }
}

impl Requestable for File {
    fn request(&self) -> Result<String, io::Error> {
        let path = &self.path[1..];
        let mut file = std::fs::File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        Ok(contents)
    }

    fn scheme(&self) -> &str {
        &self.scheme
    }
}
