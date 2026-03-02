# browser

Implementation of the [browser.engineering](https://browser.engineering) book — building a fundamental web browser from scratch in Rust.

## Structure

- `src/http.rs` — HTTP client
- `src/parser.rs` — HTML parser
- `src/css.rs` — CSS parser
- `src/layout.rs` — Layout engine
- `src/text_layout.rs` — Text rendering
- `src/gui.rs` — GUI window (minifb)
- `src/url.rs` — URL parsing
- `src/viewsource.rs` — view-source:// scheme

## Build

```bash
cargo build --release
cargo run --bin main    # URL parser / HTTP client
cargo run --bin gui     # GUI browser window
```
