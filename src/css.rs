use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct TextStyle {
    pub weight: FontWeight,
    pub style: FontStyle,
    pub size: f32,
    pub family: String,
}

impl Default for TextStyle {
    fn default() -> Self {
        Self {
            weight: FontWeight::Normal,
            style: FontStyle::Normal,
            size: 16.0,
            family: "Times".to_string(),
        }
    }
}

impl TextStyle {
    pub fn from_tag(tag: &str) -> Self {
        let mut style = Self::default();

        match tag {
            "b" | "strong" => style.weight = FontWeight::Bold,
            "i" | "em" => style.style = FontStyle::Italic,
            "h1" => {
                style.size = 32.0;
                style.weight = FontWeight::Bold;
            }
            "h2" => {
                style.size = 24.0;
                style.weight = FontWeight::Bold;
            }
            "h3" => {
                style.size = 19.0;
                style.weight = FontWeight::Bold;
            }
            "h4" => {
                style.size = 16.0;
                style.weight = FontWeight::Bold;
            }
            "h5" => {
                style.size = 13.0;
                style.weight = FontWeight::Bold;
            }
            "h6" => {
                style.size = 11.0;
                style.weight = FontWeight::Bold;
            }
            "small" => style.size = 12.0,
            "big" => style.size = 20.0,
            "code" | "pre" => style.family = "Courier".to_string(),
            _ => {}
        }

        style
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_css() {
        let css = "body { color: red; font-size: 16px; }".to_string();
        let mut parser = CssParser::new(css);
        let rules = parser.parse();

        assert_eq!(rules.len(), 1);
        assert_eq!(rules[0].selector, "body");
        assert_eq!(rules[0].properties.get("color").unwrap(), "red");
        assert_eq!(rules[0].properties.get("font-size").unwrap(), "16px");
    }

    #[test]
    fn test_multiple_rules() {
        let css = "h1 { font-size: 32px; } p { color: blue; }".to_string();
        let mut parser = CssParser::new(css);
        let rules = parser.parse();

        assert_eq!(rules.len(), 2);
    }
}
pub struct CssRule {
    pub selector: String,
    pub properties: HashMap<String, String>,
}

pub struct CssParser {
    css: String,
    pos: usize,
}

impl CssParser {
    pub fn new(css: String) -> Self {
        Self { css, pos: 0 }
    }

    pub fn parse(&mut self) -> Vec<CssRule> {
        let mut rules = Vec::new();

        while self.pos < self.css.len() {
            self.skip_whitespace();

            if self.pos >= self.css.len() {
                break;
            }

            if let Some(rule) = self.parse_rule() {
                rules.push(rule);
            }
        }

        rules
    }

    fn parse_rule(&mut self) -> Option<CssRule> {
        // Parse selector
        let selector = self.parse_selector()?;

        self.skip_whitespace();

        if self.current_char() != '{' {
            return None;
        }
        self.pos += 1;

        // Parse properties
        let properties = self.parse_properties();

        Some(CssRule {
            selector,
            properties,
        })
    }

    fn parse_selector(&mut self) -> Option<String> {
        let start = self.pos;

        while self.pos < self.css.len() && self.current_char() != '{' {
            self.pos += 1;
        }

        if start == self.pos {
            return None;
        }

        Some(self.css[start..self.pos].trim().to_string())
    }

    fn parse_properties(&mut self) -> HashMap<String, String> {
        let mut properties = HashMap::new();

        loop {
            self.skip_whitespace();

            if self.pos >= self.css.len() || self.current_char() == '}' {
                self.pos += 1; // Skip '}'
                break;
            }

            // Parse property name
            let name_start = self.pos;
            while self.pos < self.css.len()
                && self.current_char() != ':'
                && self.current_char() != '}'
            {
                self.pos += 1;
            }

            if self.current_char() != ':' {
                break;
            }

            let name = self.css[name_start..self.pos].trim().to_string();
            self.pos += 1; // Skip ':'

            self.skip_whitespace();

            // Parse property value
            let value_start = self.pos;
            while self.pos < self.css.len()
                && self.current_char() != ';'
                && self.current_char() != '}'
            {
                self.pos += 1;
            }

            let value = self.css[value_start..self.pos].trim().to_string();

            if self.current_char() == ';' {
                self.pos += 1;
            }

            properties.insert(name, value);
        }

        properties
    }

    fn current_char(&self) -> char {
        self.css.chars().nth(self.pos).unwrap_or('\0')
    }

    fn skip_whitespace(&mut self) {
        while self.pos < self.css.len() && self.current_char().is_whitespace() {
            self.pos += 1;
        }
    }
}

// Font and styling
#[derive(Debug, Clone, PartialEq)]
pub enum FontWeight {
    Normal,
    Bold,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FontStyle {
    Normal,
    Italic,
}
