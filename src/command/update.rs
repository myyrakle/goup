use serde::Deserialize;

use clap::Args;

#[derive(Clone, Debug, Default, Deserialize, Args)]
pub struct ConfigOptions {}

#[derive(Clone, Debug, Args)]
#[clap(name = "update", about = "Update Rust toolchains and rustup")]
pub struct Command {
    #[clap(flatten)]
    pub value: ConfigOptions,
}
