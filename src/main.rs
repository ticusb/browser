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
}
