mod url;

use url::Url;

fn main() {
    let url_value = Url::new("https://www.poop.com");
    let url = match &url_value {
        Some(_url) => _url,
        None => {
            panic!("Failed to create URL");
        }
    };

    println!("Scheme: {}", url.get_scheme());
    println!("Host: {}", url.get_host());
    println!("URL: {}", url.get_url());
    println!("Path: {}", url.get_path());
}
