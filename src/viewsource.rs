use crate::requestable::Requestable;
use crate::url::Url;
use std::io;

pub struct ViewSource {
    scheme: String,
    url: Url,
}

impl ViewSource {
    pub fn new(view_source: &str) -> Result<Self, &'static str> {
        if !view_source.starts_with("view-source:") {
            return Err("Invalid view-source URL");
        }

        let url_str = &view_source[12..]; // Skip "view-source:"
        let url = Url::new(url_str)?;

        Ok(Self {
            scheme: "view-source".to_string(),
            url,
        })
    }
}

impl Requestable for ViewSource {
    fn request(&self) -> Result<String, io::Error> {
        self.url.request()
    }

    fn scheme(&self) -> String {
        String::from(&self.scheme)
    }
}