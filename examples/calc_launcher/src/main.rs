use std::process::Command;

fn main() {
    println!("Launching calculator...");
    let _ = Command::new("calc.exe").spawn();
}
