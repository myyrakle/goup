use clap::Args;
use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize, Args)]
pub struct ConfigOptions {
    #[clap(
        short,
        long,
        default_value = "false",
        help = "Automatic yes to prompts; assume 'yes' as answer to all prompts and run non-interactively"
    )]
    pub yes: bool,
}

#[derive(Clone, Debug, Args)]
#[clap(name = "uninstall", about = "Uninstall goup")]
pub struct Command {
    #[clap(flatten)]
    pub value: ConfigOptions,
}
