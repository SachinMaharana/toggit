use anyhow::{bail, Result};
use billboard::Billboard;
use log::info;

use reqwest::header::{AUTHORIZATION, USER_AGENT};
use serde::{Deserialize, Serialize};
use std::{
    env,
    path::{Path, PathBuf},
};
use std::{fs, str::FromStr};
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

    #[structopt(required = true)]
    repo: String,

    #[structopt(subcommand)]
    cmd: Option<Togit>,
}

#[derive(Debug, StructOpt)]
#[structopt()]
enum Togit {
    Init,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct Config {
    token: String,
    owner: String,
}

impl Config {
    fn to_file<T: AsRef<Path>>(&self, path: T) -> Result<()> {
        let toml = toml::to_string(self)?;
        fs::create_dir_all(path.as_ref().parent().unwrap())?;
        fs::write(path, toml)?;
        Ok(())
    }

    fn get_config(&self, config_path: &PathBuf) -> Result<Config> {
        // Ok(Config::from_str(&fs::read_to_string(&config_path)?)?)
        fs::read_to_string(&config_path)
            .map_err(|e| e.into())
            .and_then(|contents| Config::from_str(&contents).map_err(|e| e.into()))
    }
}

impl FromStr for Config {
    type Err = toml::de::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        toml::from_str(s)
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct Repo {
    private: bool,
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

    let config = Config::default().get_config(&config_path)?;

    let request_url = format!(
        "https://api.github.com/repos/{owner}/{repo}",
        owner = config.owner,
        repo = cli.repo
    );

    let client = reqwest::blocking::Client::new();

    let response = {
        let response = client
            .get(&request_url)
            .header(
                AUTHORIZATION,
                format!("token {token}", token = config.token),
            )
            .header(USER_AGENT, &config.owner)
            .send()?;
        if response.status() != 200 {
            bail!("{}", response.text()?);
        }
        let response: Repo = response.json()?;
        response
    };

    info!(
        "Current: {}",
        if response.private {
            "private"
        } else {
            "public"
        }
    );

    let repo = Repo {
        private: !response.private,
    };

    let patch_response = {
        let patch_response = client
            .patch(&request_url)
            .header(
                AUTHORIZATION,
                format!("token {token}", token = config.token),
            )
            .header(USER_AGENT, config.owner)
            .json(&serde_json::json!(repo))
            .send()?;

        if patch_response.status() != 200 {
            bail!("Error: {}", patch_response.text()?);
        }
        let patch_response: Repo = patch_response.json()?;
        patch_response
    };

    info!(
        "Toggled to: {}",
        if patch_response.private {
            "private"
        } else {
            "public"
        }
    );
    Ok(())
}

fn initialize_togit(config_path: &PathBuf) -> Result<()> {
    let url = "https://github.com/settings/tokens";
    Billboard::default().display(format!("To find your github token, go to {}", url).as_str());
    let token = get_user_input("Enter API Token:\n");
    let owner = get_user_input("Enter Owner Name:\n");
    let config = Config { token, owner };
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

// curl \
//   -X PATCH \
//   -H "Accept: application/vnd.github.v3+json" \
//   https://api.github.com/repos/octocat/hello-world \
//   -d '{"name":"name"}'

//  let res = client
//         .get("https://api.github.com/repos/sachinmaharana/rkill")
//         .header(AUTHORIZATION, auth)
//         .json(&serde_json::json!({
//             "private": "true"}
//         ))
//         .send()?;

// let client = reqwest::blocking::Client::new();
//     let auth = format!("token {}", config.token);
//     let request_url = format!("https://api.github.com/repos/sachinmaharana/rkill");
//     println!("{}", request_url);
//     dbg!(&auth);
//     let res = client
//         .get(&request_url)
//         // .header(AUTHORIZATION, auth)
//         .send()?;
//     println!("{:?}", res);
