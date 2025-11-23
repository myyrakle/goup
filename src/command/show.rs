use serde::Deserialize;

use clap::Args;

pub mod active_toolchain;
pub mod home;
pub mod profile;

#[derive(Clone, Debug, Default, Deserialize, Args)]
pub struct ConfigOptions {
    #[clap(
        short,
        long,
        default_value = "false",
        help = "Enable verbose output with rustc information for all installed toolchains"
    )]
    pub verbose: bool,
}

#[derive(Clone, Debug, Args)]
#[clap(
    name = "show",
    about = "Show the active and installed toolchains or profiles"
)]
pub struct Command {
    #[clap(flatten)]
    pub value: ConfigOptions,

    #[clap(subcommand)]
    pub action: Option<SubCommand>,
}

#[derive(clap::Subcommand, Debug, Clone)]
pub enum SubCommand {
    Home(home::Command),
    ActiveToolchain(active_toolchain::Command),
    Profile(profile::Command),
}
