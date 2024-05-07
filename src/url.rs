pub struct Url {
    scheme: String,
    host: String,
    url: String,
    path: String,
}

impl Url {
    pub fn new(url: &str) -> Url {
        let parts: (&str, &str) = url.split_once("://").unwrap();
        let s = parts.0.to_string();
        let mut u = parts.1.to_string();
        assert!(s == "http");

        if !u.contains('/') {
            u.push('/');
        }

        let parts: (&str, &str) = url.split_once('/').unwrap();
        let h = parts.0.to_string();
        let p = parts.1.to_string();
        Url {
            scheme: s,
            host: h,
            url: u,
            path: p,
        }
    }

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
