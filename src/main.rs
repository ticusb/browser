mod url;

use url::Url;

fn main() {
    let url = Url::new("http://www.poop.com");
    println!("scheme: {}", url.get_scheme());
    println!("host: {}", url.get_host());
    println!("URL: {}", url.get_url());
    println!("path: {}", url.get_path());
}
