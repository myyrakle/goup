mod command;

use clap::Parser;

fn main() {
    let _args = command::Command::parse();

    match _args.action {
        command::SubCommand::Show(cmd) => {
            println!("Show command executed with verbose: {}", cmd.value.verbose);
        }
    }
}
