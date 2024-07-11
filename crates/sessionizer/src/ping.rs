use events::Response;
use serde_json::json;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::UnixStream;

use crate::prelude::*;

pub async fn run(mut stream: UnixStream) -> Result<()> {
    let request = json!({ "event": "ping", });

    stream.write_all(request.to_string().as_bytes()).await?;

    let mut buf = vec![0; 1024];

    let n = stream.read(&mut buf).await?;

    let _: Response<()> = serde_json::from_slice(&buf[..n]).unwrap();

    println!("pong");

    Ok(())
}
