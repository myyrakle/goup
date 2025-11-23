mod command;

use clap::Parser;

fn main() {
    let args = command::Command::parse();

    match args.action {
        command::SubCommand::Toolchain(cmd) => match cmd.action {
            command::toolchain::SubCommand::List(_list_cmd) => {
                println!("List command executed");
            }
            command::toolchain::SubCommand::Install(_install_cmd) => {
                println!("Install command executed");
            }
            command::toolchain::SubCommand::Uninstall(_uninstall_cmd) => {
                println!("Uninstall command executed");
            }
        },
        command::SubCommand::Default(_cmd) => {
            println!("Default command executed");
        }
        command::SubCommand::Show(cmd) => match cmd.action {
            Some(command::show::SubCommand::Profile(_profile_cmd)) => {
                println!("Profile command executed");
            }
            Some(command::show::SubCommand::ActiveToolchain(_active_toolchain_cmd)) => {
                println!("Show command executed with verbose: {}", cmd.value.verbose);
            }
            Some(command::show::SubCommand::Home(_home_cmd)) => {
                println!("Home command executed");
            }
            None => {
                println!("Show command executed with verbose: {}", cmd.value.verbose);
            }
        },
        command::SubCommand::Update(_cmd) => {
            println!("Update command executed");
        }
    }
}
