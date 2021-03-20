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
    pub misc: Miscellaneous,
    pub version_info: VersionInfo,
    pub ganon_changes: GanondorfChanges,
}
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct InstantInfo {
    pub donkey_kong: bool,
    pub captain_falcon: bool,
    pub corrin: bool,
    pub dedede: bool,
    pub cloud: bool,
    pub ganon: bool,
    pub ike: bool,
    pub krool: bool,
    pub marth: bool,
    pub shulk: bool,
    pub roy: bool,
    pub bowser: bool,
    pub charizard: bool,
    pub drmario: bool,
    pub ivysaur: bool,
    pub wolf: bool,
    pub peach: bool,
    pub daisy: bool,
    pub incineroar: bool,
    pub byleth: bool,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct VersionInfo {
    pub version_num: String,
}
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GanondorfChanges {
    pub rng_ganon_u_smash: bool,
    pub rng_ganon_f_smash: bool,
    pub rng_ganon_u_smash_chances_upper: String,
    pub rng_ganon_f_smash_chances_upper: String,
}
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Miscellaneous {
    pub airdodge_cancels: bool,
    pub infinite_airdodges: bool,
    pub jab_cancels: bool,
    pub up_special_cancels: bool,
    pub aerial_smash_attacks: bool,
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
                marth: true,
                shulk: true,
                roy: true,
                bowser: true,
                charizard: true,
                drmario: true,
                ivysaur: true,
                wolf: true,
                peach: true,
                daisy: true,
                incineroar: true,
                byleth: true,
            },
            misc: Miscellaneous {
                airdodge_cancels: true,
                infinite_airdodges: true,
                jab_cancels: true,
                up_special_cancels: true,
                aerial_smash_attacks: true,
            },
            ganon_changes: GanondorfChanges{
                rng_ganon_u_smash: true,
                rng_ganon_f_smash: true,
                rng_ganon_u_smash_chances_upper: 5.to_string(),
                rng_ganon_f_smash_chances_upper: 5.to_string(),
            },
            version_info: VersionInfo {
                version_num: env!("CARGO_PKG_VERSION").to_string(),
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
                if Version::parse(&config.version_info.version_num) < Version::parse(&env!("CARGO_PKG_VERSION").to_string()) {
                    println!("Super turbo mode: Configuration file version mismatch");
                    skyline_web::DialogOk::ok(format!("Updating configuration file to latest format"));
                    config.version_info.version_num = env!("CARGO_PKG_VERSION").to_string();
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
        self.version_info.version_num = env!("CARGO_PKG_VERSION").to_string();
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