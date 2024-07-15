use clap::Parser;
use std::str;

mod prelude;
mod server;
mod tmux;

use crate::prelude::*;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[clap(flatten)]
    globals: Globals,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, clap::Args)]
pub struct Globals {
    /// User ID. You can get it by running `id -u`.
    #[clap(long, env = "SESSIONIZER_USER_ID", global = true, default_value=get_user_id().to_string())]
    user_id: u32,
}

#[derive(Debug, Parser)]
pub enum Command {
    /// Handle TMUX Servers
    Server(crate::server::App),
}

fn main() -> Result<()> {
    env_logger::init();
    color_eyre::install()?;

    let args = Args::parse();

    match args.command {
        Command::Server(app) => crate::server::run(app, args.globals),
    }
}

fn get_user_id() -> u32 {
    let output = match std::process::Command::new("id").arg("-u").output() {
        Ok(output) => output,
        Err(_) => {
            log::error!("Failed to get user ID");
            return 0o0;
        }
    };

    match output.status.success() {
        true => {
            let uid = match str::from_utf8(&output.stdout) {
                Ok(uid) => uid,
                Err(_) => {
                    log::error!("Failed to parse user ID as utf8");
                    return 0o0;
                }
            };
            match uid.trim().parse() {
                Ok(uid) => uid,
                Err(_) => {
                    log::error!("Failed to parse user ID to u32");
                    0o0
                }
            }
        }
        false => {
            log::error!(
                "Command failed with error: {}",
                str::from_utf8(&output.stderr).unwrap_or("Unknown error")
            );
            0o0
        }
    }
}
