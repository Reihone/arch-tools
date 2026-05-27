use crate::installers;
use colored::*;
use dialoguer::{theme::ColorfulTheme, Select};

pub async fn show_main_menu() -> Result<bool, String> {
    let options = vec![
        "GPU Drivers (NVIDIA/Mesa/Intel)",
        "Desktop Environment (KDE/GNOME/Hyprland/i3)",
        "Applications (Steam, Discord, Telegram)",
        "Browsers (Firefox, Chrome, Zen)",
        "Package Managers (yay, paru, flatpak, snap)",
        "Full Auto Setup (Install everything)",
        "Custom Selection",
        "Exit",
    ];

    let theme = ColorfulTheme::default();
    let selection = Select::with_theme(&theme)
        .with_prompt("\nWhat would you like to do?")
        .items(&options)
        .default(0)
        .interact_on_opt(&dialoguer::console::Term::stderr())
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Selection cancelled".to_string())?;

    match selection {
        0 => {
            installers::gpu::show_gpu_menu().await?;
            Ok(true)
        }
        1 => {
            installers::desktop::show_desktop_menu().await?;
            Ok(true)
        }
        2 => {
            installers::apps::show_apps_menu().await?;
            Ok(true)
        }
        3 => {
            installers::browsers::show_browsers_menu().await?;
            Ok(true)
        }
        4 => {
            installers::package_managers::show_pm_menu().await?;
            Ok(true)
        }
        5 => {
            installers::full_setup::run_full_setup().await?;
            Ok(true)
        }
        6 => {
            installers::custom::show_custom_menu().await?;
            Ok(true)
        }
        7 => Ok(false),
        _ => Err("Invalid selection".to_string()),
    }
}
