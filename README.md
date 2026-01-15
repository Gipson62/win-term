# win-term
========

Small Windows-only crate to estimate console sizes in pixels.

Why this exists
--------------

On many Windows hosts and terminal emulators, native font-metric APIs
like `GetCurrentConsoleFontEx` either return unusable values or do not
reflect user DPI/zoom. This crate provides a pragmatic, "good enough"
estimate by scaling conservative default font dimensions with the
console window DPI and converting console character dimensions to pixels.

Quick usage
-----------

```rust
use win_term::{get_size_of_the_font, get_size_of_the_terminal};

fn main() {
	if let Ok(font) = get_size_of_the_font() {
		println!("Font cell: {}x{} px", font.width, font.height);
	}
	if let Ok(term) = get_size_of_the_terminal() {
		println!("Terminal: {}x{} px", term.width, term.height);
	}
}
```

Limitations
-----------

- Results are estimates and may be incorrect for custom fonts, per-window
  zoom, or terminal emulators that don't expose accurate metrics.
- This crate is Windows-only and intended for simple, pragmatic use cases.

License: MIT