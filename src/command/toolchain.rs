use clap::Args;

pub mod install;
pub mod list;
pub mod uninstall;

#[derive(Clone, Debug, Args)]
#[clap(name = "toolchain", about = "Install, uninstall, or list toolchains")]
pub struct Command {
    #[clap(subcommand)]
    pub action: SubCommand,
}

#[derive(clap::Subcommand, Debug, Clone)]
pub enum SubCommand {
    List(list::Command),
    Install(install::Command),
    Uninstall(uninstall::Command),
}
