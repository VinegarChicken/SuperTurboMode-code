use std::{fs, vec};
use std::fs::File;
use std::io::Write;
use std::convert::From;

use skyline::error::show_error;

use semver::Version;

use serde::{Deserialize, Serialize};

const CONFIG_FILE_PATH: &str = "sd:/atmosphere/contents/01006A800016E000/romfs/SuperTurboMode.toml";

lazy_static::lazy_static! {
    pub static ref CONFIG: Box<Config> = Config::open().unwrap();
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    pub instant_info: InstantInfo,
    pub version_info: VersionInfo,
}
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct InstantInfo {
    //tests
    pub donkey_kong: bool,
    pub captain_falcon: bool,
    pub corrin: bool,
    pub dedede: bool,
    pub cloud: bool,
    pub ganon: bool,
    pub ike: bool,
    pub krool: bool,
    pub marth: bool,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct VersionInfo {
    pub version_info: String,
}


impl Config {
    pub fn new() -> Self {
        Config {
            instant_info: InstantInfo {
                donkey_kong: true,
                captain_falcon: true,
                corrin: true,
                dedede: true,
                cloud: true,
                ganon: true,
                ike: true,
                krool: true,
                marth: true
            },
            version_info: VersionInfo {
                version_info: env!("CARGO_PKG_VERSION").to_string(),
            },
            .. Config::default()
        }
    }

    pub fn open() -> Result<Box<Config>, String> {
        match fs::read_to_string(CONFIG_FILE_PATH) {
            // File exists
            Ok(content) => {
                // Try deserializing
                let mut config = match toml::from_str(&content) {
                    // Deserialized properly
                    Ok(conf) => {
                        Box::new(conf)
                    },
                    // Something happened when deserializing
                    Err(_) => {
                        println!("Super turbo mode Configuration file could not be deserialized");
                        show_error(1, "Configuration file could not be deserialized", &format!("Your configuration file ({}) is either poorly manually edited, outdated, corrupted", CONFIG_FILE_PATH));
                        println!("Super turbo mode Generating configuration file...");
                        Box::new(Config::new())
                    }
                };

                // Make sure the version matches with the current release
                if Version::parse(&config.version_info.version_info) < Version::parse(&env!("CARGO_PKG_VERSION").to_string()) {
                    println!("Super turbo mode: Configuration file version mismatch");
                    skyline_web::DialogOk::ok(format!("Updating configuration file to latest format"));

                    config.version_info.version_info = env!("CARGO_PKG_VERSION").to_string();
                    config.update();
                    config.save().unwrap();

                    Ok(config)
                } else {
                    config.update();
                    config.save().unwrap();
                    Ok(config)
                }
            }
            // File does not exist, generate it
            Err(_) => {
                skyline_web::DialogOk::ok(format!("Thank you for installing Super Turbo mode !\n\nConfiguration file will now be generated"));

                let config = Box::new(Config::new());
                config.save().unwrap();

                Ok(config)
            }
        }
    }

    fn update(&mut self) {
        self.version_info.version_info = env!("CARGO_PKG_VERSION").to_string();
    }
    pub fn save(&self) -> Result<(), std::io::Error> {
        let config_txt = toml::to_vec(&self).unwrap();

        let mut file = match File::create(CONFIG_FILE_PATH) {
            Ok(file) => file,
            Err(err) => return Err(err),
        };

        match file.write_all(&config_txt) {
            Ok(_) => {},
            Err(err) => return Err(err),
        }
        Ok(())
    }
}
pub fn main() -> Box<Config> {
    Config::open().unwrap()
}