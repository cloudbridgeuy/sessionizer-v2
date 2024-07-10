use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "xtasks")]
#[command(about = "Run project tasks using rust instead of scripts")]
pub struct App {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Run the mdxa-cli in dev mode
    #[command(disable_help_flag = true, disable_version_flag = true)]
    Mdka(MdkaArgs),
    /// Build the release binary
    Release(ReleaseArgs),
    /// Builds a binary and installs it at the given path
    Install(InstallArgs),
}

#[derive(Args, Debug)]
pub struct InstallArgs {
    /// Name of the binary to run.
    #[arg(short, long)]
    pub name: String,

    /// Path to install the binary to.
    #[arg(short, long)]
    pub path: String,
}

#[derive(Args, Debug)]
pub struct ReleaseArgs {
    /// Binary to build
    #[arg(short, long, default_value = "rargs")]
    pub binary: String,
    /// Don't build for Apple Silicon
    #[arg(long)]
    pub no_apple_silicon: bool,
    /// Don't build for Apple x86_64
    #[arg(long)]
    pub no_apple_x86_64: bool,
    /// Don't build for linux AAarch64
    #[arg(long)]
    pub no_linux_aarch64: bool,
}

#[derive(Args, Debug)]
pub struct MdkaArgs {
    #[arg(trailing_var_arg = true, allow_hyphen_values = true, hide = true)]
    pub _args: Vec<String>,
}
