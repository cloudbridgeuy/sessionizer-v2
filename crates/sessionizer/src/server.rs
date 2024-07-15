use clap::{Parser, Subcommand};

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
    /// Attaches to a session on a TMUX server, creating both if necessary.
    #[clap(name = "attach")]
    Attach(AttachOptions),
    /// Detaches from the current TMUX server.
    #[clap(name = "detach")]
    Detach,
}

#[derive(Debug, Parser)]
pub struct AttachOptions {
    /// Name of the TMUX server.
    #[clap(default_value = "default")]
    server_name: String,
    /// Recreate the server if it already exists.
    #[clap(long)]
    recreate: bool,
    /// Session name defined as a filesystem directory.
    #[clap(short, long, default_value = get_cwd())]
    session: String,
}

/// Gets the current working directory as a String, or empty if an error occurs.
fn get_cwd() -> String {
    std::env::current_dir()
        .ok()
        .and_then(|p| p.to_str().map(|s| s.to_string()))
        .unwrap_or_default()
}

pub fn run(app: App, globals: crate::Globals) -> Result<()> {
    match app.command {
        Command::List => list(globals),
        Command::Detach => detach(),
        Command::Attach(options) => attach(options),
    }
}

#[derive(Debug, Parser)]
pub struct SwitchOptions {
    /// Name of the TMUX server.
    name: String,
}

fn detach() -> Result<()> {
    crate::tmux::server::detach()
}

fn list(globals: crate::Globals) -> Result<()> {
    let folder_path = format!("/tmp/tmux-{}", globals.user_id);
    if std::fs::metadata(&folder_path).is_err() {
        std::fs::create_dir(&folder_path)?;
    };

    for entry in std::fs::read_dir(folder_path)? {
        let entry = entry?;
        if let Some(file_name) = entry.file_name().to_str() {
            println!("{}", file_name);
        }
    }

    Ok(())
}

fn attach(options: AttachOptions) -> Result<()> {
    crate::tmux::server::create(&options.server_name, options.session, options.recreate)?;

    Ok(())
}
