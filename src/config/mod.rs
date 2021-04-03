use std::{fs, vec};
use std::fs::File;
use std::io::Write;
use std::convert::From;
use std::*;
use skyline::error::show_error;


use semver::Version;

use serde::{Deserialize, Serialize};
use std::path::Path;

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
    pub banjo_changes: BanjoChanges,
    pub fox_changes: FoxChanges,
    pub lucas_changes: LucasChanges,
    //pub lucario_changes: LucarioChanges,
    pub beyonetta_changes: BayonettaChanges,
    pub homerun_contest: HomerunContest,

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
    pub lucas: bool,
    pub ridley: bool,
    pub banjo: bool,
    pub littlemac: bool,
    pub fox: bool,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct VersionInfo {
    pub version_num: String,
}
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct HomerunContest {
    pub power_multiplier: String,
}
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GanondorfChanges {
    pub rng_ganon_u_smash: bool,
    pub rng_ganon_f_smash: bool,
    pub rng_ganon_u_smash_chances_upper: String,
    pub rng_ganon_f_smash_chances_upper: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LucasChanges {
    pub giant_special_n: bool,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BanjoChanges {
    pub fair_spike: bool,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct FoxChanges {
    pub better_down_air: bool,
    pub two_billion_percent_fair: bool,
    pub shine_actual_spike: bool,
}
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LucarioChanges {
    pub always_max_aura: bool,
    pub overpowered_giant_aura_sphere: bool,
}
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BayonettaChanges {
    pub super_long_witch_time: bool,
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
                lucas: true,
                ridley: true,
                banjo: true,
                littlemac: true,
                fox: true,
            },
            misc: Miscellaneous {
                airdodge_cancels: true,
                infinite_airdodges: true,
                jab_cancels: true,
                up_special_cancels: true,
                aerial_smash_attacks: true,
            },
            homerun_contest: HomerunContest{
                power_multiplier: "1.0".parse().unwrap(),
            },
            ganon_changes: GanondorfChanges{
                rng_ganon_u_smash: true,
                rng_ganon_f_smash: true,
                rng_ganon_u_smash_chances_upper: 5.to_string(),
                rng_ganon_f_smash_chances_upper: 5.to_string(),
            },
            banjo_changes: BanjoChanges{
                fair_spike: true,
            },
            fox_changes: FoxChanges{
                better_down_air: true,
                two_billion_percent_fair: true,
                shine_actual_spike: true,
            },
            lucas_changes: LucasChanges{
                giant_special_n: false,
            },
            /*
            lucario_changes: LucarioChanges{
                always_max_aura: false,
                overpowered_giant_aura_sphere: false,
            },
             */
            beyonetta_changes: BayonettaChanges{
                super_long_witch_time: false,
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
//I'll figure something else better later
pub fn param_configs() -> std::io::Result<()>{
    if CONFIG.lucas_changes.giant_special_n{
        let lucas_vl_param_path = Path::new("sd:/Ultimate/mods/SuperTurboMode-base/fighter/lucas/param/vl.prc");
        let disabled_lucas_vl_param_path = Path::new("sd:/Ultimate/mods/SuperTurboMode-base/fighter/lucas/param/.vl.prc");
        if Path::exists(Path::new("sd:/Ultimate/mods/SuperTurboMode-base/fighter/lucas/param/.vl.prc")){
            fs::rename(disabled_lucas_vl_param_path, lucas_vl_param_path);
        }
    }
    else{
        fs::rename(Path::new("sd:/Ultimate/mods/SuperTurboMode-base/fighter/lucas/param/vl.prc"), Path::new("sd:/Ultimate/mods/SuperTurboMode-base/fighter/lucas/param/.vl.prc"));
    }
    if CONFIG.beyonetta_changes.super_long_witch_time{
        let beyonetta_vl_param_path = Path::new("sd:/Ultimate/mods/SuperTurboMode-base/fighter/bayonetta/param/vl.prc");
        let beyonetta_lucas_vl_param_path = Path::new("sd:/Ultimate/mods/SuperTurboMode-base/fighter/bayonetta/param/.vl.prc");
        if Path::exists(Path::new("sd:/Ultimate/mods/SuperTurboMode-base/fighter/bayonetta/param/.vl.prc")){
            fs::rename(beyonetta_lucas_vl_param_path, beyonetta_vl_param_path);
        }
    }
    else{
        fs::rename(Path::new("sd:/Ultimate/mods/SuperTurboMode-base/fighter/bayonetta/param/vl.prc"), Path::new("sd:/Ultimate/mods/SuperTurboMode-base/fighter/bayonetta/param/.vl.prc"));
    }
/*
    if CONFIG.lucario_changes.always_max_aura{
        let lucario_always_aura_param_path = Path::new("sd:/Ultimate/mods/SuperTurboMode-base/fighter/lucario/param/AlwaysMaxAura.prc");
        let lucario_op_aura_ball_param_path = Path::new("sd:/Ultimate/mods/SuperTurboMode-base/fighter/lucario/param/GiantAuraSphere.prc");
        let lucario_vl_param = Path::new("sd:/Ultimate/mods/SuperTurboMode-base/fighter/lucario/param/vl.prc");
        if Path::exists(Path::new(lucario_vl_param)){
            fs::rename(lucario_vl_param, lucario_always_aura_param_path);
        }
        else{
            fs::rename(lucario_always_aura_param_path, lucario_vl_param);
        }
    }

    if CONFIG.lucario_changes.overpowered_giant_aura_sphere{
        let lucario_always_aura_param_path = Path::new("sd:/Ultimate/mods/SuperTurboMode-base/fighter/lucario/param/AlwaysMaxAura.prc");
        let lucario_op_aura_ball_param_path = Path::new("sd:/Ultimate/mods/SuperTurboMode-base/fighter/lucario/param/GiantAuraSphere.prc");
        let lucario_vl_param = Path::new("sd:/Ultimate/mods/SuperTurboMode-base/fighter/lucario/param/vl.prc");
        if Path::exists(Path::new(lucario_vl_param)){
            fs::rename(lucario_vl_param, lucario_op_aura_ball_param_path);
        }
        else{
            fs::rename(lucario_op_aura_ball_param_path, lucario_vl_param);
        }
    }
    else{
        fs::rename(Path::new("sd:/Ultimate/mods/SuperTurboMode-base/fighter/lucario/param/vl.prc"), Path::new("sd:/Ultimate/mods/SuperTurboMode-base/fighter/lucario/param/.vl.prc"));
    }
    if CONFIG.lucario_changes.overpowered_giant_aura_sphere && CONFIG.lucario_changes.always_max_aura{
        let lucario_always_aura_param_path = Path::new("sd:/Ultimate/mods/SuperTurboMode-base/fighter/lucario/param/AlwaysMaxAura.prc");
        let lucario_op_aura_ball_param_path = Path::new("sd:/Ultimate/mods/SuperTurboMode-base/fighter/lucario/param/GiantAuraSphere.prc");
        let lucario_vl_param = Path::new("sd:/Ultimate/mods/SuperTurboMode-base/fighter/lucario/param/vl.prc");
        if Path::exists(Path::new(lucario_vl_param)){
            fs::rename(lucario_vl_param, lucario_op_aura_ball_param_path);
        }
        else{
            fs::rename(lucario_op_aura_ball_param_path, lucario_vl_param);
        }
    }
    else{
        fs::rename(Path::new("sd:/Ultimate/mods/SuperTurboMode-base/fighter/lucario/param/vl.prc"), Path::new("sd:/Ultimate/mods/SuperTurboMode-base/fighter/lucario/param/.vl.prc"));
    }

     */
    Ok(())
}

pub fn main() -> Box<Config> {
    Config::open().unwrap()
}