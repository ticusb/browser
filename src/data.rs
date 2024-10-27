use crate::requestable::Requestable;

use std::io;
use std::str::Split;

pub struct Data {
    scheme: String,
    data_type: String,
    content: String,
}

impl Data {
    pub fn new(data: &str) -> Result<Self, &'static str> {
        let mut parts: Split<&str> = data.split(":");
        let scheme = parts.next().ok_or("Invalid Data")?;

        if scheme != "data" {
            return Err("Unsupported scheme");
        }

        let remainder = parts.next().ok_or("Invalid Data")?;
        parts = remainder.split(",");
        let data_type = parts.next().ok_or("Invalid Data")?;
        let content = parts.next().ok_or("Data Empty")?;

        Ok(Self {
            scheme: scheme.to_string(),
            data_type: data_type.to_string(),
            content: content.to_string(),
        })
    }
}

impl Requestable for Data {
    fn request(&self) -> Result<String, io::Error> {
        Ok(self.content.clone())
    }

    fn scheme(&self) -> String {
        format!("{}:{}", &self.scheme, &self.data_type)
    }
}
