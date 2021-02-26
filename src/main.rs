use std::env;

use anyhow::{bail, Result};
use log::info;

use commands::{get_current_visibility, initialize_toggit, toggle};
mod commands;

use cli::{initialize, Toggit};
mod cli;

mod utils;

fn main() -> Result<()> {
    let cli = initialize();

    if cli.verbose {
        env::set_var("RUST_LOG", "info")
    }

    env_logger::init();

    let config_path = utils::get_global_config_path();

    info!("starting toggit..");

    match cli.cmd {
        Toggit::Init => initialize_toggit(&config_path)?,
        Toggit::Toggle { repo } => toggle(&config_path, &repo)?,
        Toggit::Visible { repo } => {
            if let Err(e) = get_current_visibility(&config_path, &repo) {
                bail!(e)
            }
        }
    }
    Ok(())
}
