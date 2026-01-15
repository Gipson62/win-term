//! Small Windows-only helper to estimate console pixel sizes.
//!
//! Why this exists
//! ----------------
//! `GetCurrentConsoleFontEx` and other font-metric APIs are unreliable
//! on many Windows terminal hosts and emulators (they may return unusable
//! values or not reflect user DPI/zoom). This crate provides a "good
//! enough" estimation by using conservative defaults and the current
//! DPI to compute per-character pixel sizes, then converting console
//! character dimensions into pixels.
//!
//! Short summary
//! - `get_size_of_the_font()` returns an estimated font cell size in pixels.
//! - `get_size_of_the_terminal()` returns a pixel-size estimate for the
//!   console by multiplying the estimated font size by console character
//!   dimensions.
//!
//! Limitations: values are approximations and may be incorrect for
//! custom fonts, per-window zoom, or terminal emulators that do not
//! expose accurate metrics.

use windows_sys::Win32::{
    Foundation::HANDLE,
    System::Console::{
        GetConsoleScreenBufferInfo, GetConsoleWindow, GetStdHandle, CONSOLE_SCREEN_BUFFER_INFO,
        COORD, SMALL_RECT, STD_OUTPUT_HANDLE,
    },
    UI::HiDpi::GetDpiForWindow,
};
#[cfg(target_os = "windows")]
/// Default font width for Windows console on 96 DPI (approx. Consolas 12pt)
pub static mut DEFAULT_FONT_SIZE_WIDTH: u16 = 10;
#[cfg(target_os = "windows")]
/// Default font height for Windows console on 96 DPI (approx. Consolas 12pt)
pub static mut DEFAULT_FONT_SIZE_HEIGHT: u16 = 22;

/// Struct to hold terminal size information in terms of width and height.
#[derive(Debug)]
pub struct TerminalSize {
    /// Width of the terminal in pixels
    pub width: i32,
    /// Height of the terminal in pixels
    pub height: i32,
}

/// Struct to hold font size information in terms of width and height.
#[derive(Debug)]
pub struct FontSize {
    /// Width of a single character in pixels
    pub width: i32,
    /// Height of a single character in pixels
    pub height: i32,
}

/// Enum to represent possible errors that can occur while getting terminal or font size.
#[derive(Debug)]
pub enum TerminalError {
    /// Standard output handle not found
    NoStdHandle,
    /// Failed to retrieve console screen buffer information
    NoScreenBufferInfo,
}

/// Estimate the console font cell size in pixels.
///
/// Implementation details
/// - The function uses conservative default font cell dimensions
///   (`DEFAULT_FONT_SIZE_WIDTH` / `DEFAULT_FONT_SIZE_HEIGHT`) that roughly
///   match Consolas 12pt at 96 DPI, then scales them using the DPI value
///   returned by `GetDpiForWindow` for the console window.
/// - This approach is used because querying `GetCurrentConsoleFontEx`
///   often returns unusable metrics on many terminal hosts; the result
///   here is explicitly an estimate and may not match the actual font
///   used by the terminal.
///
/// Returns
/// - `Ok(FontSize)` with the estimated font cell size in pixels.
/// - `Err(TerminalError::NoStdHandle)` if the standard output handle
///   cannot be obtained.
pub fn get_size_of_the_font() -> Result<FontSize, TerminalError> {
    unsafe {
        let h_console: HANDLE = GetStdHandle(STD_OUTPUT_HANDLE);
        if h_console.is_null() {
            return Err(TerminalError::NoStdHandle);
        }
        let dpi = GetDpiForWindow(GetConsoleWindow());
        let scale = dpi as f32 / 96.0;
        let (width, height) = (
            (DEFAULT_FONT_SIZE_WIDTH as f32 * scale).round() as u16,
            (DEFAULT_FONT_SIZE_HEIGHT as f32 * scale).round() as u16,
        );
        // Failed to retrieve console screen buffer information
        return Ok(FontSize {
            width: width as i32,
            height: height as i32,
        });
    }
}

/// Estimate the terminal window size in pixels.
///
/// Implementation details
/// - This function reads the console screen buffer information and uses the
///   estimated font cell pixel size (from `get_size_of_the_font()`) to
///   convert character dimensions into pixels.
/// - Note: the implementation multiplies the estimated font cell size by
///   the console buffer dimensions returned in `CONSOLE_SCREEN_BUFFER_INFO`.
///   That value may represent the full buffer size (not the visible
///   client window) depending on the host. The returned pixel size is an
///   estimate intended to be "good enough" for most simple use cases.
///
/// Returns
/// - `Ok(TerminalSize)` with the estimated terminal width and height in pixels.
/// - `Err(TerminalError::NoStdHandle)` or `Err(TerminalError::NoScreenBufferInfo)`
///   if native calls fail.
pub fn get_size_of_the_terminal() -> Result<TerminalSize, TerminalError> {
    unsafe {
        let h_console: HANDLE = GetStdHandle(STD_OUTPUT_HANDLE);
        if h_console.is_null() {
            return Err(TerminalError::NoStdHandle);
        }

        let mut info = CONSOLE_SCREEN_BUFFER_INFO {
            dwSize: COORD { X: 0, Y: 0 },
            dwCursorPosition: COORD { X: 0, Y: 0 },
            wAttributes: 0,
            srWindow: SMALL_RECT {
                Left: 0,
                Top: 0,
                Right: 0,
                Bottom: 0,
            },
            dwMaximumWindowSize: COORD { X: 0, Y: 0 },
        };
        if GetConsoleScreenBufferInfo(h_console, &mut info) == 0 {
            return Err(TerminalError::NoScreenBufferInfo);
        }
        let terminal_size = get_size_of_the_font()?;
        let pixel_size = TerminalSize {
            width: terminal_size.width * info.dwSize.X as i32,
            height: terminal_size.height * info.dwSize.Y as i32,
        };
        return Ok(pixel_size);
    }
}
