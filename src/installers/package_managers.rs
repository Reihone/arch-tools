use crate::package_manager::{detect_package_manager, install_package, is_installed};
use colored::*;
use dialoguer::{theme::ColorfulTheme, Checkboxes};

pub async fn show_pm_menu() -> Result<(), String> {
    let options = vec!["yay", "paru", "aura", "flatpak", "snap"];

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
        let package = match idx {
            0 => "yay",
            1 => "paru",
            2 => "aura",
            3 => "flatpak",
            4 => "snapd",
            _ => continue,
        };

        if !is_installed(package) {
            install_package(pm, package).await?;
        } else {
            println!("{} {} is already installed", "[✓]".green(), package.green());
        }
    }

    println!("{}", "\n[✓] Package managers setup completed!".green());
    Ok(())
}
