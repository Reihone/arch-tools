use colored::*;
use dialoguer::{theme::ColorfulTheme, Confirm};
use crate::installers::{gpu, desktop, apps, browsers, package_managers};

pub async fn run_full_setup() -> Result<(), String> {
    let theme = ColorfulTheme::default();
    
    let confirm = Confirm::with_theme(&theme)
        .with_prompt("This will install all recommended packages. Continue?")
        .interact_on_opt(&dialoguer::console::Term::stderr())
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Selection cancelled".to_string())?;

    if !confirm {
        println!("{}", "Setup cancelled.".yellow());
        return Ok(());
    }

    println!("{}", "\nStarting full system setup...\n".cyan().bold());

    println!("{}", "\n[1/5] Setting up GPU drivers...".bold());
    gpu::show_gpu_menu().await?;

    println!("{}", "\n[2/5] Setting up Desktop Environment...".bold());
    desktop::show_desktop_menu().await?;

    println!("{}", "\n[3/5] Installing applications...".bold());
    apps::show_apps_menu().await?;

    println!("{}", "\n[4/5] Installing browsers...".bold());
    browsers::show_browsers_menu().await?;

    println!("{}", "\n[5/5] Setting up package managers...".bold());
    package_managers::show_pm_menu().await?;

    println!("{}", "\n".to_string() + &"=".repeat(60));
    println!("{}", "[✓] FULL SETUP COMPLETED!".green().bold());
    println!("{}", "=".repeat(60));
    
    Ok(())
}
