// main.rs

mod config;
mod cli;
mod setup;
mod global_config;

use config::Config;
use setup::SetupService;
use cli::SetupType;
use global_config::CLIConfig;
use clap::{Command};
use std::error::Error;
use std::process::Command as SystemCommand;
use dialoguer::{Select, theme::ColorfulTheme};


use crossterm::{
    cursor::{Hide, MoveTo, Show},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    ExecutableCommand,
};
use std::io::{self, Write};

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

    let output = SystemCommand::new("cargo")
        .arg("build")
        .output()?;

    if !output.status.success() {
        eprintln!("Failed to build the project.");
        return Err("Build failed".into());
    }

    println!("Initialization and build complete.");
    Ok(())
}
