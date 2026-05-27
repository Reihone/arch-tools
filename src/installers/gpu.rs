use crate::package_manager::{detect_package_manager, install_multiple};
use colored::*;
use dialoguer::{theme::ColorfulTheme, Select};

pub async fn show_gpu_menu() -> Result<(), String> {
    let options = vec!["NVIDIA", "AMD (Mesa)", "Intel", "Auto-detect", "Back"];

    let theme = ColorfulTheme::default();
    let selection = Select::with_theme(&theme)
        .with_prompt("\nSelect your GPU:")
        .items(&options)
        .default(0)
        .interact_on_opt(&dialoguer::console::Term::stderr())
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Selection cancelled".to_string())?;

    let pm = detect_package_manager();

    match selection {
        0 => {
            println!("{}", "Installing NVIDIA drivers...".cyan());
            let packages = &["nvidia", "nvidia-utils"];
            install_multiple(pm, packages).await?;
        }
        1 => {
            println!("{}", "Installing Mesa (AMD/Intel open drivers)...".cyan());
            let packages = &["mesa", "lib32-mesa", "vulkan-radeon", "lib32-vulkan-radeon"];
            install_multiple(pm, packages).await?;
        }
        2 => {
            println!("{}", "Installing Intel drivers...".cyan());
            let packages = &["intel-media-driver", "libva-intel-driver"];
            install_multiple(pm, packages).await?;
        }
        3 => {
            println!("{}", "Auto-detecting GPU... (Not implemented yet)".yellow());
        }
        4 => return Ok(()),
        _ => return Err("Invalid selection".to_string()),
    }

    println!("{}", "\n[✓] GPU setup completed!".green());
    Ok(())
}
