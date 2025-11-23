use clap::Args;
use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize, Args)]
pub struct ConfigOptions {
    #[clap(
        short,
        long,
        default_value = "false",
        help = "Enable verbose output with toolchain information"
    )]
    pub verbose: bool,
}

#[derive(Clone, Debug, Args)]
#[clap(name = "list", about = "List installed toolchains")]
pub struct Command {
    #[clap(flatten)]
    pub value: ConfigOptions,
}
