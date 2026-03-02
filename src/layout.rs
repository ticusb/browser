pub struct Layout {
    display_list: Vec<(f32, f32, String)>, // x, y, character
    cursor_x: f32,
    cursor_y: f32,
    width: f32,
    height: f32,
    hstep: f32,
    vstep: f32,
}

// Chapter 2 Exercise: Scrolling
pub struct Scrollbar {
    pub position: f32,
    pub viewport_height: f32,
    pub content_height: f32,
}

impl Scrollbar {
    pub fn new(viewport_height: f32, content_height: f32) -> Self {
        Self {
            position: 0.0,
            viewport_height,
            content_height,
        }
    }

    pub fn scroll(&mut self, delta: f32) {
        self.position += delta;
        self.position = self
            .position
            .max(0.0)
            .min(self.content_height - self.viewport_height);
    }

    pub fn scroll_to(&mut self, position: f32) {
        self.position = position
            .max(0.0)
            .min(self.content_height - self.viewport_height);
    }

    pub fn page_up(&mut self) {
        self.scroll(-self.viewport_height);
    }

    pub fn page_down(&mut self) {
        self.scroll(self.viewport_height);
    }

    pub fn get_visible_range(&self) -> (f32, f32) {
        (self.position, self.position + self.viewport_height)
    }
}

// Chapter 2 Exercise: Mouse handling
pub struct MouseEvent {
    pub x: f32,
    pub y: f32,
    pub event_type: MouseEventType,
}

pub enum MouseEventType {
    Click,
    Move,
    Scroll(f32),
}

impl Layout {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            display_list: Vec::new(),
            cursor_x: 13.0,
            cursor_y: 13.0,
            width,
            height,
            hstep: 13.0,  // Horizontal step for characters
            vstep: 18.0,  // Vertical step for lines
        }
    }

    pub fn add_text(&mut self, text: &str) {
        for c in text.chars() {
            self.add_character(c);
        }
    }

    fn add_character(&mut self, c: char) {
        if c == '\n' {
            self.cursor_y += self.vstep;
            self.cursor_x = 13.0;
        } else {
            // Add character to display list
            self.display_list.push((self.cursor_x, self.cursor_y, c.to_string()));
            
            // Move cursor
            self.cursor_x += self.hstep;
            
            // Wrap to next line if needed
            if self.cursor_x >= self.width - 13.0 {
                self.cursor_y += self.vstep;
                self.cursor_x = 13.0;
            }
        }
    }

    pub fn display_list(&self) -> &Vec<(f32, f32, String)> {
        &self.display_list
    }

    pub fn get_height(&self) -> f32 {
        self.cursor_y + self.vstep
    }
}

// For rendering in a scrollable canvas
pub struct Canvas {
    width: f32,
    height: f32,
    scroll: f32,
}

impl Canvas {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
            scroll: 0.0,
        }
    }

    pub fn scroll_down(&mut self, amount: f32, max_height: f32) {
        self.scroll = (self.scroll + amount).min(max_height - self.height).max(0.0);
    }

    pub fn scroll_up(&mut self, amount: f32) {
        self.scroll = (self.scroll - amount).max(0.0);
    }

    pub fn draw(&self, layout: &Layout) -> String {
        let mut output = String::new();
        
        for (x, y, text) in layout.display_list() {
            // Only draw if within visible area
            if *y >= self.scroll && *y < self.scroll + self.height {
                let screen_y = y - self.scroll;
                output.push_str(&format!("({}, {}): {}\n", x, screen_y, text));
            }
        }
        
        output
    }

    pub fn get_scroll(&self) -> f32 {
        self.scroll
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layout_wrapping() {
        let mut layout = Layout::new(100.0, 600.0);
        layout.add_text("Hello World");
        assert!(layout.display_list().len() > 0);
    }

    #[test]
    fn test_newline() {
        let mut layout = Layout::new(800.0, 600.0);
        layout.add_text("Line1\nLine2");
        let list = layout.display_list();
        
        // Find the 'L' from Line2
        let line2_y = list.iter()
            .find(|(_, _, c)| c == "L")
            .map(|(_, y, _)| y)
            .unwrap();
        
        // Should be on a different line
        assert!(*line2_y > 13.0);
    }
}