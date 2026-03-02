mod css;
mod data;
mod file;
mod http;
mod layout;
mod parser;
mod requestable;
mod text_layout;
mod url;
mod viewsource;

use data::Data;
use file::File;
use parser::{print_tree, HtmlParser};
use requestable::Requestable;
use std::env;
use text_layout::TextLayout;
use url::Url;
use viewsource::ViewSource;

fn load(obj: &dyn Requestable) {
    match obj.request() {
        Ok(body) => {
            if obj.scheme() == "view-source" {
                show_source(&body);
            } else {
                show(&body);
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn show(body: &str) {
    // Parse HTML
    let mut parser = HtmlParser::new(body.to_string());
    let nodes = parser.parse();

    // Option 1: Simple text extraction (Chapter 1 style)
    // let text = extract_text(&nodes);
    // let decoded = decode_entities(&text);
    // println!("{}", decoded);

    // Option 2: Print HTML tree (Chapter 3)
    if env::var("SHOW_TREE").is_ok() {
        println!("=== HTML Tree ===");
        print_tree(&nodes, 0);
        println!();
    }

    // Option 3: Advanced text layout (Chapter 3)
    let mut layout = TextLayout::new(800.0);
    layout.layout(&nodes);

    println!("=== Rendered Text ===");
    for run in layout.display_list() {
        let weight = match run.style.weight {
            css::FontWeight::Bold => "B",
            css::FontWeight::Normal => "N",
        };
        let style = match run.style.style {
            css::FontStyle::Italic => "I",
            css::FontStyle::Normal => "N",
        };
        
        // Simple rendering: show position, style, and text
        println!("({:.0}, {:.0}) [{}{}:{:.0}] {}", 
            run.x, run.y, weight, style, run.style.size, run.text);
    }
}

fn show_source(body: &str) {
    let output = decode_entities(body);
    println!("{}", output);
}

fn decode_entities(body: &str) -> String {
    body.replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&")
        .replace("&quot;", "\"")
        .replace("&apos;", "'")
        .replace("&nbsp;", " ")
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <url>", args[0]);
        eprintln!("Examples:");
        eprintln!("  {} http://example.com", args[0]);
        eprintln!("  {} https://browser.engineering/", args[0]);
        eprintln!("  {} file:///path/to/file.html", args[0]);
        eprintln!("  {} 'data:text/html,<h1>Hello</h1>'", args[0]);
        eprintln!("  {} view-source:http://example.com", args[0]);
        eprintln!();
        eprintln!("Environment variables:");
        eprintln!("  SHOW_TREE=1  Show HTML parse tree");
        std::process::exit(1);
    }

    let request_path = &args[1];

    let result = if request_path.starts_with("http://") || request_path.starts_with("https://") {
        Url::new(request_path)
            .map_err(|e| format!("URL error: {}", e))
            .map(|url| load(&url))
    } else if request_path.starts_with("file://") {
        File::new(request_path)
            .map_err(|e| format!("File error: {}", e))
            .map(|file| load(&file))
    } else if request_path.starts_with("data:") {
        Data::new(request_path)
            .map_err(|e| format!("Data URL error: {}", e))
            .map(|data| load(&data))
    } else if request_path.starts_with("view-source:") {
        ViewSource::new(request_path)
            .map_err(|e| format!("ViewSource error: {}", e))
            .map(|view_source| load(&view_source))
    } else {
        Err("Unsupported scheme. Use http://, https://, file://, data:, or view-source:".to_string())
    };

    if let Err(e) = result {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_entities() {
        let input = "&lt;div&gt;Hello &amp; goodbye&lt;/div&gt;";
        let output = decode_entities(input);
        assert_eq!(output, "<div>Hello & goodbye</div>");
    }
}