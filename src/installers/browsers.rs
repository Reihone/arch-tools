use crate::package_manager::{detect_package_manager, install_multiple};
use crate::arch;
use colored::*;
use dialoguer::{theme::ColorfulTheme, Checkboxes};

pub async fn show_browsers_menu(architecture: arch::Architecture) -> Result<(), String> {
    let mut options = vec!["Firefox", "Chromium", "Zen", "Brave"];
    let mut browser_packages = vec!["firefox", "chromium", "zen-browser", "brave-browser"];

    if arch::is_x86_64(architecture) {
        options.insert(2, "Google Chrome");
        browser_packages.insert(2, "google-chrome");
    } else {
        println!(
            "{} Google Chrome is not available for ARM64",
            "[!]".yellow()
        );
    }

    let theme = ColorfulTheme::default();
    let selections = Checkboxes::with_theme(&theme)
        .with_prompt("\nSelect browsers to install:")
        .items(&options)
        .interact_on_opt(&dialoguer::console::Term::stderr())
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Selection cancelled".to_string())?;

    if selections.is_empty() {
        println!("{}", "No browsers selected.".yellow());
        return Ok(());
    }

    let pm = detect_package_manager();
    let mut packages_to_install = Vec::new();

    for idx in selections {
        packages_to_install.push(browser_packages[idx]);
    }

    println!(
        "{}",
        format!("Installing {} browsers...", packages_to_install.len()).cyan()
    );
    install_multiple(pm, &packages_to_install).await?;

    println!("{}", "\n[✓] Browsers installed successfully!".green());
    Ok(())
}
