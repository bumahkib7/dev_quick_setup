use std::error::Error;
use crate::config::Config;
use colored::*;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use dialoguer::{Confirm, Input, MultiSelect};
use dialoguer::theme::ColorfulTheme;
use indicatif::{ProgressBar, ProgressStyle};
use sys_info;
use crate::cli::SetupType;
use rayon::prelude::*;

pub struct SetupService<'a> {
    config: &'a mut Config,
}

impl<'a> SetupService<'a> {
    pub fn new(config: &'a mut Config) -> Self {
        SetupService { config }
    }


    pub fn run(&mut self, setup_type: SetupType) -> Result<(), Box<dyn Error>> {
        Self::detect_system()?;
        self.ensure_homebrew_installed()?;

        match setup_type {
            SetupType::Basic => self.run_basic_setup(),
            SetupType::Full => self.run_full_setup(),
            SetupType::Customized => self.run_customized_setup(),
        }
    }

    fn ensure_homebrew_installed(&self) -> Result<(), Box<dyn Error>> {
        match Command::new("brew").arg("--version").output() {
            Ok(output) => {
                if output.status.success() {
                    println!("Homebrew is already installed.");
                } else {
                    println!("Homebrew is not installed, attempting to install...");
                    self.install_homebrew()?;
                }
            },
            Err(_) => {
                println!("Error checking Homebrew, attempting to install...");
                self.install_homebrew()?;
            }
        }
        Ok(())
    }


    fn install_homebrew(&self) -> Result<(), Box<dyn Error>> {
        let install_script = "/bin/bash -c \"$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)\"";
        let status = Command::new(install_script)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()?;

        if !status.success() {
            return Err("Failed to install Homebrew.".into());
        }
        Ok(())
    }


    fn install_tool(&self, tool: &str) -> Result<(), Box<dyn Error>> {
        println!("Checking if {} is already installed...", tool);

        let check_installed = Command::new("brew")
            .arg("list")
            .arg(tool)
            .output()?;

        if check_installed.status.success() {
            println!("{} is already installed.", tool);
        } else {
            let pb = ProgressBar::new_spinner();


            let style = ProgressStyle::default_spinner();

            pb.set_style(style.template("{spinner:.green} Installing...").unwrap());


            let tool_owned = tool.to_owned();

            pb.set_message(tool_owned);

            pb.enable_steady_tick(Duration::from_millis(100));
            let output = Command::new("brew")
                .arg("install")
                .arg(tool)
                .output()?;

            pb.finish_and_clear();

            if output.status.success() {
                println!("{} {}", tool, "✓".green());
            } else {
                println!("{} {}", tool, "✗".red());
                eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
            }
        }
        Ok(())
    }

    fn install_tools(&self, tools: &[String], stage: &str) -> Result<(), Box<dyn Error>> {
        println!("\n{}:", stage.green());

        if Confirm::new()
            .with_prompt(format!("Do you want to customize the tools in this stage for {}?", stage))
            .interact()?
        {
            let selections = MultiSelect::new()
                .items(&tools)
                .with_prompt(format!("Select the tools to install for {}", stage))
                .defaults(&vec![true; tools.len()])
                .interact_opt()?;

            if let Some(indices) = selections {
                self.install_selected_tools(tools, &indices)?;
            } else {
                println!("Cancelled installation.");
            }
        } else {
            self.install_selected_tools(tools, &(0..tools.len()).collect::<Vec<usize>>())?;
        }

        Ok(())
    }

    fn install_selected_tools(&self, tools: &[String], selected_indices: &[usize]) -> Result<(), Box<dyn Error>> {
        let total = selected_indices.len();
        let pb = Arc::new(Mutex::new(ProgressBar::new(total as u64)));
        pb.lock().unwrap().set_style(ProgressStyle::default_bar()
            .template("{wide_bar} {pos}/{len} ({eta})")?
            .progress_chars("=> "));

        selected_indices.par_iter().for_each_with(pb.clone(), |pb, &i| {
            if let Err(err) = self.install_tool(&tools[i]) {
                eprintln!("Error installing {}: {}", tools[i], err);
            }
            pb.lock().unwrap().inc(1);
        });

        pb.lock().unwrap().finish_with_message("Installation complete");
        Ok(())
    }

    fn detect_system() -> Result<(), Box<dyn Error>> {
        let os_type = sys_info::os_type()?;
        let os_release = sys_info::os_release()?;

        println!("OS Type: {}, OS Release: {}", os_type, os_release);

        Ok(())
    }

    pub fn run_customized_setup(&mut self) -> Result<(), Box<dyn Error>> {
        let mut custom_tools = Vec::new();
        loop {
            let tool: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter a tool to install (leave empty to finish)")
                .interact_text()?;

            if tool.is_empty() {
                break;
            }

            custom_tools.push(tool);
        }

        self.config.update_customized(&custom_tools);
        self.config.save()?;

        for tool in custom_tools {
            self.install_tool(&tool)?;
        }

        Ok(())
    }


    fn check_tool_availability(&self, tool: &str) -> Result<bool, Box<dyn Error>> {
        let output = Command::new("brew")
            .arg("info")
            .arg(tool)
            .output()?;

        Ok(output.status.success())
    }
    fn run_basic_setup(&self) -> Result<(), Box<dyn Error>> {
        for tool in &self.config.basic {
            self.check_tool_availability(tool)?;
            self.install_tool(tool)?;
        }
        Ok(())
    }
    pub fn run_full_setup(&self) -> Result<(), Box<dyn Error>> {
        self.install_tools(&self.config.full.languages, "Languages")?;
        self.install_tools(&self.config.full.text_editors, "Text Editors")?;
        self.install_tools(&self.config.full.frameworks, "Frameworks")?;
        self.install_tools(&self.config.full.networking_tools, "Networking Tools")?;
        self.install_tools(&self.config.full.utilities, "Utilities")?;
        self.install_tools(&self.config.full.extras, "Extras")?;
        Ok(())
    }
}
