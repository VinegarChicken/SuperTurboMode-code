#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#![allow(unused_imports)]


use skyline::*;
use std::path::Path;
use std::fs;
use smash::lib::lua_const::*;
use crate::config::CONFIG;
use smash::app::{sv_battle_object, utility, BattleObjectModuleAccessor};
use smash::app::lua_bind::WorkModule;
use skyline::hooks::{getRegionAddress, Region};
use smash::resource::find_subsequence;
use smash::hash40;

static INT_SEARCH_CODE: &[u8] = &[
    0x00, 0x1c, 0x40, 0xf9, 0x08, 0x00, 0x40, 0xf9, 0x03, 0x11, 0x40, 0xf9,
];

static FLOAT_SEARCH_CODE: &[u8] = &[
    0x00, 0x1c, 0x40, 0xf9, 0x08, 0x00, 0x40, 0xf9, 0x03, 0x19, 0x40, 0xf9,
];

static mut INT_OFFSET : usize = 0x4E19D0;
static mut FLOAT_OFFSET : usize = 0x4E19D0;

mod mario;
mod custom;
mod koopa;
mod fox;
mod ganon;
mod captain;
mod ivysaur;
mod simonandrichter;
mod donkey;
mod dedede;
mod cloud;
mod squirtle;
mod marth;
mod krool;
mod ike;
mod shulk;
mod zss;
mod drmario;
mod ness;
mod roy;
mod corrin;
mod config;
mod charizard;
mod wolf;
mod daisy;
mod peach;
mod byleth;
mod incineroar;
mod lucas;
mod ridley;
mod banjo;
mod mac;
mod ken;
mod terry;
mod darkpit;
mod pit;
mod link;
mod younglink;
mod hero;
mod joker;
mod wario;
mod luigi;
mod mewtwo;
mod snake;
mod palutena;
mod bowserjr;
mod diddykong;
mod falco;
mod samusd;
mod samus;
mod lucario;
mod kirby;
mod zelda;
mod greninja;
mod rob;
mod metaknight;
mod pyra;
mod sephiroth;

//mod sephiroth;

fn is_jump_param(param_type: u64, param_hash: u64) -> bool{
    if param_hash == 0 {
        if param_type == hash40("jump_count_max"){
            return true;
        }
    }
    return false;
}
#[skyline::hook(replace = smash::app::lua_bind::ArticleModule::is_generatable)]
pub unsafe fn is_generatable_hook(module_accessor: &mut smash::app::BattleObjectModuleAccessor, article: i32) -> bool {
    return true;
}
#[skyline::hook(offset = INT_OFFSET)]
pub unsafe fn get_param_int_replace(boma: u64, param_type: u64, param_hash: u64) -> i32 {
    let module_accessor = &mut *(*((boma as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
    let ret = original!()(boma, param_type, param_hash);
    let fighter_kind = utility::get_kind(module_accessor);
    if utility::get_category(module_accessor) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        if param_type == hash40("jump_count_max"){
            return ret + 5;
        }
        if fighter_kind == *FIGHTER_KIND_BRAVE{
            if param_hash == smash::hash40("hold_frame_m") {
                return 0;
            }
            if param_hash == smash::hash40("hold_frame_l") {
                return 1;
            }
            else {
                return ret;
            }
        }
        if fighter_kind == *FIGHTER_KIND_CHROM{
            if param_hash == smash::hash40("charge_speed_mul") {
                return 100;
            }
            else {
                return ret;
            }
        }
        if fighter_kind == *FIGHTER_KIND_ELIGHT{
            if param_hash == smash::hash40("charge_frame") {
                return 1;
            }
            else {
                return ret;
            }
        }
        if fighter_kind == *FIGHTER_KIND_DONKEY{
            if param_hash == smash::hash40("max_charge_frame") {
                return 1;
            }
            else {
                return ret;
            }
        }
        if fighter_kind == *FIGHTER_KIND_JACK{
            if param_hash == smash::hash40("doyle_frame") {
                return 999999999;
            }
            else {
                return ret;
            }
        }
        if fighter_kind == *FIGHTER_KIND_CLOUD{
            if param_hash == smash::hash40("limit_break_clear_frame") {
                return 86400;
            }
            else {
                return ret;
            }
        }
        if fighter_kind == *FIGHTER_KIND_SHEIK{
            if param_hash == smash::hash40("needle_num_max") {
                return 100;
            }
            else {
                return ret;
            }
        }
        if fighter_kind == *FIGHTER_KIND_KAMUI{
            if param_hash == smash::hash40("max_hold_frame") {
                return 1;
            }
            else {
                return ret;
            }
        }
        if fighter_kind == *FIGHTER_KIND_LITTLEMAC{
            if param_hash == smash::hash40("special_n_max_charge_frame") {
                return 1;
            }
            else {
                return ret;
            }
        }
        if fighter_kind == *FIGHTER_KIND_LUCARIO{
            if param_hash == smash::hash40("max_charge_frame") {
                return 1;
            }
            else {
                return ret;
            }
        }
        if param_type == smash::hash40("common") {
            if param_hash == smash::hash40("invalid_capture_frame") {
                return 1
            }
            else{
                return ret;
            }
        }
/*
        if is_jump_param(param_type, param_hash){
            return ret + 5;
        }

 */


        else {
            return ret;
        }
    }
    else {
        return ret;
    }

}

#[skyline::hook(offset = FLOAT_OFFSET)]
pub unsafe fn get_param_float_replace(module_accessor: u64, param_type: u64, param_hash: u64) -> f32 {
    let boma = &mut *(*((module_accessor as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
    let ret = original!()(module_accessor, param_type, param_hash);
    let fighter_kind = utility::get_kind(boma);
    if utility::get_category(boma) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        if fighter_kind == *FIGHTER_KIND_CLOUD{
            if param_hash == smash::hash40("limit_gauge_add") {
                return 86400.0;
            }
            else {
                return ret;
            }
        }
        if fighter_kind == *FIGHTER_KIND_CLOUD{
            if param_hash == smash::hash40("limit_gauge_damage_dec") {
                return 86400.0;
            }
            else {
                return ret;
            }
        }
        if fighter_kind == *FIGHTER_KIND_SAMUS{
            if param_hash == smash::hash40("cshot_charge_frame") {
                return 0.1;
            }
            else {
                return ret;
            }
        }
        if fighter_kind == *FIGHTER_KIND_SAMUSD{
            if param_hash == smash::hash40("cshot_charge_frame") {
                return 0.1;
            }
            else {
                return ret;
            }
        }
        if fighter_kind == *FIGHTER_KIND_IKE{
            if param_hash == smash::hash40("charge_speed_mul") {
                return 100.0;
            }
            else {
                return ret;
            }
        }
        if fighter_kind == *FIGHTER_KIND_ROY{
            if param_hash == smash::hash40("charge_speed_mul") {
                return 100.0;
            }
            else {
                return ret;
            }
        }
        if fighter_kind == *FIGHTER_KIND_CHROM{
            if param_hash == smash::hash40("charge_speed_mul") {
                return 100.0;
            }
            else {
                return ret;
            }
        }
        if fighter_kind == *FIGHTER_KIND_MARTH{
            if param_hash == smash::hash40("charge_frame_max") {
                return 0.001;
            }
            else {
                return ret;
            }
        }
        if fighter_kind == *FIGHTER_KIND_LUCINA{
            if param_hash == smash::hash40("charge_frame_max") {
                return 0.001;
            }
            else {
                return ret;
            }
        }
        if fighter_kind == *FIGHTER_KIND_LUIGI{
            if param_hash == smash::hash40("discharge_prob") {
                return 100.0;
            }
            else {
                return ret;
            }
        }
        if fighter_kind == *FIGHTER_KIND_LUCARIO{
            if param_hash == smash::hash40("min_aurapower") {
                return 1.67;
            }
            else {
                return ret;
            }
        }
        if fighter_kind == *FIGHTER_KIND_WARIO{
            if param_hash == smash::hash40("gass_max_time") {
                return 0.1;
            }
            if param_hash == smash::hash40("gass_middle") {
                return 0.1;
            }
            if param_hash == smash::hash40("gass_large_time") {
                return 0.1;
            }
            else {
                return ret;
            }
        }
        if fighter_kind == *FIGHTER_KIND_JACK{
            if param_hash == smash::hash40("rebel_gauge_add") {
                return 999999999.0;
            }
            else {
                return ret;
            }
        }
        if fighter_kind == *FIGHTER_KIND_GEKKOUGA{
            if param_hash == smash::hash40("charge_frame_max") {
                return 1.0;
            }
            else {
                return ret;
            }
        }
        if fighter_kind == *FIGHTER_KIND_KOOPAJR{
            if param_hash == smash::hash40("charge_frame") {
                return 1.0;
            }
            else {
                return ret;
            }
        }
        if fighter_kind == *FIGHTER_KIND_LINK{
            if param_hash == smash::hash40("bow_charge_spd_div") {
                return 1E-06;
            }
            else {
                return ret;
            }
        }
        if fighter_kind == *FIGHTER_KIND_TOONLINK{
            if param_hash == smash::hash40("bow_charge_spd_div") {
                return 1E-06;
            }
            else {
                return ret;
            }
        }
        if fighter_kind == *FIGHTER_KIND_SHEIK{
            if param_hash == smash::hash40("charge_speed_mul") {
                return 1E-06;
            }
            else {
                return ret;
            }
        }

        if fighter_kind == *FIGHTER_KIND_DEDEDE{
            if param_hash == smash::hash40("hold_max_damage_max") {
                return 1.0;
            }
            if param_hash == smash::hash40("hold_max_f") {
                return 1.0;
            }
            else {
                return ret;
            }
        }
        if fighter_kind == *FIGHTER_KIND_DOLLY{
            if param_hash == smash::hash40("super_special_damage") {
                return 0.0;
            }
            if param_hash == smash::hash40("super_special_hp_min") {
                return 0.0;
            }
            else {
                return ret;
            }
        }
        if fighter_kind == *FIGHTER_KIND_DIDDY{
            if param_hash == smash::hash40("special_n_charge_spd_mul") {
                return 1e-08;
            }
            else {
                return ret;
            }
        }
        else {
            return ret;
        }
    }
    else {
        return ret;
    }
}

#[skyline::main(name = "super_turbo_mode")]
pub fn main() {
    unsafe{
        let text_ptr = getRegionAddress(Region::Text) as *const u8;
        let text_size = (getRegionAddress(Region::Rodata) as usize) - (text_ptr as usize);
        let text = std::slice::from_raw_parts(text_ptr, text_size);
        if let Some(offset) = find_subsequence(text, FLOAT_SEARCH_CODE) {
            FLOAT_OFFSET = offset;
        }
        if let Some(offset) = find_subsequence(text, INT_SEARCH_CODE) {
            INT_OFFSET = offset;
        }
    }
    //config::other_configs();
    byleth::install();
    mario::install();
    //::install();
    koopa::install();
    custom::install();
    fox::install();
    ganon::install();
    captain::install();
    ivysaur::install();
    simonandrichter::install();
    donkey::install();
    dedede::install();
    cloud::install();
    squirtle::install();
    //samusanddarksamus::install();
    marth::install();
    krool::install();
    ike::install();
    shulk::install();
    zss::install();
    drmario::install();
    ness::install();
    roy::install();
    corrin::install();
    charizard::install();
    wolf::install();
    daisy::install();
    peach::install();
    incineroar::install();
    lucas::install();
    ridley::install();
    banjo::install();
    mac::install();
    ken::install();
    terry::install();
    darkpit::install();
    pit::install();
    link::install();
    younglink::install();
    hero::install();
    joker::install();
    wario::install();
    luigi::install();
    mewtwo::install();
    snake::install();
    palutena::install();
    bowserjr::install();
    diddykong::install();
    falco::install();
    samus::install();
    samusd::install();
    lucario::install();
    kirby::install();
    zelda::install();
    greninja::install();
    rob::install();
    metaknight::install();
    pyra::install();
    skyline::install_hooks!(get_param_int_replace, get_param_float_replace);
    sephiroth::install();
}