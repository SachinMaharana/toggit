use std::env;

use anyhow::Result;
use args::{get_cli, Toggit};
use log::info;

use commands::{initialize_toggit, toggle};
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
    }
    Ok(())
}
