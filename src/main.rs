mod url;

use url::Url;

fn main() {
    let url_value = Url::new("http://www.poop.com/toilet");
    let url = match &url_value {
        Some(_url) => _url,
        None => {
            panic!("Failed to create URL");
        }
    };

    println!("URL: {}", url.get_url());
    println!("Scheme: {}", url.get_scheme());
    println!("Host: {}", url.get_host());
    println!("Path: {}", url.get_path());
}
