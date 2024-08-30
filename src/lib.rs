use windows_sys::Win32::{
    Foundation::HANDLE,
    System::Console::{
        GetConsoleScreenBufferInfo, GetConsoleWindow, GetStdHandle, CONSOLE_SCREEN_BUFFER_INFO,
        SMALL_RECT, STD_OUTPUT_HANDLE,
    },
    UI::HiDpi::GetDpiForWindow,
};

/// Struct to hold terminal size information in terms of width and height.
#[derive(Debug)]
pub struct TerminalSize {
    pub width: i32,  // Width of the terminal in pixels
    pub height: i32, // Height of the terminal in pixels
}

/// Struct to hold font size information in terms of width and height.
#[derive(Debug)]
pub struct FontSize {
    pub width: i32,  // Width of a single character in pixels
    pub height: i32, // Height of a single character in pixels
}

/// Enum to represent possible errors that can occur while getting terminal or font size.
#[derive(Debug)]
pub enum TerminalError {
    NoStdHandle,         // Standard output handle not found
    NoScreenBufferInfo,  // Failed to retrieve console screen buffer information
    UnsupportedDpi,      // DPI setting is unsupported (not 96, 120, or 144)
}

/// This function retrieves the font size used by the terminal in pixels.
/// 
/// ## Assumptions:
/// - The font size is set to 12 points, and the font type is "Consolas".
/// - No zooming in or out has been done.
/// - The DPI is set to either 100%, 125%, or 150% scaling (175% is not supported).
/// 
/// ## Returns:
/// - `Ok(FontSize)` with the font width and height in pixels.
/// - `Err(TerminalError)` if there's an issue obtaining the standard handle or the DPI is unsupported.
///
/// ## Note:
/// - The DPI values used are approximations for common scaling settings:
///   - 96 DPI (100% scaling)
///   - 120 DPI (125% scaling)
///   - 144 DPI (150% scaling)
pub fn get_size_of_the_font() -> Result<FontSize, TerminalError> {
    unsafe {
        let h_console: HANDLE = GetStdHandle(STD_OUTPUT_HANDLE);
        if h_console.is_null() {
            return Err(TerminalError::NoStdHandle);
        }
        let pixel_size = match GetDpiForWindow(GetConsoleWindow()) {
            96 => FontSize {
                width: 9,
                height: 20
            },
            120 => FontSize {
                width: 12,
                height: 25
            },
            144 => FontSize {
                width: 14,
                height: 32
            },
            _ => return Err(TerminalError::UnsupportedDpi)
        };
        return Ok(pixel_size);
    }
}

/// This function retrieves the size of the terminal window in pixels.
/// 
/// ## Assumptions:
/// - The font size is set to 12 points, and the font type is "Consolas".
/// - No zooming in or out has been done.
/// - The DPI is set to either 100%, 125%, or 150% scaling (175% is not supported).
/// 
/// ## Returns:
/// - `Ok(TerminalSize)` with the terminal's width and height in pixels.
/// - `Err(TerminalError)` if there's an issue obtaining the standard handle, retrieving screen buffer info, or the DPI is unsupported.
///
/// ## Note:
/// - The DPI values used are approximations for common scaling settings:
///   - 96 DPI (100% scaling)
///   - 120 DPI (125% scaling)
///   - 144 DPI (150% scaling)
pub fn get_size_of_the_terminal() -> Result<TerminalSize, TerminalError> {
    unsafe {
        let h_console: HANDLE = GetStdHandle(STD_OUTPUT_HANDLE);
        if h_console.is_null() {
            return Err(TerminalError::NoStdHandle);
        }

        let mut info = CONSOLE_SCREEN_BUFFER_INFO {
            dwSize: windows_sys::Win32::System::Console::COORD { X: 0, Y: 0 },
            dwCursorPosition: windows_sys::Win32::System::Console::COORD { X: 0, Y: 0 },
            wAttributes: 0,
            srWindow: SMALL_RECT {
                Left: 0,
                Top: 0,
                Right: 0,
                Bottom: 0,
            },
            dwMaximumWindowSize: windows_sys::Win32::System::Console::COORD { X: 0, Y: 0 },
        };
        if GetConsoleScreenBufferInfo(h_console, &mut info) == 0 {
            return Err(TerminalError::NoScreenBufferInfo);
        }
        let pixel_size = match GetDpiForWindow(GetConsoleWindow()) {
            96 => TerminalSize {
                width: 9 * info.dwSize.X as i32,
                height: 20 * info.dwSize.Y as i32,
            },
            120 => TerminalSize {
                width: 12 * info.dwSize.X as i32,
                height: 25 * info.dwSize.Y as i32,
            },
            144 => TerminalSize {
                width: 14 * info.dwSize.X as i32,
                height: 32 * info.dwSize.Y as i32
            },
            _ => return Err(TerminalError::UnsupportedDpi)
        };
        return Ok(pixel_size);
    }
}