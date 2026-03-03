# 🦀 Rust Crypter v1.0.0

![Rust](https://img.shields.io/badge/language-Rust-orange.svg) 
![License](https://img.shields.io/badge/license-Educational-blue.svg)
![Status](https://img.shields.io/badge/status-Production--Ready-success.svg)
![Detection](https://img.shields.io/badge/Detection-3%2F71-brightgreen.svg)

A high-performance, stealth-focused Rust crypter designed for Windows environments. This framework utilizes **AES-128 encryption**, **reflective memory loading**, and **advanced behavioral evasion** to facilitate the secure execution of PE files without disk interaction.

---

## 🛠 Advanced Stealth Features

### 🛡️ Core Evasion Strategy
- **Fileless Execution (Reflective Loading)**: Utilizes a custom memory loader (`memexec`) to execute the decrypted payload directly in RAM, leaving zero traces on the physical disk.
- **Dynamic API Camouflage (IAT Padding)**: Automatically pads the *Import Address Table* with dozens of legitimate Windows APIs. This makes the binary appear as a complex, standard utility to heuristic and AI-based scanners (CrowdStrike, Symantec).
- **Environmental Fingerprinting (Anti-Sandbox)**: 
    - Detects restricted sandbox environments by checking for hardware specs (CPU core count < 2).
    - Implements **Busy-Loop Behavioral Delays** instead of standard `Sleep()`, which confuses automated analysis timeouts.

### 🎭 Polymorphic Engine
- **Low-Entropy Metallic Bloating**: Generates 35MB-55MB of unique, repetitive "Log-like" resource data per build. This lowers the file's entropy, bypassing AI/ML scanners that flag high-entropy (encrypted/packed) files.
- **Benign Persona Impersonation**: Each build randomly adopts the metadata identity of trusted software such as **Spotify**, **OneDrive**, **NVIDIA**, **Teams**, or **Discord**.

### 🧩 Anti-Analysis & Reversing
- **API-less Debugger Check**: Directly walks the **Process Environment Block (PEB)** via inline assembly (`GS:[0x60]`) to detect debuggers without calling traceable Win32 APIs.
- **Decoy String Injection**: Saturates the binary with hundreds of innocent system strings, log messages, and error codes to frustrate static analysis and string-grepping.

---

## 🏗 Project Architecture

The project is structured as a modular Rust Workspace for maximum extensibility:

- **`common/`**: Shared cryptographic primitives and obfuscation logic.
- **`builder/`**: The modern CLI orchestrator that handles payload encryption and automated stub compilation.
- **`stub/`**: The "Ghost" delivery vehicle—an ultra-quiet, highly-obfuscated loader optimized for minimum visibility.
- **`examples/`**: 
    - `hello_world`: Verifies memory execution works.
    - `calc_launcher`: A classic proof-of-concept.
    - `suspicious_poc`: A noisy PE that mimics process injection to test detection bypass.

---

## 🚀 Quick Start Guide

### 1. Requirements
- [Rust & Cargo](https://rustup.rs/) (Stable)
- Target: `x86_64-pc-windows-msvc`

### 2. Prepare your Payload
Build your target executable first. For example, using our built-in POC:
```powershell
cargo build -p suspicious_poc --release
```

### 3. Generate the Stealth Stub
Use the builder to encrypt your payload and compile the final obfuscated binary:
```powershell
# -i: Input path | -b: Build and compile stub
cargo run -p builder -- -i target\release\suspicious_poc.exe -b
```

### 4. Deployment
The final, stealthy executable is ready at:
📂 `target\release\stub.exe` (Note: It will be renamed according to the random persona, e.g., `SpotifyUpdate.exe`).

---

## ⚖️ Legal Disclaimer
**This project is for educational and authorized security testing purposes only.** 
The author is NOT responsible for any misuse or illegal activities conducted with this tool. Unauthorized use on systems you do not own is strictly prohibited.

---
*Created with 🦀 by the Rust Stealth Community.*
