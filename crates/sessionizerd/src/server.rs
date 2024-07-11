use events::Request;
use serde_json::json;
use std::str;
use tokio::io::AsyncWriteExt;
use tokio::net::UnixStream;

use crate::prelude::*;

pub async fn create(
    mut stream: UnixStream,
    request: Request<events::server::Create>,
) -> Result<()> {
    let mut payload = request.payload.unwrap();

    if payload.user_id.is_none() {
        payload.user_id = Some(get_user_id()?);
    }

    let user_id = payload.user_id.unwrap();

    tmux_create_server(user_id, payload.name.as_ref(), payload.recreate)?;

    let response = json!({ "event": "server::create", });

    stream.write_all(response.to_string().as_bytes()).await?;
    Ok(())
}

pub async fn list(mut stream: UnixStream, request: Request<events::server::List>) -> Result<()> {
    let mut payload = request.payload.unwrap();

    if payload.user_id.is_none() {
        payload.user_id = Some(get_user_id()?);
    }

    let user_id = payload.user_id.unwrap();
    let folder_path = format!("/tmp/tmux-{}", user_id);
    if std::fs::metadata(&folder_path).is_err() {
        std::fs::create_dir(&folder_path)?;
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

fn tmux_kill_server(name: &str) -> Result<()> {
    let output = std::process::Command::new("tmux")
        .args(["kill-server", "-L", name])
        .output()?;

    match output.status.success() {
        true => Ok(()),
        false => {
            log::error!(
                "tmux kill-server -L {}: {}",
                name,
                str::from_utf8(&output.stderr).unwrap_or("Unknown error")
            );
            Ok(())
        }
    }
}

fn tmux_create_server(user_id: u32, name: &str, recreate: bool) -> Result<()> {
    if recreate {
        tmux_kill_server(name)?;
    }

    let folder_path = format!("/tmp/tmux-{}", user_id);
    if std::fs::metadata(&folder_path).is_err() {
        std::fs::create_dir(&folder_path)?;
    };

    // Look inside the folder to see if the file `payload.name` exists inside.
    let file_path = format!("{}/{}", folder_path, name);
    for entry in std::fs::read_dir(folder_path)? {
        let entry = entry?;
        if let Some(file_name) = entry.file_name().to_str() {
            if file_name == file_path {
                return Ok(());
            }
        }
    }

    let output = std::process::Command::new("touch")
        .args([file_path])
        .output()?;

    match output.status.success() {
        true => Ok(()),
        false => {
            log::error!(
                "tmux -L {}: {}",
                name,
                str::from_utf8(&output.stderr).unwrap_or("Unknown error")
            );
            Err(eyre!("Failed to create new tmux server"))
        }
    }
}
