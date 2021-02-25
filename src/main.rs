use anyhow::{bail, Result};
use billboard::Billboard;
use log::info;

use serde::{Deserialize, Serialize};
use std::fs;
use std::{
    env,
    path::{Path, PathBuf},
};
use structopt::StructOpt;
use text_io::read;

const TOGIT_CONFIG_FILE_NAME: &str = "default.toml";
#[derive(Debug, StructOpt)]
#[structopt(
    name = "togit",
    about = "toggle your github repository private or public"
)]
struct Cli {
    #[structopt(short, long)]
    debug: bool,

    #[structopt(subcommand)]
    cmd: Option<Togit>,
}

#[derive(Debug, StructOpt)]
#[structopt()]
enum Togit {
    Init,
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    token: String,
}

impl Config {
    fn to_file<T: AsRef<Path>>(&self, path: T) -> Result<()> {
        let toml = toml::to_string(self)?;
        fs::create_dir_all(path.as_ref().parent().unwrap())?;
        fs::write(path, toml)?;
        Ok(())
    }
}

fn main() -> Result<()> {
    env_logger::init();
    let cli = Cli::from_args();
    let config_path = get_global_config_path();

    if cli.cmd.is_some() {
        initialize_togit(&config_path)?;
    }

    if !config_path.exists() {
        bail!(
            "config path does not exist {}. try running `togit init`",
            config_path.display()
        )
    }

    Ok(())
}

fn initialize_togit(config_path: &PathBuf) -> Result<()> {
    let url = "https://github.com/settings/tokens";
    Billboard::default().display(format!("To find your github token, go to {}", url).as_str());
    let token = get_user_input("Enter API Token:\n");
    let config = Config { token };
    config.to_file(&config_path)?;
    Ok(())
}

fn get_togit_home_dir() -> PathBuf {
    match env::var("TOGIT_HOME") {
        Ok(value) => {
            info!("Using \'TOGIT_HOME\' {}", value);
            Path::new(&value).to_path_buf()
        }
        Err(_) => {
            info!("did not found \'TOGIT_HOME\'. using default");
            dirs::home_dir()
                .expect("could not find home directory")
                .join(".togit")
        }
    }
}

fn get_global_config_path() -> PathBuf {
    let home_dir = get_togit_home_dir();
    let config_path = home_dir.join("config").join(TOGIT_CONFIG_FILE_NAME);
    info!("Using global config file: {}", config_path.display());
    config_path
}

fn get_user_input(prompt: &str) -> String {
    eprint!("{}", prompt);
    let mut input: String = read!("{}\n");
    input.truncate(input.trim_end().len());
    input
}
