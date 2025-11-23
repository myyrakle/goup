use clap::Args;

pub mod uninstall;
pub mod update;

#[derive(Clone, Debug, Args)]
#[clap(name = "self", about = "Modify the goup installation")]
pub struct Command {
    #[clap(subcommand)]
    pub action: SubCommand,
}

#[derive(clap::Subcommand, Debug, Clone)]
pub enum SubCommand {
    Uninstall(uninstall::Command),
    Update(update::Command),
}
