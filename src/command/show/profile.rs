use clap::Args;
use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize, Args)]
pub struct ConfigOptions {}

#[derive(Clone, Debug, Args)]
#[clap(
    name = "profile",
    about = "Show the default profile used for the `goup install` command"
)]
pub struct Command {
    #[clap(flatten)]
    pub value: ConfigOptions,
}
