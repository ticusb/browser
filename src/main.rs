mod data;
mod file;
mod requestable;
mod url;
mod viewsource;

use data::Data;
use file::File;
use requestable::Requestable;
use std::env;
use url::Url;
use viewsource::ViewSource;

fn load(obj: &dyn Requestable) {
    match obj.request() {
        Ok(body) => {
            if obj.scheme() == "view-source" {
                show_source(body);
            } else {
                show(body);
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn show(body: String) {
    let mut in_tag = false;
    let mut output = String::new();

    for c in body.chars() {
        if c == '<' {
            in_tag = true;
        } else if c == '>' {
            in_tag = false;
        } else if in_tag == false {
            output.push(c);
        }
    }

    output = decode_entities(&output);
    println!("{}", output);
}

fn show_source(body: String) {
    let output = decode_entities(&body);
    println!("{}", output);
}

fn decode_entities(body: &String) -> String {
    body.replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&")
        .replace("&quot;", "\"")
        .replace("&apos;", "'")
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <url>", args[0]);
        std::process::exit(1);
    }

    let request_path = &args[1];

    if request_path.starts_with("http://") || request_path.starts_with("https://") {
        let url = Url::new(request_path).unwrap();
        load(&url);
        return;
    } else if request_path.starts_with("file://") {
        let file = File::new(request_path).unwrap();
        load(&file);
        return;
    } else if request_path.starts_with("data:") {
        let data = Data::new(request_path).unwrap();
        load(&data);
        return;
    } else if request_path.starts_with("view-source:") {
        let view_source = ViewSource::new(request_path).unwrap();
        load(&view_source);
    } else {
        eprintln!("Unsupported scheme");
        std::process::exit(1);
    }
}
