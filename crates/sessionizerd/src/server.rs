use events::Request;
use serde_json::json;
use std::str;
use tokio::io::AsyncWriteExt;
use tokio::net::UnixStream;

use crate::prelude::*;

pub async fn list(mut stream: UnixStream, request: Request<events::server::List>) -> Result<()> {
    log::info!("crate::server::list");
    let mut payload = request.payload.unwrap();

    if payload.user_id.is_none() {
        payload.user_id = Some(get_user_id()?);
    }

    let user_id = payload.user_id.unwrap();
    let folder_path = format!("/tmp/tmux-{}", user_id);

    if std::fs::metadata(&folder_path).is_err() {
        return Err(eyre!("No TMUX servers found"));
    };

    let mut lines: Vec<String> = vec![];

    for entry in std::fs::read_dir(folder_path)? {
        let entry = entry?;
        if let Some(file_name) = entry.file_name().to_str() {
            lines.push(file_name.to_string());
        }
    }

    let response = json!({
        "event": "server::list",
        "payload": Some(lines),
    });

    log::info!("{}", response);

    stream.write_all(response.to_string().as_bytes()).await?;
    Ok(())
}

fn get_user_id() -> Result<u32> {
    let output = std::process::Command::new("id").arg("-u").output()?;

    match output.status.success() {
        true => {
            let uid = str::from_utf8(&output.stdout)?;
            Ok(uid.trim().parse()?)
        }
        false => {
            log::error!(
                "Command failed with error: {}",
                str::from_utf8(&output.stderr).unwrap_or("Unknown error")
            );
            Err(eyre!("Failed to get user ID"))
        }
    }
}
