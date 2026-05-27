use colored::*;
use dialoguer::{theme::ColorfulTheme, Input};

pub async fn show_custom_menu() -> Result<(), String> {
    println!("{}", "\nCustom Package Installation".cyan().bold());
    println!("{}", "Enter package names separated by spaces:".yellow());
    println!("{}", "Example: firefox telegram vlc".italic());

    let theme = ColorfulTheme::default();
    let input: String = Input::with_theme(&theme)
        .with_prompt("Packages")
        .interact_text()
        .map_err(|e| e.to_string())?;

    if input.trim().is_empty() {
        println!("{}", "No packages specified.".yellow());
        return Ok(());
    }

    let packages: Vec<&str> = input.trim().split_whitespace().collect();
    println!(
        "{}",
        format!("Will install: {}", packages.join(", ")).cyan()
    );

    // TODO: Implement actual installation
    println!("{}", "Custom installation (not yet implemented)".yellow());

    Ok(())
}
