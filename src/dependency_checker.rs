use crate::package_manager::{detect_package_manager, install_package, is_installed};
use colored::*;

const REQUIRED_PACKAGES: &[&str] = &["git", "base-devel"];

pub async fn check_and_install_dependencies() -> Result<(), String> {
    let pm = detect_package_manager();
    
    for package in REQUIRED_PACKAGES {
        if !is_installed(package) {
            println!(
                "{} {} not found, installing...",
                "[!]".yellow(),
                package.bold()
            );
            install_package(pm, package).await?;
        } else {
            println!("{} {} is installed", "[✓]".green(), package.green());
        }
    }
    
    Ok(())
}
