use crate::package_manager::{detect_package_manager, install_package, is_installed};
use crate::arch;
use colored::*;
use dialoguer::{theme::ColorfulTheme, Checkboxes};

pub async fn show_pm_menu(architecture: arch::Architecture) -> Result<(), String> {
    let mut options = vec!["paru", "aura", "flatpak", "snap"];
    let mut pm_packages = vec!["paru", "aura", "flatpak", "snapd"];

    if arch::is_x86_64(architecture) {
        options.insert(0, "yay");
        pm_packages.insert(0, "yay");
    } else {
        println!(
            "{} yay is not available for ARM64, use paru instead",
            "[!]".yellow()
        );
    }

    let theme = ColorfulTheme::default();
    let selections = Checkboxes::with_theme(&theme)
        .with_prompt("\nSelect package managers to install:")
        .items(&options)
        .interact_on_opt(&dialoguer::console::Term::stderr())
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Selection cancelled".to_string())?;

    if selections.is_empty() {
        println!("{}", "No package managers selected.".yellow());
        return Ok(());
    }

    let pm = detect_package_manager();

    for idx in selections {
        let package = pm_packages[idx];

        if !is_installed(package) {
            install_package(pm, package).await?;
        } else {
            println!(
                "{} {} is already installed",
                "[✓]".green(),
                package.green()
            );
        }
    }

    println!("{}", "\n[✓] Package managers setup completed!".green());
    Ok(())
}
