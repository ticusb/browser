use crate::css::{FontWeight, TextStyle};
use crate::parser::{Element, Node};

#[derive(Debug, Clone)]
pub struct TextRun {
    pub x: f32,
    pub y: f32,
    pub text: String,
    pub style: TextStyle,
}

pub struct TextLayout {
    display_list: Vec<TextRun>,
    cursor_x: f32,
    cursor_y: f32,
    width: f32,
    margin: f32,
    line_height_multiplier: f32,
}

// Chapter 3 Exercise: Handle soft hyphens
fn handle_soft_hyphens(text: &str) -> String {
    // Soft hyphen (&shy; or \u{00AD}) should only display when breaking a word
    text.replace('\u{00AD}', "")
}

// Chapter 3 Exercise: Handle preformatted text
fn handle_preformatted_text(text: &str, preserve_whitespace: bool) -> String {
    if preserve_whitespace {
        text.to_string()
    } else {
        // Collapse whitespace
        let mut result = String::new();
        let mut prev_was_whitespace = false;

        for c in text.chars() {
            if c.is_whitespace() {
                if !prev_was_whitespace {
                    result.push(' ');
                    prev_was_whitespace = true;
                }
            } else {
                result.push(c);
                prev_was_whitespace = false;
            }
        }

        result.trim().to_string()
    }
}

impl TextLayout {
    pub fn new(width: f32) -> Self {
        Self {
            display_list: Vec::new(),
            cursor_x: 13.0,
            cursor_y: 13.0,
            width,
            margin: 13.0,
            line_height_multiplier: 1.25,
        }
    }

    pub fn layout(&mut self, nodes: &[Node]) {
        let default_style = TextStyle::default();
        self.layout_nodes(nodes, &default_style);
    }

    fn layout_nodes(&mut self, nodes: &[Node], parent_style: &TextStyle) {
        for node in nodes {
            match node {
                Node::Text(text) => {
                    self.layout_text(text, parent_style);
                }
                Node::Element(element) => {
                    self.layout_element(element, parent_style);
                }
            }
        }
    }

    fn layout_element(&mut self, element: &Element, parent_style: &TextStyle) {
        // Handle block-level elements
        let is_block = matches!(
            element.tag.as_str(),
            "p" | "div" | "h1" | "h2" | "h3" | "h4" | "h5" | "h6" 
            | "ul" | "ol" | "li" | "blockquote" | "pre"
        );

        if is_block && self.cursor_x > self.margin {
            // Start on a new line
            self.cursor_y += parent_style.size * self.line_height_multiplier;
            self.cursor_x = self.margin;
        }

        // Get style for this element
        let mut style = parent_style.clone();
        let tag_style = TextStyle::from_tag(&element.tag);
        style.weight = tag_style.weight;
        style.style = tag_style.style;
        style.size = tag_style.size;
        style.family = tag_style.family;

        // Handle special elements
        match element.tag.as_str() {
            "br" => {
                self.cursor_y += style.size * self.line_height_multiplier;
                self.cursor_x = self.margin;
                return;
            }
            "hr" => {
                self.cursor_y += style.size * self.line_height_multiplier;
                self.cursor_x = self.margin;
                self.cursor_y += style.size * self.line_height_multiplier;
                return;
            }
            _ => {}
        }

        // Layout children
        self.layout_nodes(&element.children, &style);

        // Add spacing after block elements
        if is_block {
            self.cursor_y += style.size * self.line_height_multiplier;
            self.cursor_x = self.margin;
        }
    }

    fn layout_text(&mut self, text: &str, style: &TextStyle) {
        // Apply soft hyphen handling and whitespace processing
        let processed_text = handle_soft_hyphens(text);
        let final_text = handle_preformatted_text(&processed_text, false);
        
        // Split text into words
        for word in final_text.split_whitespace() {
            let word_width = self.measure_word(word, style);
            
            // Check if we need to wrap
            if self.cursor_x + word_width >= self.width - self.margin {
                self.cursor_y += style.size * self.line_height_multiplier;
                self.cursor_x = self.margin;
            }

            // Add the word to display list
            self.display_list.push(TextRun {
                x: self.cursor_x,
                y: self.cursor_y,
                text: word.to_string(),
                style: style.clone(),
            });

            // Move cursor
            self.cursor_x += word_width;

            // Add space width
            let space_width = self.measure_char(' ', style);
            self.cursor_x += space_width;
        }
    }

    fn measure_word(&self, word: &str, style: &TextStyle) -> f32 {
        let mut width = 0.0;
        for c in word.chars() {
            width += self.measure_char(c, style);
        }
        width
    }

    fn measure_char(&self, _c: char, style: &TextStyle) -> f32 {
        // Simplified character width calculation
        // In a real browser, this would use actual font metrics
        let base_width = style.size * 0.6;
        
        match style.weight {
            FontWeight::Bold => base_width * 1.1,
            FontWeight::Normal => base_width,
        }
    }

    pub fn display_list(&self) -> &Vec<TextRun> {
        &self.display_list
    }

    pub fn get_height(&self) -> f32 {
        self.cursor_y + 18.0
    }
}

// Simple layout mode that just extracts text (for backwards compatibility)
pub fn extract_text(nodes: &[Node]) -> String {
    let mut text = String::new();
    extract_text_recursive(nodes, &mut text);
    text
}

fn extract_text_recursive(nodes: &[Node], output: &mut String) {
    for node in nodes {
        match node {
            Node::Text(text) => {
                output.push_str(text);
            }
            Node::Element(element) => {
                // Add newlines for block elements
                let is_block = matches!(
                    element.tag.as_str(),
                    "p" | "div" | "h1" | "h2" | "h3" | "h4" | "h5" | "h6" 
                    | "br" | "li"
                );

                if is_block && !output.is_empty() && !output.ends_with('\n') {
                    output.push('\n');
                }

                extract_text_recursive(&element.children, output);

                if is_block && !output.ends_with('\n') {
                    output.push('\n');
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::HtmlParser;

    #[test]
    fn test_text_layout() {
        let html = "<p>Hello World</p>".to_string();
        let mut parser = HtmlParser::new(html);
        let nodes = parser.parse();
        
        let mut layout = TextLayout::new(800.0);
        layout.layout(&nodes);
        
        assert!(layout.display_list().len() > 0);
    }

    #[test]
    fn test_word_wrapping() {
        let html = "<p>This is a very long line of text that should wrap to multiple lines when the width is limited</p>".to_string();
        let mut parser = HtmlParser::new(html);
        let nodes = parser.parse();
        
        let mut layout = TextLayout::new(200.0);
        layout.layout(&nodes);
        
        // Should have multiple y positions (multiple lines)
        let unique_y: std::collections::HashSet<_> = 
            layout.display_list().iter().map(|run| run.y as i32).collect();
        assert!(unique_y.len() > 1);
    }
}