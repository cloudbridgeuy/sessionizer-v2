use clap::Parser;
use clap_stdin::MaybeStdin;
use mdka::from_html;

mod prelude;

use crate::prelude::*;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// HTML input.
    #[clap(default_value = "-")]
    pub input: MaybeStdin<String>,
}

fn main() -> Result<()> {
    env_logger::init();
    color_eyre::install()?;

    let cli = Args::parse();

    let result = from_html(&cli.input);

    println!("{}", result);

    Ok(())
}
