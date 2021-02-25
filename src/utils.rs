use crate::commands::Config;
use reqwest::{
    blocking::RequestBuilder,
    header::{HeaderMap, HeaderValue, AUTHORIZATION, USER_AGENT},
};

use std::{
    env,
    path::{Path, PathBuf},
};

const TOGIT_CONFIG_FILE_NAME: &str = "default.toml";

pub enum MethodType {
    Get,
    Patch,
}

fn get_togit_home_dir() -> PathBuf {
    if let Ok(value) = env::var("TOGGIT_HOME") {
        log::info!("Using \'TOGGIT_HOME\' {}", value);
        Path::new(&value).to_path_buf()
    } else {
        log::info!("did not found \'TOGGIT_HOME\'. using default");
        dirs::home_dir()
            .expect("could not find home directory")
            .join(".togit")
    }
}

pub fn get_global_config_path() -> PathBuf {
    let home_dir = get_togit_home_dir();
    let config_path = home_dir.join("config").join(TOGIT_CONFIG_FILE_NAME);
    log::info!("Using global config file: {}", config_path.display());
    config_path
}

pub fn get_client(request_url: &str, config: Config) -> impl Fn(MethodType) -> RequestBuilder + '_ {
    move |method: MethodType| {
        let client = reqwest::blocking::Client::new();

        let mut headers = HeaderMap::new();

        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("token {token}", token = config.token)).unwrap(),
        );
        headers.insert(USER_AGENT, HeaderValue::from_str(&config.owner).unwrap());

        match method {
            MethodType::Get => client.get(request_url).headers(headers),
            MethodType::Patch => client.patch(request_url).headers(headers),
        }
    }
}
