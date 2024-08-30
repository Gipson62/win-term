use windows_sys::Win32::{
    Foundation::HANDLE,
    Graphics::Gdi::{GetDC, GetDeviceCaps, ReleaseDC, LOGPIXELSX, LOGPIXELSY},
    System::Console::{
        GetConsoleScreenBufferInfo, GetConsoleWindow, GetStdHandle, CONSOLE_SCREEN_BUFFER_INFO,
        SMALL_RECT, STD_OUTPUT_HANDLE,
    },
    UI::HiDpi::GetDpiForWindow,
};

fn main() {
    unsafe {
        let h_console: HANDLE = GetStdHandle(STD_OUTPUT_HANDLE);
        if h_console.is_null() {
            eprintln!("error get_std_handle");
            return;
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
            eprintln!("error getting screen buffer info");
            return;
        }
        //that's the height for font size 12 (should be by default for the consolas font)
        let pixel_size = match GetDpiForWindow(GetConsoleWindow()) {
            // A font that's 12 point size should convert to 16 pixels when you have 96 DPI
            // But we have to account for the spacing between lines ~4 pixels
            // But we have to account for the spacing between chars ~2 pixels
            96  => (20 * info.dwSize.Y as i32, 9 * info.dwSize.X as i32),
            120 => (25 * info.dwSize.Y as i32, 12 * info.dwSize.X as i32),
            144 => (32 * info.dwSize.Y as i32, 14 * info.dwSize.X as i32),
            _ => unimplemented!("This DPI shouldn't exist"),
        };

        println!("size: {} x {}", pixel_size.1, pixel_size.0);
        // formula to get the height of the font in pixel with a 96 DPI. `font_size*(1.0 + 1.0/3.0)`
        // for each DPI you get a scaling of 1.5 and the width seems to be half the height
    }
}
