use clap::Parser;
use std::io;
use tokio::io::AsyncWriteExt;
use tokio::net::{UnixListener, UnixStream};

use events::{Request, Response};

mod prelude;

use crate::prelude::*;

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

    let socket_file = args.socket_file.clone();

    log::info!("Binding to path: {}", &socket_file);

    // Remove existing socket file if it exists
    if std::fs::metadata(&socket_file).is_ok() {
        std::fs::remove_file(&socket_file).expect("Failed to remove existing socket file");
    }

    let listener = UnixListener::bind(&socket_file).expect("Failed to bind to path");
    let listener_delete_on_drop = UnixListenerDeleteOnDrop::new(socket_file, listener);

    loop {
        log::info!("Waiting for incoming connections...");
        match listener_delete_on_drop.listener.accept().await {
            Ok((stream, addr)) => {
                log::info!("New connection: {:?} - {:?}", stream, addr);
                tokio::spawn(handle_client(stream));
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
    let mut msg = vec![0; 1024];

    stream.readable().await?;

    loop {
        match stream.try_read(&mut msg) {
            Ok(n) => {
                msg.truncate(n);
                break;
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => continue,
            Err(e) => return Err(e.into()),
        }
    }

    let request = serde_json::from_slice::<Request>(&msg)?;
    log::info!("request: {}", request);

    match request.event.as_str() {
        "ping" => {
            let response = serde_json::to_string(&Response {
                event: "pong".to_string(),
            })?;
            stream.write_all(response.as_bytes()).await?;
        }
        _ => {
            let response = serde_json::to_string(&Request {
                event: "error".to_string(),
            })?;
            stream.write_all(response.as_bytes()).await?;
        }
    }

    Ok(())
}
