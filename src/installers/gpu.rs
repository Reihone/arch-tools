use crate::package_manager::{detect_package_manager, install_multiple};
use crate::arch;
use colored::*;
use dialoguer::{theme::ColorfulTheme, Select};

pub async fn show_gpu_menu(architecture: arch::Architecture) -> Result<(), String> {
    if arch::is_arm64(architecture) {
        println!(
            "{} ARM64 detected - NVIDIA drivers not available",
            "[!]".yellow()
        );
        println!("{} Available: Mesa", "[i]".cyan());
        let options = vec!["AMD (Mesa)", "Intel", "Back"];

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
                println!("{}", "Installing Mesa (AMD/Intel open drivers)...".cyan());
                let packages = &["mesa", "vulkan-radeon"];
                install_multiple(pm, packages).await?;
            }
            1 => {
                println!("{}", "Installing Intel drivers...".cyan());
                let packages = &["intel-media-driver", "libva-intel-driver"];
                install_multiple(pm, packages).await?;
            }
            2 => return Ok(()),
            _ => return Err("Invalid selection".to_string()),
        }
    } else {
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
    }

    println!("{}", "\n[✓] GPU setup completed!".green());
    Ok(())
}
