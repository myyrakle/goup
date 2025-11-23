use clap::Args;
use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize, Args)]
pub struct ConfigOptions {}

#[derive(Clone, Debug, Args)]
#[clap(
    name = "install",
    about = "Install or update the given toolchains, or by default the active toolchain"
)]
pub struct Command {
    #[clap(flatten)]
    pub value: ConfigOptions,

    #[clap(
        name = "TOOLCHAIN",
        help = "Toolchain name, such as 'stable', 'nightly', '1.8.0', or a custom toolchain name. For more information see `rustup help
                  toolchain"
    )]
    pub toolchains: Vec<String>,
}
