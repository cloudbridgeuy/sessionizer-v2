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
}

pub async fn run(stream: UnixStream, app: App, global: crate::Global) -> Result<()> {
    match app.command {
        Command::List => list(stream, global).await,
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

    match response.event.as_str() {
        "server::list" => {
            for line in lines {
                println!("{}", line);
            }
        }
        "error" => println!("Received error from server"),
        _ => println!("Received unknown event from server"),
    };

    Ok(())
}
