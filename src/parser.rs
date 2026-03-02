use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Text(String),
    Element(Element),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Element {
    pub tag: String,
    pub attributes: HashMap<String, String>,
    pub children: Vec<Node>,
}

impl Element {
    pub fn new(tag: String) -> Self {
        Self {
            tag,
            attributes: HashMap::new(),
            children: Vec::new(),
        }
    }
}

pub struct HtmlParser {
    html: String,
    pos: usize,
}

impl HtmlParser {
    pub fn new(html: String) -> Self {
        Self { html, pos: 0 }
    }

    pub fn parse(&mut self) -> Vec<Node> {
        let mut nodes = Vec::new();
        
        while self.pos < self.html.len() {
            if self.current_char() == '<' {
                if let Some(node) = self.parse_tag() {
                    nodes.push(node);
                }
            } else {
                if let Some(text) = self.parse_text() {
                    nodes.push(Node::Text(text));
                }
            }
        }
        
        nodes
    }

    fn current_char(&self) -> char {
        self.html.chars().nth(self.pos).unwrap_or('\0')
    }

    fn peek(&self, offset: usize) -> char {
        self.html.chars().nth(self.pos + offset).unwrap_or('\0')
    }

    fn parse_text(&mut self) -> Option<String> {
        let start = self.pos;
        
        while self.pos < self.html.len() && self.current_char() != '<' {
            self.pos += 1;
        }
        
        if start == self.pos {
            return None;
        }
        
        let text = self.html[start..self.pos].to_string();
        Some(decode_entities(&text))
    }

    fn parse_tag(&mut self) -> Option<Node> {
        self.pos += 1; // Skip '<'
        
        // Handle comments
        if self.pos + 3 < self.html.len() 
            && &self.html[self.pos..self.pos + 3] == "!--" {
            self.parse_comment();
            return None;
        }

        // Handle doctype
        if self.current_char() == '!' {
            self.skip_until('>');
            self.pos += 1;
            return None;
        }

        // Check for closing tag
        let is_closing = self.current_char() == '/';
        if is_closing {
            self.pos += 1;
        }

        // Parse tag name
        let tag_start = self.pos;
        while self.pos < self.html.len() 
            && !self.current_char().is_whitespace() 
            && self.current_char() != '>' 
            && self.current_char() != '/' {
            self.pos += 1;
        }

        let tag_name = self.html[tag_start..self.pos].to_lowercase();

        // Parse attributes
        let attributes = if !is_closing {
            self.parse_attributes()
        } else {
            HashMap::new()
        };

        // Skip to '>'
        self.skip_whitespace();
        
        // Check for self-closing
        let is_self_closing = self.current_char() == '/';
        if is_self_closing {
            self.pos += 1;
        }

        self.skip_until('>');
        self.pos += 1;

        // Handle self-closing and void elements
        if is_closing {
            return None;
        }

        let is_void = matches!(
            tag_name.as_str(),
            "area" | "base" | "br" | "col" | "embed" | "hr" | "img" | "input" 
            | "link" | "meta" | "param" | "source" | "track" | "wbr"
        );

        let mut element = Element::new(tag_name.clone());
        element.attributes = attributes;

        if is_self_closing || is_void {
            return Some(Node::Element(element));
        }

        // Parse children for non-void elements
        element.children = self.parse_children(&tag_name);

        Some(Node::Element(element))
    }

    fn parse_children(&mut self, parent_tag: &str) -> Vec<Node> {
        let mut children = Vec::new();

        while self.pos < self.html.len() {
            // Check for closing tag
            if self.current_char() == '<' && self.peek(1) == '/' {
                let tag_start = self.pos + 2;
                let mut tag_end = tag_start;
                
                while tag_end < self.html.len() 
                    && !self.html.chars().nth(tag_end).unwrap().is_whitespace()
                    && self.html.chars().nth(tag_end).unwrap() != '>' {
                    tag_end += 1;
                }

                let closing_tag = self.html[tag_start..tag_end].to_lowercase();
                
                if closing_tag == parent_tag {
                    // Skip the closing tag
                    self.skip_until('>');
                    self.pos += 1;
                    break;
                }
            }

            if self.current_char() == '<' {
                if let Some(node) = self.parse_tag() {
                    children.push(node);
                }
            } else {
                if let Some(text) = self.parse_text() {
                    children.push(Node::Text(text));
                }
            }
        }

        children
    }

    fn parse_attributes(&mut self) -> HashMap<String, String> {
        let mut attributes = HashMap::new();

        loop {
            self.skip_whitespace();

            if self.pos >= self.html.len() 
                || self.current_char() == '>' 
                || self.current_char() == '/' {
                break;
            }

            // Parse attribute name
            let name_start = self.pos;
            while self.pos < self.html.len() 
                && !self.current_char().is_whitespace()
                && self.current_char() != '='
                && self.current_char() != '>'
                && self.current_char() != '/' {
                self.pos += 1;
            }

            let name = self.html[name_start..self.pos].to_lowercase();

            self.skip_whitespace();

            // Parse attribute value
            let value = if self.current_char() == '=' {
                self.pos += 1;
                self.skip_whitespace();

                let quote = self.current_char();
                if quote == '"' || quote == '\'' {
                    self.pos += 1;
                    let value_start = self.pos;
                    
                    while self.pos < self.html.len() && self.current_char() != quote {
                        self.pos += 1;
                    }
                    
                    let value = self.html[value_start..self.pos].to_string();
                    self.pos += 1; // Skip closing quote
                    value
                } else {
                    // Unquoted attribute value
                    let value_start = self.pos;
                    while self.pos < self.html.len()
                        && !self.current_char().is_whitespace()
                        && self.current_char() != '>' {
                        self.pos += 1;
                    }
                    self.html[value_start..self.pos].to_string()
                }
            } else {
                // Boolean attribute
                String::new()
            };

            attributes.insert(name, value);
        }

        attributes
    }

    fn parse_comment(&mut self) {
        // Skip "!--"
        self.pos += 3;

        // Find "-->"
        while self.pos + 2 < self.html.len() {
            if &self.html[self.pos..self.pos + 3] == "-->" {
                self.pos += 3;
                return;
            }
            self.pos += 1;
        }
    }

    fn skip_whitespace(&mut self) {
        while self.pos < self.html.len() && self.current_char().is_whitespace() {
            self.pos += 1;
        }
    }

    fn skip_until(&mut self, target: char) {
        while self.pos < self.html.len() && self.current_char() != target {
            self.pos += 1;
        }
    }
}

fn decode_entities(text: &str) -> String {
    text.replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&")
        .replace("&quot;", "\"")
        .replace("&apos;", "'")
        .replace("&nbsp;", " ")
}

// Print tree for debugging
pub fn print_tree(nodes: &[Node], indent: usize) {
    for node in nodes {
        match node {
            Node::Text(text) => {
                let trimmed = text.trim();
                if !trimmed.is_empty() {
                    println!("{}{}", "  ".repeat(indent), trimmed);
                }
            }
            Node::Element(element) => {
                println!("{}<{}>", "  ".repeat(indent), element.tag);
                print_tree(&element.children, indent + 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_html() {
        let html = "<html><body>Hello</body></html>".to_string();
        let mut parser = HtmlParser::new(html);
        let nodes = parser.parse();
        
        assert_eq!(nodes.len(), 1);
    }

    #[test]
    fn test_parse_attributes() {
        let html = r#"<a href="http://example.com" class="link">Link</a>"#.to_string();
        let mut parser = HtmlParser::new(html);
        let nodes = parser.parse();
        
        if let Some(Node::Element(element)) = nodes.first() {
            assert_eq!(element.tag, "a");
            assert_eq!(element.attributes.get("href").unwrap(), "http://example.com");
            assert_eq!(element.attributes.get("class").unwrap(), "link");
        } else {
            panic!("Expected element node");
        }
    }

    #[test]
    fn test_self_closing_tags() {
        let html = "<br/><img src=\"test.jpg\"/>".to_string();
        let mut parser = HtmlParser::new(html);
        let nodes = parser.parse();
        
        assert_eq!(nodes.len(), 2);
    }
}