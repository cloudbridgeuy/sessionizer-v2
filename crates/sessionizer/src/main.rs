use clap::Parser;
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::UnixStream;

mod prelude;

use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
struct Request {
    event: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Response {
    event: String,
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path where the Socket file will be created to store the daemon socket.
    #[arg(long, default_value = "/tmp/sessionizerd.sock")]
    socket_file: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut builder = env_logger::builder();

    builder
        .filter(None, log::LevelFilter::Debug)
        .write_style(env_logger::WriteStyle::Never)
        .init();

    color_eyre::install()?;

    let args = Args::parse();

    let mut stream = UnixStream::connect(args.socket_file).await?;

    let request = Request {
        event: "ping".to_string(),
    };

    let request_json = serde_json::to_string(&request).unwrap();

    stream.write_all(request_json.as_bytes()).await?;

    let mut buf = vec![0; 1024];

    let n = stream.read(&mut buf).await?;

    let response: Response = serde_json::from_slice(&buf[..n]).unwrap();

    // Handle the response
    match response.event.as_str() {
        "pong" => println!("Received pong from server"),
        "error" => println!("Received error from server"),
        _ => println!("Received unknown event from server"),
    }

    Ok(())
}
