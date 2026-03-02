use minifb::{Key, Window, WindowOptions};
use std::env;
use fontdue::{Font, FontSettings};

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
use parser::HtmlParser;
use requestable::Requestable;
use text_layout::TextLayout;
use url::Url;
use viewsource::ViewSource;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

struct Browser {
    window: Window,
    buffer: Vec<u32>,
    scroll: f32,
    content_height: f32,
    font: Font,
    font_bold: Font,
}

impl Browser {
    fn new() -> Self {
        let mut window = Window::new(
            "Browser - Rust Web Browser",
            WIDTH,
            HEIGHT,
            WindowOptions::default(),
        )
        .unwrap_or_else(|e| {
            panic!("Unable to create window: {}", e);
        });

        window.limit_update_rate(Some(std::time::Duration::from_millis(16)));

        // Load embedded fonts
        let font = Font::from_bytes(
            include_bytes!("../fonts/DejaVuSans.ttf") as &[u8],
            FontSettings::default(),
        )
        .expect("Failed to load font");

        let font_bold = Font::from_bytes(
            include_bytes!("../fonts/DejaVuSans-Bold.ttf") as &[u8],
            FontSettings::default(),
        )
        .expect("Failed to load bold font");

        Self {
            window,
            buffer: vec![0xFFFFFF; WIDTH * HEIGHT],
            scroll: 0.0,
            content_height: 0.0,
            font,
            font_bold,
        }
    }

    fn load(&mut self, url_str: &str) {
        // Fetch content
        // 
        println!("{}", url_str);
        let body = if url_str.starts_with("http://") || url_str.starts_with("https://") {
            let url = Url::new(url_str).expect("Invalid URL");
            url.request().expect("Failed to fetch URL")
        } else if url_str.starts_with("file://") {
            let file = File::new(url_str).expect("Invalid file URL");
            file.request().expect("Failed to read file")
        } else if url_str.starts_with("data:") {
            let data = Data::new(url_str).expect("Invalid data URL");
            data.request().expect("Failed to parse data URL")
        } else if url_str.starts_with("view-source:") {
            let view_source = ViewSource::new(url_str).expect("Invalid view-source URL");
            view_source.request().expect("Failed to fetch source")
        } else {
            panic!("Unsupported URL scheme");
        };

        // Parse HTML
        let mut parser = HtmlParser::new(body);
        let nodes = parser.parse();

        // Layout text
        let mut layout = TextLayout::new(WIDTH as f32 - 20.0);
        layout.layout(&nodes);

        self.content_height = layout.get_height();

        // Render to buffer
        self.render_layout(&layout);
    }

    fn render_layout(&mut self, layout: &TextLayout) {
        // Clear buffer (white background)
        self.buffer.fill(0xFFFFFF);

        for run in layout.display_list() {
            let screen_y = run.y - self.scroll;

            // Only render if visible
            if screen_y < -50.0 || screen_y > HEIGHT as f32 {
                continue;
            }

            // Render each character in the word
            let mut x = run.x;
            for ch in run.text.chars() {
                let char_width = self.draw_char(ch, x as i32, screen_y as i32, &run.style);
                x += char_width;
            }
        }
    }

    fn draw_char(&mut self, ch: char, x: i32, y: i32, style: &css::TextStyle) -> f32 {
        // Choose font based on weight
        let font = match style.weight {
            css::FontWeight::Bold => &self.font_bold,
            css::FontWeight::Normal => &self.font,
        };

        let size = style.size;
        
        // Rasterize the character
        let (metrics, bitmap) = font.rasterize(ch, size);

        // Draw the bitmap
        for (bitmap_y, row) in bitmap.chunks(metrics.width).enumerate() {
            for (bitmap_x, &alpha) in row.iter().enumerate() {
                let px = x + bitmap_x as i32 + metrics.xmin;
                let py = y + bitmap_y as i32 + metrics.ymin;

                if px >= 0 && px < WIDTH as i32 && py >= 0 && py < HEIGHT as i32 {
                    let pixel_idx = (py as usize) * WIDTH + (px as usize);
                    
                    // Alpha blending
                    if alpha > 0 {
                        let alpha_norm = alpha as f32 / 255.0;
                        let bg = self.buffer[pixel_idx];
                        let bg_r = ((bg >> 16) & 0xFF) as f32;
                        let bg_g = ((bg >> 8) & 0xFF) as f32;
                        let bg_b = (bg & 0xFF) as f32;

                        // Text color (black)
                        let fg_r = 0.0;
                        let fg_g = 0.0;
                        let fg_b = 0.0;

                        let r = (fg_r * alpha_norm + bg_r * (1.0 - alpha_norm)) as u32;
                        let g = (fg_g * alpha_norm + bg_g * (1.0 - alpha_norm)) as u32;
                        let b = (fg_b * alpha_norm + bg_b * (1.0 - alpha_norm)) as u32;

                        self.buffer[pixel_idx] = (r << 16) | (g << 8) | b;
                    }
                }
            }
        }

        metrics.advance_width
    }

    fn scroll_down(&mut self) {
        self.scroll += 50.0;
        let max_scroll = (self.content_height - HEIGHT as f32).max(0.0);
        self.scroll = self.scroll.min(max_scroll);
    }

    fn scroll_up(&mut self) {
        self.scroll -= 50.0;
        self.scroll = self.scroll.max(0.0);
    }

    fn run(&mut self, url: &str) {
        self.load(url);

        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            let mut needs_redraw = false;

            // Handle scrolling
            if self.window.is_key_pressed(Key::Down, minifb::KeyRepeat::Yes) {
                self.scroll_down();
                needs_redraw = true;
            }
            if self.window.is_key_pressed(Key::Up, minifb::KeyRepeat::Yes) {
                self.scroll_up();
                needs_redraw = true;
            }
            if self.window.is_key_pressed(Key::PageDown, minifb::KeyRepeat::Yes) {
                self.scroll += HEIGHT as f32;
                let max_scroll = (self.content_height - HEIGHT as f32).max(0.0);
                self.scroll = self.scroll.min(max_scroll);
                needs_redraw = true;
            }
            if self.window.is_key_pressed(Key::PageUp, minifb::KeyRepeat::Yes) {
                self.scroll -= HEIGHT as f32;
                self.scroll = self.scroll.max(0.0);
                needs_redraw = true;
            }

            // Re-render if scrolled
            if needs_redraw {
                self.load(url);
            }

            // Update window
            self.window
                .update_with_buffer(&self.buffer, WIDTH, HEIGHT)
                .unwrap();
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <url>", args[0]);
        eprintln!("Examples:");
        eprintln!("  {} http://example.com", args[0]);
        eprintln!("  {} 'data:text/html,<h1>Hello World</h1>'", args[0]);
        eprintln!();
        eprintln!("Controls:");
        eprintln!("  Up/Down arrows: Scroll");
        eprintln!("  Page Up/Down: Scroll by page");
        eprintln!("  ESC: Quit");
        std::process::exit(1);
    }

    let url = &args[1];
    let mut browser = Browser::new();
    browser.run(url);
}