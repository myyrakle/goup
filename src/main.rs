mod command;

use clap::Parser;

fn main() {
    let _args = command::Command::parse();

    match _args.action {
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
