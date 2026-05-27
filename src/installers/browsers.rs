use crate::package_manager::{detect_package_manager, install_multiple};
use colored::*;
use dialoguer::{theme::ColorfulTheme, Checkboxes};

pub async fn show_browsers_menu() -> Result<(), String> {
    let options = vec!["Firefox", "Chromium", "Google Chrome", "Zen", "Brave"];

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
        match idx {
            0 => packages_to_install.push("firefox"),
            1 => packages_to_install.push("chromium"),
            2 => packages_to_install.push("google-chrome"),
            3 => packages_to_install.push("zen-browser"),
            4 => packages_to_install.push("brave-browser"),
            _ => {}
        }
    }

    println!(
        "{}",
        format!("Installing {} browsers...", packages_to_install.len()).cyan()
    );
    install_multiple(pm, &packages_to_install).await?;

    println!("{}", "\n[✓] Browsers installed successfully!".green());
    Ok(())
}
