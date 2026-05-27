use std::process::Command;
use colored::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Architecture {
    X86_64,
    ARM64,
    Unsupported,
}

impl std::fmt::Display for Architecture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Architecture::X86_64 => write!(f, "x86_64"),
            Architecture::ARM64 => write!(f, "aarch64"),
            Architecture::Unsupported => write!(f, "unsupported"),
        }
    }
}

pub fn detect_architecture() -> Architecture {
    let output = Command::new("uname")
        .arg("-m")
        .output();

    match output {
        Ok(out) => {
            let arch_str = String::from_utf8_lossy(&out.stdout).trim().to_string();
            let arch = match arch_str.as_str() {
                "x86_64" => Architecture::X86_64,
                "aarch64" => Architecture::ARM64,
                "arm64" => Architecture::ARM64,
                _ => Architecture::Unsupported,
            };

            println!(
                "{} Detected architecture: {}",
                "[+]".green(),
                format!("{}", arch).cyan().bold()
            );
            arch
        }
        Err(_) => {
            println!(
                "{} Could not detect architecture, assuming x86_64",
                "[!]".yellow()
            );
            Architecture::X86_64
        }
    }
}

pub fn get_arch_suffix(arch: Architecture) -> String {
    match arch {
        Architecture::X86_64 => String::new(),
        Architecture::ARM64 => "-aarch64".to_string(),
        Architecture::Unsupported => String::new(),
    }
}

pub fn is_arm64(arch: Architecture) -> bool {
    arch == Architecture::ARM64
}

pub fn is_x86_64(arch: Architecture) -> bool {
    arch == Architecture::X86_64
}
