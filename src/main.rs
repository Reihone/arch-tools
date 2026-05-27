mod dependency_checker;
mod menu;
mod package_manager;
mod installers;

use colored::*;
use std::io;

#[tokio::main]
async fn main() {
    println!("\n{}", "=".repeat(60).cyan());
    println!("{}", "  ARCH TOOLS - CachyOS Auto Configuration".cyan().bold());
    println!("{}", "=".repeat(60).cyan());
    println!();

    // Check and install dependencies
    println!("{}", "[*] Checking system dependencies...".yellow());
    if let Err(e) = dependency_checker::check_and_install_dependencies().await {
        eprintln!("{} {}", "[ERROR]".red().bold(), e);
        std::process::exit(1);
    }

    println!("{}", "[✓] All dependencies satisfied!".green());
    println!();

    // Show main menu
    loop {
        match menu::show_main_menu().await {
            Ok(true) => continue,
            Ok(false) => {
                println!("{}", "\n[*] Exiting... Thank you for using Arch Tools!".cyan());
                break;
            }
            Err(e) => {
                eprintln!("{} {}", "[ERROR]".red().bold(), e);
                println!("{}", "Press Enter to continue...".yellow());
                let _ = io::stdin().read_line(&mut String::new());
            }
        }
    }
}
