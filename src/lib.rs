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

//mod sephiroth;
#[skyline::hook(offset = INT_OFFSET)]
pub unsafe fn get_param_int_replace(boma: u64, param_type: u64, param_hash: u64) -> i32 {
    let module_accessor = &mut *(*((boma as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
    let ret = original!()(boma, param_type, param_hash);
    let fighter_kind = utility::get_kind(module_accessor);
    if fighter_kind == *FIGHTER_KIND_GEKKOUGA{
        if param_hash == smash::hash40("charge_frame_max") {
            return 0;
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
    skyline::install_hook!(get_param_int_replace);
    //sephiroth::install();
}