use std::env;

use anyhow::{bail, Result};
use args::{get_cli, Toggit};
use log::info;

use commands::{get_current_visibility, initialize_toggit, toggle};
mod args;
mod commands;
mod utils;

fn main() -> Result<()> {
    let cli = get_cli();

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
