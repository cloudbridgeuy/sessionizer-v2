use clap::Parser;
use std::str;
use tokio::net::UnixStream;

mod ping;
mod prelude;
mod server;

use crate::prelude::*;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[clap(flatten)]
    global: Global,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, clap::Args)]
pub struct Global {
    /// AWS Region
    #[clap(
        long,
        env = "SESSIONIZER_SOCKET_FILE",
        global = true,
        default_value = "/tmp/sessionizerd.sock"
    )]
    socket_file: String,
    /// User ID. You can get it by running `id -u`.
    #[clap(long, env = "SESSIONIZER_USER_ID", global = true)]
    user_id: Option<u32>,
}

#[derive(Debug, Parser)]
pub enum Command {
    /// Handle TMUX Servers
    Server(crate::server::App),
    /// Ping the server
    Ping,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    color_eyre::install()?;

    let args = Args::parse();

    let socket_file = args.global.socket_file.to_string();
    let stream = UnixStream::connect(socket_file).await?;

    match args.command {
        Command::Ping => crate::ping::run(stream).await,
        Command::Server(app) => crate::server::run(stream, app, args.global).await,
    }
}
