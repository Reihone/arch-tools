use crate::package_manager::{detect_package_manager, install_multiple};
use crate::arch;
use colored::*;
use dialoguer::{theme::ColorfulTheme, Select};

pub async fn show_desktop_menu(architecture: arch::Architecture) -> Result<(), String> {
    let options = vec!["KDE Plasma", "GNOME", "Hyprland", "i3", "Back"];

    let theme = ColorfulTheme::default();
    let selection = Select::with_theme(&theme)
        .with_prompt("\nSelect Desktop Environment:")
        .items(&options)
        .default(0)
        .interact_on_opt(&dialoguer::console::Term::stderr())
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Selection cancelled".to_string())?;

    let pm = detect_package_manager();

    match selection {
        0 => {
            println!("{}", "Installing KDE Plasma...".cyan());
            let packages = &["plasma-desktop", "plasma-applications", "sddm"];
            install_multiple(pm, packages).await?;
        }
        1 => {
            println!("{}", "Installing GNOME...".cyan());
            let packages = &["gnome", "gnome-shell", "gdm"];
            install_multiple(pm, packages).await?;
        }
        2 => {
            println!("{}", "Installing Hyprland...".cyan());
            let packages = &["hyprland", "hyprpaper", "hyprcursor"];
            install_multiple(pm, packages).await?;
        }
        3 => {
            println!("{}", "Installing i3...".cyan());
            let packages = &["i3-wm", "i3status", "dmenu"];
            install_multiple(pm, packages).await?;
        }
        4 => return Ok(()),
        _ => return Err("Invalid selection".to_string()),
    }

    println!("{}", "\n[✓] Desktop Environment setup completed!".green());
    Ok(())
}
