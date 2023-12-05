use clap::{self, Parser, ValueEnum};

#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
pub enum SetupType {
    Basic,
    Full,
    Customized,
}

#[derive(Parser)]
#[clap(author, version, about)]
pub struct CliArgs {

    #[clap(short, long, value_enum, default_value_t = SetupType::Basic)]
    pub setup_type: SetupType,


    #[clap(short, long, default_value = "config.json")]
    pub config_file: String,
}
