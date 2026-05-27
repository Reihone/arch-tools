use std::process::Command;
use colored::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PackageManager {
    Pacman,
    Yay,
    Paru,
}

impl std::fmt::Display for PackageManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PackageManager::Pacman => write!(f, "pacman"),
            PackageManager::Yay => write!(f, "yay"),
            PackageManager::Paru => write!(f, "paru"),
        }
    }
}

pub fn detect_package_manager() -> PackageManager {
    if is_installed("paru") {
        println!("{} Detected paru", "[+]".green());
        PackageManager::Paru
    } else if is_installed("yay") {
        println!("{} Detected yay", "[+]".green());
        PackageManager::Yay
    } else {
        println!("{} Using pacman (fallback)", "[+]".yellow());
        PackageManager::Pacman
    }
}

pub fn is_installed(package: &str) -> bool {
    Command::new("which")
        .arg(package)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

pub async fn install_package(pm: PackageManager, package: &str) -> Result<(), String> {
    println!("{} Installing {}...", "[*]".cyan(), package.bold());
    
    let cmd = match pm {
        PackageManager::Pacman => {
            Command::new("sudo")
                .args(&["pacman", "-S", "--noconfirm", package])
                .output()
        }
        PackageManager::Yay => {
            Command::new("yay")
                .args(&["-S", "--noconfirm", package])
                .output()
        }
        PackageManager::Paru => {
            Command::new("paru")
                .args(&["-S", "--noconfirm", package])
                .output()
        }
    };

    match cmd {
        Ok(output) if output.status.success() => {
            println!("{} {} installed successfully", "[✓]".green(), package.green());
            Ok(())
        }
        Ok(_) => Err(format!("Failed to install {}", package)),
        Err(e) => Err(format!("Command error: {}", e)),
    }
}

pub async fn install_multiple(
    pm: PackageManager,
    packages: &[&str],
) -> Result<(), String> {
    for package in packages {
        install_package(pm, package).await?;
    }
    Ok(())
}
