// main.rs


mod config;
mod cli;
mod setup;
mod global_config;

use std::env;
use config::Config;
use setup::SetupService;
use cli::SetupType;
use global_config::CLIConfig;
use clap::{Command};
use std::error::Error;
use dialoguer::{Select, theme::ColorfulTheme};


use crossterm::{
    style::{Color, Print, ResetColor, SetForegroundColor},
    ExecutableCommand,
};
use std::io::{self, Write};
use std::path::PathBuf;

fn print_header(text: &str) -> io::Result<()> {
    io::stdout().execute(SetForegroundColor(Color::Cyan))?;
    io::stdout().execute(Print(text))?;
    io::stdout().execute(ResetColor)?;
    io::stdout().write_all(b"\n")?;
    io::stdout().flush()?;
    Ok(())
}



fn main() -> Result<(), Box<dyn Error>> {
    let matches = Command::new("DevQuickSetup")
        .version("1.0")
        .author("Bukhari kibuka")
        .about("Sets up development environments")
        .subcommand(Command::new("init")
            .about("Initializes the application and builds it"))
        .get_matches();

    if matches.subcommand_matches("init").is_some() {
        return initialize_application();
    }
    print_header("Welcome to My CLI Application")?;


    let setup_options = ["Basic", "Full", "Customized"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose your setup type")
        .default(0)
        .items(&setup_options)
        .interact()?;

    let setup_type = match selection {
        0 => SetupType::Basic,
        1 => SetupType::Full,
        2 => SetupType::Customized,
        _ => unreachable!(),
    };

    let mut config = Config::load()?;
    let mut setup_service = SetupService::new(&mut config);
    setup_service.run(setup_type)?;

    Ok(())
}

fn initialize_application() -> Result<(), Box<dyn Error>> {

    let _global_config = CLIConfig::load_or_init()?;


    let arch = std::env::consts::ARCH;
    println!("Detected CPU architecture: {}", arch);


    let target_path = match arch {
        "x86_64" => PathBuf::from("/usr/local/bin/devsetup"),
        "aarch64" => PathBuf::from("/usr/local/bin/devsetup"), // Example for M1 Macs
        _ => return Err("Unsupported architecture".into()),
    };


    let current_exe_path = env::current_exe()?;


    if target_path.exists() {
        println!("devsetup is already installed.");
        return Ok(());
    }

    println!("Attempting to create a symlink for devsetup.");
    println!("This operation might require elevated privileges.");


    match std::os::unix::fs::symlink(&current_exe_path, &target_path) {
        Ok(_) => println!("devsetup command installed successfully. You might need to restart your terminal."),
        Err(ref e) if e.kind() == io::ErrorKind::PermissionDenied => {
            println!("Permission denied. Please run the command with 'sudo' for necessary permissions.");
            return Err("Permission denied. Rerun with sudo.".into());
        }
        Err(e) => return Err(e.into()),
    }

    Ok(())
}

