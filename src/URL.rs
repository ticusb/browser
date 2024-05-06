pub struct URL {
    scheme: String,
    host: String,
    url: String,
    path: String,
}

impl URL {
    pub fn new(url: &str) -> URL {
        let parts: (&str, &str) = url.split_once("://").unwrap();
        let scheme = parts.0.to_string();
        let mut url = parts.1.to_string();
        assert!(scheme == "http");

        if !url.contains('/') {
            url.push('/');
        }

        let parts: (&str, &str) = url.split_once("/").unwrap();
        let hostca = parts.0.to_string();
        let path = parts.1.to_string();
        URL {
            scheme: scheme,
            host: host,
            url: url,
            path: path,
        }
    }

    pub fn get_scheme(&self) -> &str {
        return &self.scheme;
    }

    pub fn get_host(&self) -> &str {
        return &self.host;
    }

    pub fn get_url(&self) -> &str {
        return &self.url;
    }

    pub fn get_path(&self) -> &str {
        return &self.path;
    }
}
