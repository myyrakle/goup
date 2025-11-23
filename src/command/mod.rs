use clap::Parser;

pub mod show;
pub mod toolchain;
pub mod update;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Command {
    #[clap(subcommand)]
    pub action: SubCommand,
}

#[derive(clap::Subcommand, Debug)]
pub enum SubCommand {
    Toolchain(toolchain::Command),
    Show(show::Command),
    Update(update::Command),
}
