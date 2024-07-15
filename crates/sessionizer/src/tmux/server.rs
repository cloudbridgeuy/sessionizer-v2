use crate::prelude::*;

pub fn create(
    server_name: impl Into<String> + Display,
    session: impl Into<String>,
    recreate: bool,
) -> Result<()> {
    let server_name: &str = &server_name.into();
    let session: &str = &session.into();

    log::debug!("Creating tmux session: {session} on server {server_name}");

    if let Ok(tmux_value) = std::env::var("TMUX") {
        return Err(eyre!(
            "Already inside a `tmux` server [TMUX]: {}",
            tmux_value
        ));
    }

    if recreate {
        kill(server_name)?;
    }

    if has_session(server_name, session).is_err() {
        new_session(server_name, session)?
    }

    attach(server_name, session)?;

    Ok(())
}

fn _switch_client(
    server_name: impl Into<String> + Display,
    session: impl Into<String>,
) -> Result<()> {
    let server_name: &str = &server_name.into();
    let session: &str = &session.into();

    log::debug!("Switching tmux client to a session on server({server_name}) named: {session}");

    let mut child = std::process::Command::new("tmux")
        .args(["-L", server_name, "attach", "-t", session])
        .spawn()
        .wrap_err(
            "Failed to switch the tmux client to a session on server({name}) named: {session}",
        )?;

    child
        .wait()
        .wrap_err("Failed to wait for the tmux client to switch to the session")?;

    Ok(())
}
fn attach(server_name: impl Into<String> + Display, session: impl Into<String>) -> Result<()> {
    let server_name: &str = &server_name.into();
    let session: &str = &session.into();

    log::debug!("Attaching to tmux session on server({server_name}) named: {session}");

    let mut child = std::process::Command::new("tmux")
        .args([
            "-L",
            server_name,
            "attach",
            "-t",
            format!("={}", session).as_str(),
        ])
        .spawn()
        .wrap_err("Failed to attach to tmux session on server({name}) named: {session}")?;

    child
        .wait()
        .wrap_err("Failed to wait for the tmux client to attach to the session")?;

    Ok(())
}

fn new_session(name: impl Into<String> + Display, session: impl Into<String>) -> Result<()> {
    let server_name: &str = &name.into();
    let session: &str = &session.into();

    log::debug!("Creating a new tmux session on server({server_name}) named: {session}");

    let mut child = std::process::Command::new("tmux")
        .args([
            "-L",
            server_name,
            "new-session",
            "-s",
            session,
            "-c",
            session,
            "-d",
        ])
        .spawn()
        .wrap_err("Failed to launch tmux `new-session` command")?;

    child
        .wait()
        .wrap_err("Failed to create a new tmux session on server({name}) named: {session}")?;

    Ok(())
}

fn has_session(server_name: impl Into<String> + Display, session: impl Into<String>) -> Result<()> {
    let server_name: &str = &server_name.into();
    let session: &str = &session.into();

    log::debug!("Checking if tmux server ({server_name}) has session: {session}");

    let mut child = std::process::Command::new("tmux")
        .args([
            "-L",
            server_name,
            "has-session",
            "-t",
            format!("={}", session).as_str(),
        ])
        .spawn()
        .wrap_err("Failed to launch tmux `has-session` command")?;

    match child
        .wait()
        .wrap_err("Failed to check if the tmux server ({name}) has session: {session}")
    {
        Ok(_) => Ok(()),
        Err(e) => {
            log::error!("tmux has-session -t ={}: {}", session, e);
            Err(e)
        }
    }
}

pub fn detach() -> Result<()> {
    if let Ok(tmux_value) = std::env::var("TMUX") {
        // Working inside a tmux session
        if let Some(last_segment) = tmux_value.rsplit('/').next() {
            if let Some(current_server) = last_segment.split(',').next() {
                let mut child = std::process::Command::new("tmux")
                    .args(["-L", current_server, "detach"])
                    .spawn()
                    .wrap_err("Failed to launch tmux `detach` command")?;

                child
                    .wait()
                    .wrap_err("Failed to detach from current tmux server")?;
            } else {
                return Err(eyre!("Failed to parse the TMUX environment variable"));
            }
        } else {
            return Err(eyre!("Failed to get current tmux server"));
        }
    }

    Ok(())
}

pub fn kill(server_name: impl Into<String> + Display) -> Result<()> {
    let server_name: &str = &server_name.into();

    let child = std::process::Command::new("tmux")
        .args(["-L", server_name, "kill-server"])
        .spawn()
        .wrap_err("Failed to launch tmux `kill-server` command")?;

    let output = child
        .wait_with_output()
        .wrap_err("Failed to kill tmux server")?;

    if !output.status.success() {
        let stderr = output.stderr;
        log::error!(
            "tmux kill-server -L {}: {}",
            server_name,
            std::str::from_utf8(&stderr).unwrap_or("Unknown error")
        );
        return Err(eyre!("Failed to kill tmux server"));
    }

    Ok(())
}
