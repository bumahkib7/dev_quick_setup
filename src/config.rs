use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};
use dirs;

#[derive(Deserialize, Serialize)]
pub struct FullConfig {
    pub languages: Vec<String>,
    pub text_editors: Vec<String>,
    pub frameworks: Vec<String>,
    pub networking_tools: Vec<String>,
    pub utilities: Vec<String>,
    pub extras: Vec<String>,
}

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub basic: Vec<String>,
    pub full: FullConfig,
    pub customized: Vec<String>,
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let path = Config::get_config_path();

        if let Some(parent) = path.parent() {
            if parent.exists() {
                fs::remove_dir_all(parent)?;
            }
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }

        if !path.exists() {
            let default_config = Config::default(); // Define a default method or provide default values
            default_config.save()?;
            Ok(default_config)
        } else {
            let config_str = fs::read_to_string(&path)?;
            let config = serde_json::from_str(&config_str)?;
            Ok(config)
        }
    }


    fn get_config_path() -> PathBuf {
        dirs::home_dir()
            .expect("Could not find the home directory")
            .join(".DevQuickSetup")
            .join("config.json")
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_str = serde_json::to_string_pretty(self)?;
        let path = Config::get_config_path();
        fs::write(path.clone(), config_str)?;
        println!("Config saved to {:?}", path);
        Ok(())
    }

    pub fn update_customized(&mut self, tools: &[String]) {
        self.customized = tools.to_vec();
    }


    pub fn default() -> Self {
        Config {
            basic: vec![
                "git".into(),
                "curl".into(),
                "tmux".into(),
                "brew".into(), // Homebrew for package management
                "zsh".into(), // Zsh shell
                "wget".into(),
                "openssl".into(),
                "ssh".into(),
                "htop".into(),
                "bash".into(),
                "tree".into(),
                "grep".into(),
                "sed".into(),
                "jq".into(),
                "ncdu".into(),
                "rsync".into(),
                "findutils".into(),
                "coreutils".into(),
                "unzip".into(),
                "zip".into(),
            ],
            full: FullConfig {
                languages: vec![
                    "rust".into(),
                    "python".into(),
                    "java".into(),
                    "go".into(),
                    "ruby".into(),
                    "node".into(),
                    "php".into(),
                    "typescript".into(),
                    "scala".into(),
                    "perl".into(),
                    "lua".into(),
                    "kotlin".into(),
                    "haskell".into(),
                    "swift".into(),
                    "c".into(),
                    "c++".into(),
                    "c#".into(),
                    "dart".into(),
                    "elixir".into(),
                    "groovy".into(),
                ],
                text_editors: vec![
                    "vscode".into(),
                    "vim".into(),
                    "sublime-text".into(),
                    "emacs".into(),
                    "atom".into(),
                    "notepad-plus-plus".into(),
                    "gedit".into(),
                    "visual-studio".into(),
                    "textmate".into(),
                    "nano".into(),
                    "jupyter-notebook".into(),
                    "eclipse".into(),
                    "intellij-idea".into(),
                    "netbeans".into(),
                    "code::blocks".into(),
                    "brackets".into(),
                    "geany".into(),
                    "bluefish".into(),
                    "kate".into(),
                    "kdevelop".into(),
                ],
                frameworks: vec![
                    "actix-web".into(),
                    "express".into(),
                    "django".into(),
                    "ruby-on-rails".into(),
                    "spring-boot".into(),
                    "laravel".into(),
                    "flask".into(),
                    "sinatra".into(),
                    "struts".into(),
                    "nestjs".into(),
                    "koa".into(),
                    "ruby-grape".into(),
                    "gin".into(),
                    "symfony".into(),
                    "play-framework".into(),
                    "sails".into(),
                    "phoenix".into(),
                    "meteor".into(),
                    "django-rest-framework".into(),
                    "rocket".into(),
                ],
                networking_tools: vec![
                    "wireshark".into(),
                    "nmap".into(),
                    "postman".into(),
                    "curl".into(),
                    "wget".into(),
                    "netcat".into(),
                    "tcpdump".into(),
                    "ping".into(),
                    "traceroute".into(),
                    "sshuttle".into(),
                    "ngrok".into(),
                    "putty".into(),
                    "telnet".into(),
                    "FileZilla".into(),
                    "FileZilla-server".into(),
                    "ipconfig".into(),
                    "ifconfig".into(),
                    "dnsutils".into(),
                    "netstat".into(),
                    "ssmtp".into(),
                ],
                utilities: vec![
                    "htop".into(),
                    "jq".into(),
                    "tree".into(),
                    "watch".into(),
                    "tmux".into(),
                    "screen".into(),
                    "cron".into(),
                    "ffmpeg".into(),
                    "imagemagick".into(),
                    "ncdu".into(),
                    "gparted".into(),
                    "iotop".into(),
                    "lsof".into(),
                    "strace".into(),
                    "tldr".into(),
                    "fd".into(),
                    "exa".into(),
                    "fzf".into(),
                    "ripgrep".into(),
                    "ag".into(),
                ],
                extras: vec![
                    "spotify".into(),
                    "slack".into(),
                    "docker".into(),
                    "telegram".into(),
                    "whatsapp".into(),
                    "skype".into(),
                    "discord".into(),
                    "zoom".into(),
                    "microsoft-teams".into(),
                    "virtualbox".into(),
                    "vagrant".into(),
                    "gimp".into(),
                    "inkscape".into(),
                    "blender".into(),
                    "steam".into(),
                    "unity".into(),
                    "android-studio".into(),
                    "xcode".into(),
                    "visual-studio-code-insiders".into(),
                    "teamviewer".into(),
                    "obs".into(),
                ],
            },
            customized: vec![
            ],
        }
    }

}
