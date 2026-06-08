use std::env;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

pub fn run() {
    let current_exe = match env::current_exe() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error: Could not determine current executable path: {}", e);
            std::process::exit(1);
        }
    };

    let install_dir = target_dir();
    let install_path = install_dir.join(binary_name());

    // Check if already installed at target
    if current_exe == install_path {
        println!("✓ Already installed at: {}", install_path.display());
        return;
    }

    println!("🎮 Couchpad — Install\n");
    println!("  Current location: {}", current_exe.display());
    println!("  Install to:       {}", install_path.display());
    println!();
    print!("  Proceed? [Y/n] ");
    std::io::stdout().flush().ok();

    let mut input = String::new();
    if std::io::stdin().read_line(&mut input).is_ok() {
        let input = input.trim().to_lowercase();
        if !input.is_empty() && input != "y" && input != "yes" {
            println!("  Cancelled.");
            return;
        }
    }

    // Create install directory if needed
    if !install_dir.exists() {
        if let Err(e) = fs::create_dir_all(&install_dir) {
            eprintln!("  Error: Could not create {}: {}", install_dir.display(), e);
            eprintln!("  Try: sudo {} --install", current_exe.display());
            std::process::exit(1);
        }
    }

    // Copy binary
    if let Err(e) = fs::copy(&current_exe, &install_path) {
        eprintln!("  Error: Could not copy to {}: {}", install_path.display(), e);
        eprintln!("  Try: sudo {} --install", current_exe.display());
        std::process::exit(1);
    }

    // Set executable permission on Unix
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = fs::set_permissions(&install_path, fs::Permissions::from_mode(0o755));
    }

    println!("  ✓ Installed to: {}", install_path.display());

    // Check if install dir is in PATH
    let path_var = env::var("PATH").unwrap_or_default();
    let in_path = path_var.split(if cfg!(windows) { ';' } else { ':' })
        .any(|p| PathBuf::from(p) == install_dir);

    if !in_path {
        println!();
        println!("  ⚠ {} is not in your PATH.", install_dir.display());
        if cfg!(target_os = "macos") {
            println!("  Add this to your shell profile (~/.zshrc or ~/.bashrc):");
            println!("    export PATH=\"{}:$PATH\"", install_dir.display());
        } else {
            println!("  Add {} to your system PATH.", install_dir.display());
        }
    }

    println!("\n  Run 'couchpad' to get started.");
}

#[cfg(target_os = "macos")]
fn target_dir() -> PathBuf {
    let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(".local").join("bin")
}

#[cfg(target_os = "windows")]
fn target_dir() -> PathBuf {
    let local_app_data = env::var("LOCALAPPDATA").unwrap_or_else(|_| {
        let home = env::var("USERPROFILE").unwrap_or_else(|_| ".".to_string());
        format!("{}\\AppData\\Local", home)
    });
    PathBuf::from(local_app_data).join("Programs")
}

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
fn target_dir() -> PathBuf {
    let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(".local").join("bin")
}

fn binary_name() -> &'static str {
    if cfg!(windows) { "couchpad.exe" } else { "couchpad" }
}
