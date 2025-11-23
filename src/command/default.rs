use serde::Deserialize;

use clap::Args;

#[derive(Clone, Debug, Default, Deserialize, Args)]
pub struct ConfigOptions {}

#[derive(Clone, Debug, Args)]
#[clap(
    name = "show",
    about = "Show the active and installed toolchains or profiles"
)]
pub struct Command {
    #[clap(flatten)]
    pub value: ConfigOptions,

    #[clap(
        name = "TOOLCHAIN",
        help = "Toolchain name, such as 'stable', 'nightly', '1.8.0', or a custom toolchain name. For more information see `rustup help
                  toolchain"
    )]
    pub toolchain: Option<String>,
}
