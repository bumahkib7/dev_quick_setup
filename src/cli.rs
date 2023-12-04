// cli.rs

use clap::{self, Parser, ValueEnum};

// The enum representing the setup type
#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
pub enum SetupType {
    Basic,
    Full,
    Customized,
}

// Deriving the `Parser` trait to handle command-line arguments
#[derive(Parser)]
#[clap(author, version, about)]
pub struct CliArgs {

    #[clap(short, long, value_enum, default_value_t = SetupType::Basic)]
    pub setup_type: SetupType,


    #[clap(short, long, default_value = "config.json")]
    pub config_file: String,
}

