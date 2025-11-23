use clap::Args;
use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize, Args)]
pub struct ConfigOptions {}

#[derive(Clone, Debug, Args)]
#[clap(name = "update", about = "Download and install updates to goup")]
pub struct Command {
    #[clap(flatten)]
    pub value: ConfigOptions,
}
