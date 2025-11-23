use clap::Parser;

pub mod default;
pub mod self_;
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
    Default(default::Command),
    Show(show::Command),
    Update(update::Command),
    Self_(self_::Command),
}
