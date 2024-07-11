use clap::{Parser, Subcommand};
use events::Response;
use serde_json::json;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::UnixStream;

use crate::prelude::*;

#[derive(Debug, Parser)]
#[command(name = "server", about = "Handle TMUX Servers")]
pub struct App {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Get a list of all the running TMUX servers.
    #[clap(name = "list")]
    List,
    /// Create a new TMUX server.
    #[clap(name = "create")]
    Create(CreateOptions),
}

#[derive(Debug, Parser)]
pub struct CreateOptions {
    /// Name of the TMUX server.
    name: String,
    /// Recreate the server if it already exists.
    #[clap(long)]
    recreate: bool,
}

pub async fn run(stream: UnixStream, app: App, global: crate::Global) -> Result<()> {
    match app.command {
        Command::List => list(stream, global).await,
        Command::Create(options) => create(stream, global, options).await,
    }
}

async fn list(mut stream: UnixStream, global: crate::Global) -> Result<()> {
    let request = json!({
        "event": "server::list",
        "payload": {"user_id": global.user_id},
    });

    stream.write_all(request.to_string().as_bytes()).await?;

    let mut buf = vec![0; 1024];
    let n = stream.read(&mut buf).await?;
    let response: Response<events::misc::Lines> = serde_json::from_slice(&buf[..n])?;

    let lines = response.payload.unwrap();

    for line in lines {
        println!("{}", line);
    }

    Ok(())
}

async fn create(
    mut stream: UnixStream,
    global: crate::Global,
    options: CreateOptions,
) -> Result<()> {
    let request = json!({
        "event": "server::create",
        "payload": {
            "user_id": global.user_id,
            "name": options.name,
            "recreate": options.recreate,
        },
    });

    stream.write_all(request.to_string().as_bytes()).await?;

    let mut buf = vec![0; 1024];
    let n = stream.read(&mut buf).await?;
    let _: Response<events::misc::Lines> = serde_json::from_slice(&buf[..n])?;

    Ok(())
}
