use clap::builder::Command;
use clap::arg;

pub fn server() -> Command {
    Command::new("server").args([
            arg!(-H --host <HOST> "the server host"),
            arg!(-p --port <PORT> "the server port"),
        ]
    )
}
pub fn config() -> Command {
    Command::new("config")
        .subcommand(
            Command::new("user")
                .subcommand(Command::new("add").arg(
                    arg!(<username> "Name of the user")
                ))
        )
}

pub fn cli() -> Command {
    Command::new("github_submodule_hook")
        .subcommand(server())
        .subcommand(config())
}

/*use clap::Parser;


pub enum Action {

}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Name of the person to greet
   #[arg(short, long)]
   name: String,

   /// Number of times to greet
   #[arg(short, long, default_value_t = 1)]
   count: u8,
}*/
