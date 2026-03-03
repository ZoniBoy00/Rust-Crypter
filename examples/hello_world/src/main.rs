use windows::Win32::UI::WindowsAndMessaging::{MessageBoxA, MB_OK, MB_ICONINFORMATION};
use windows::core::s;

fn main() {
    unsafe {
        MessageBoxA(
            None,
            s!("Hello! This payload was successfully decrypted and executed from memory."),
            s!("Rust Crypter - Success"),
            MB_OK | MB_ICONINFORMATION,
        );
    }
}
