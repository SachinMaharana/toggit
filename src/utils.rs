use std::{
    env,
    path::{Path, PathBuf},
};

const TOGIT_CONFIG_FILE_NAME: &str = "default.toml";

fn get_togit_home_dir() -> PathBuf {
    match env::var("TOGGIT_HOME") {
        Ok(value) => {
            log::info!("Using \'TOGGIT_HOME\' {}", value);
            Path::new(&value).to_path_buf()
        }
        Err(_) => {
            log::info!("did not found \'TOGGIT_HOME\'. using default");
            dirs::home_dir()
                .expect("could not find home directory")
                .join(".togit")
        }
    }
}

pub fn get_global_config_path() -> PathBuf {
    let home_dir = get_togit_home_dir();
    let config_path = home_dir.join("config").join(TOGIT_CONFIG_FILE_NAME);
    log::info!("Using global config file: {}", config_path.display());
    config_path
}
