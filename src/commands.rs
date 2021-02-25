use anyhow::{bail, Result};
use billboard::Billboard;
use log::info;
use reqwest::header::{AUTHORIZATION, USER_AGENT};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::{fs, str::FromStr};
use text_io::read;

macro_rules! ternary {
    ($c:expr, $v:expr, $v1:expr) => {
        if $c {
            $v
        } else {
            $v1
        }
    };
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct Config {
    token: String,
    owner: String,
}

impl FromStr for Config {
    type Err = toml::de::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        toml::from_str(s)
    }
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

pub fn initialize_togit(config_path: &PathBuf) -> Result<()> {
    let url = "https://github.com/settings/tokens";
    Billboard::default().display(format!("To find your github token, go to {}", url).as_str());
    let token = get_user_input("Enter API Token:\n");
    let owner = get_user_input("Enter Owner Name:\n");
    let config = Config { token, owner };
    config.to_file(&config_path)?;
    Ok(())
}

fn get_user_input(prompt: &str) -> String {
    eprint!("{}", prompt);
    let mut input: String = read!("{}\n");
    input.truncate(input.trim_end().len());
    input
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct Repo {
    private: bool,
}

pub fn toggle(config_path: &PathBuf, repos: String) -> Result<()> {
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
        repo = repos
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
    let info = ternary!(patch_response.private, "private", "public");
    println!("{} toggled to: {}", repos, info);
    Ok(())
}
