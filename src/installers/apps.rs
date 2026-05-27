use crate::package_manager::{detect_package_manager, install_multiple};
use crate::arch;
use colored::*;
use dialoguer::{theme::ColorfulTheme, Checkboxes};

pub async fn show_apps_menu(architecture: arch::Architecture) -> Result<(), String> {
    let mut options = vec!["Discord", "Telegram", "AyuGram", "VLC", "Blender"];
    let mut app_packages = vec!["discord", "telegram-desktop", "ayugram-desktop", "vlc", "blender"];

    if arch::is_x86_64(architecture) {
        options.insert(0, "Steam");
        app_packages.insert(0, "steam");
    } else {
        println!(
            "{} Steam is not available for ARM64",
            "[!]".yellow()
        );
    }

    let theme = ColorfulTheme::default();
    let selections = Checkboxes::with_theme(&theme)
        .with_prompt("\nSelect applications to install:")
        .items(&options)
        .interact_on_opt(&dialoguer::console::Term::stderr())
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Selection cancelled".to_string())?;

    if selections.is_empty() {
        println!("{}", "No applications selected.".yellow());
        return Ok(());
    }

    let pm = detect_package_manager();
    let mut packages_to_install = Vec::new();

    for idx in selections {
        packages_to_install.push(app_packages[idx]);
    }

    println!(
        "{}",
        format!("Installing {} applications...", packages_to_install.len()).cyan()
    );
    install_multiple(pm, &packages_to_install).await?;

    println!("{}", "\n[✓] Applications installed successfully!".green());
    Ok(())
}
