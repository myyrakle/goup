use clap::Args;
use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize, Args)]
pub struct ConfigOptions {}

#[derive(Clone, Debug, Args)]
#[clap(name = "show", about = "Display the computed value of GOUP_HOME")]
pub struct Command {
    #[clap(flatten)]
    pub value: ConfigOptions,
}
