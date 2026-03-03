use windows::Win32::System::Threading::{OpenProcess, PROCESS_ALL_ACCESS};
use windows::Win32::System::Memory::{VirtualAllocEx, MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE_READWRITE};
use windows::Win32::System::Diagnostics::ToolHelp::{CreateToolhelp32Snapshot, Process32First, Process32Next, PROCESSENTRY32, TH32CS_SNAPPROCESS};
use windows::Win32::Foundation::CloseHandle;
use std::ffi::CStr;

/// This POC performs actions that are typically flagged by AV/EDR scanners (Process Injection techniques).
/// IT DOES NOT DO ANYTHING HARMFUL. It simply attempts to allocate memory in a target process (notepad.exe)
/// to demonstrate how scanners flag certain API calling patterns.
fn main() {
    println!("[*] Starting Suspicious POC (Process Injection Simulation)...");

    match find_process_id("notepad.exe") {
        Some(pid) => {
            println!("[+] Found notepad.exe (PID: {})", pid);
            unsafe {
                let handle_res = OpenProcess(PROCESS_ALL_ACCESS, false, pid);
                if let Ok(handle) = handle_res {
                    println!("[+] Successfully opened handle to notepad.exe");
                    
                    // Suspicious Call 1: VirtualAllocEx (Allocating executable memory in another process)
                    let addr = VirtualAllocEx(handle, None, 1024, MEM_COMMIT | MEM_RESERVE, PAGE_EXECUTE_READWRITE);
                    
                    if !addr.is_null() {
                        println!("[+] Successfully allocated RXW memory at {:?} in remote process!", addr);
                        println!("[!] WARNING: This pattern (OpenProcess + VirtualAllocEx) is a major detection trigger.");
                    } else {
                        eprintln!("[-] VirtualAllocEx failed. Access denied?");
                    }
                    
                    let _ = CloseHandle(handle);
                } else {
                    eprintln!("[-] Failed to open notepad.exe. Try running notepad first.");
                }
            }
        },
        None => {
            println!("[-] notepad.exe not found. Please open Notepad to see the detection simulation in action.");
        }
    }

    println!("[*] POC Finished. If scanned by VirusTotal, this binary will likely be flagged due to these API patterns.");
    println!("\n[!] Press Enter to exit...");
    let mut _pause = String::new();
    let _ = std::io::stdin().read_line(&mut _pause);
}

fn find_process_id(name: &str) -> Option<u32> {
    unsafe {
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0).ok()?;
        let mut entry = PROCESSENTRY32::default();
        entry.dwSize = std::mem::size_of::<PROCESSENTRY32>() as u32;

        if Process32First(snapshot, &mut entry).is_ok() {
            loop {
                let current_name = CStr::from_ptr(entry.szExeFile.as_ptr() as *const i8).to_string_lossy();
                if current_name.to_lowercase() == name.to_lowercase() {
                    let _ = CloseHandle(snapshot);
                    return Some(entry.th32ProcessID);
                }
                if Process32Next(snapshot, &mut entry).is_err() {
                    break;
                }
            }
        }
        let _ = CloseHandle(snapshot);
        None
    }
}
