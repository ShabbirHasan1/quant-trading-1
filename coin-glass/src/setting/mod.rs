use lazy_static::lazy_static;
use serde::Deserialize;
use std::{fs::File, io::Read};

static SETTING_PATH: &str = "config.toml";

#[derive(Debug, Deserialize)]
pub struct CoinGlassKeys {
    pub api_key: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub coin_glass_keys: CoinGlassKeys,
}

impl Default for Settings {
    fn default() -> Self {
        let mut file = match File::open(SETTING_PATH) {
            Ok(f) => f,
            Err(e) => panic!("Couldn't open setting file, path: {}, {}", SETTING_PATH, e),
        };

        let mut file_content = String::new();

        match file.read_to_string(&mut file_content) {
            Ok(c) => c,
            Err(e) => panic!("Couldn't read setting"),
        };

        toml::from_str(&file_content).expect("Couldn't read setting")
    }
}

impl Settings {
    pub fn get<'a>() -> &'a Self {
        lazy_static! {
            static ref CACHE: Settings = Settings::default();
        }

        &CACHE
    }
}
