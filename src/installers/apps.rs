use crate::package_manager::{detect_package_manager, install_multiple};
use colored::*;
use dialoguer::{theme::ColorfulTheme, Checkboxes};

pub async fn show_apps_menu() -> Result<(), String> {
    let options = vec!["Steam", "Discord", "Telegram", "AyuGram", "VLC", "Blender"];

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
        match idx {
            0 => packages_to_install.push("steam"),
            1 => packages_to_install.push("discord"),
            2 => packages_to_install.push("telegram-desktop"),
            3 => packages_to_install.push("ayugram-desktop"),
            4 => packages_to_install.push("vlc"),
            5 => packages_to_install.push("blender"),
            _ => {}
        }
    }

    println!(
        "{}",
        format!("Installing {} applications...", packages_to_install.len()).cyan()
    );
    install_multiple(pm, &packages_to_install).await?;

    println!("{}", "\n[✓] Applications installed successfully!".green());
    Ok(())
}
