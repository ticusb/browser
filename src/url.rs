#[derive(Eq, Debug)]
pub struct Url {
    scheme: String,
    host: String,
    url: String,
    path: String,
}

impl Url {
    pub fn new(url: &str) -> Option<Url> {
        if let Some((scheme, rest)) = url.split_once("://") {
            if scheme == "http" {
                let (host, path) = match rest.find('/') {
                    Some(index) => (&rest[..index], &rest[index..]),
                    None => (rest, "/"),
                };

                return Some(Url {
                    scheme: scheme.to_string(),
                    host: host.to_string(),
                    url: format!("{}://{}", scheme, host),
                    path: path.to_string(),
                });
            } else {
                eprintln!("Invalid Url Scheme: {} (Expected: http)", scheme)
            }
        }

        None
    }

    // pub fn new(url: &str) -> Url {
    //     let parts: (&str, &str) = url.split_once("://").unwrap();
    //     let s = parts.0.to_string();
    //     let mut u = parts.1.to_string();
    //     assert!(s == "http");

    //     if !u.contains('/') {
    //         u.push('/');
    //     }

    //     let parts: (&str, &str) = url.split_once('/').unwrap();
    //     let h = parts.0.to_string();
    //     let p = parts.1.to_string();
    //     Url {
    //         scheme: s,
    //         host: h,
    //         url: u,
    //         path: p,
    //     }
    // }

    pub fn get_scheme(&self) -> &str {
        &self.scheme
    }

    pub fn get_host(&self) -> &str {
        &self.host
    }

    pub fn get_url(&self) -> &str {
        &self.url
    }

    pub fn get_path(&self) -> &str {
        &self.path
    }
}

impl PartialEq for Url {
    fn eq(&self, other: &Self) -> bool {
        self.scheme == other.scheme
            && self.host == other.host
            && self.url == other.url
            && self.path == other.path
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_url() {
        let url = Url::new("http://www.example.com/test-path").unwrap();
        assert_eq!(url.get_scheme(), "http");
        assert_eq!(url.get_host(), "www.example.com");
        assert_eq!(url.get_url(), "http://www.example.com");
        assert_eq!(url.get_path(), "/test-path");
    }

    #[test]
    fn test_invalid_url() {
        assert!(Url::new("www.example.com").is_none());
        assert!(Url::new("").is_none());
    }

    #[test]
    fn test_url_equality() {
        let url1 = Url::new("http://www.example.com").unwrap();
        let url2 = Url::new("http://www.example.com").unwrap();
        let url3 = Url::new("https://www.example.com").unwrap();

        assert_eq!(url1, url2);
        assert_ne!(url1, url3);
    }

    // Additional test cases...
}
