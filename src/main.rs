use anyhow::Result;

use args::{get_cli, Togit};

use commands::{initialize_togit, toggle};
mod args;
mod commands;
mod utils;

fn main() -> Result<()> {
    env_logger::init();

    let cli = get_cli();

    let config_path = utils::get_global_config_path();
    let verbose = cli.verbose;

    if verbose {
        println!("starting toggit");
    }

    match cli.cmd {
        Togit::Init => initialize_togit(&config_path)?,
        Togit::Toggle { repo } => toggle(&config_path, &repo)?,
    }
    Ok(())
}
