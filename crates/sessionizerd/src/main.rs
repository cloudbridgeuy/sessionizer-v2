use clap::Parser;
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{UnixListener, UnixStream};

use events::{Request, Response};

mod prelude;
mod server;

use crate::prelude::*;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path where the Socket file will be created to store the daemon socket.
    #[arg(long, default_value = "/tmp/sessionizerd.sock")]
    socket_file: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Event {
    event: String,
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

    let socket_file = args.socket_file.clone();

    log::info!("Binding to path: {}", &socket_file);

    // Remove existing socket file if it exists
    if std::fs::metadata(&socket_file).is_ok() {
        std::fs::remove_file(&socket_file).expect("Failed to remove existing socket file");
    }

    let listener = UnixListener::bind(&socket_file).expect("Failed to bind to path");
    let listener_delete_on_drop = UnixListenerDeleteOnDrop::new(socket_file, listener);

    loop {
        match listener_delete_on_drop.listener.accept().await {
            Ok((stream, _)) => {
                tokio::spawn(async move {
                    if let Err(e) = handle_client(stream).await {
                        log::error!("Error handling client: {:?}", e);
                    }
                })
                .await?;
            }
            Err(e) => {
                log::error!("Error accepting connection: {:?}", e);
            }
        }
    }
}

struct UnixListenerDeleteOnDrop {
    path: String,
    listener: UnixListener,
}

impl UnixListenerDeleteOnDrop {
    fn new(path: String, listener: UnixListener) -> Self {
        Self { path, listener }
    }
}

impl Drop for UnixListenerDeleteOnDrop {
    fn drop(&mut self) {
        std::fs::remove_file(&self.path).unwrap();
    }
}

async fn handle_client(mut stream: UnixStream) -> Result<()> {
    let mut buf = vec![0; 1024];
    let n = stream.read(&mut buf).await?;
    let request = serde_json::from_slice::<Event>(&buf[..n])?;

    log::info!("event: {}", request.event);

    match request.event.as_str() {
        "ping" => pong(stream).await,
        "server::list" => {
            crate::server::list(
                stream,
                serde_json::from_slice::<Request<events::server::List>>(&buf[..n])?,
            )
            .await
        }
        "server::create" => {
            crate::server::create(
                stream,
                serde_json::from_slice::<Request<events::server::Create>>(&buf[..n])?,
            )
            .await
        }
        _ => unknown(stream).await,
    }?;

    Ok(())
}

async fn unknown(mut stream: UnixStream) -> Result<()> {
    let response = serde_json::to_string(&Request::<()> {
        event: "error".to_string(),
        payload: None,
    })?;
    stream.write_all(response.as_bytes()).await?;
    Ok(())
}

async fn pong(mut stream: UnixStream) -> Result<()> {
    let response = serde_json::to_string(&Response::<()> {
        event: "pong".to_string(),
        payload: None,
    })?;
    stream.write_all(response.as_bytes()).await?;
    Ok(())
}
