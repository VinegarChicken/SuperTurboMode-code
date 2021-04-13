#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]



use skyline::*;
use std::ffi::CStr;
use smash::resource::{LoadedTables, find_subsequence};
use skyline::hooks::{getRegionAddress, Region};
use std::path::Path;
use std::fs;
use crate::config::CONFIG;



mod mario;
mod falco;
mod custom;
mod koopa;
mod fox;
mod ganon;
mod captain;
mod ivysaur;
mod simonandrichter;
mod donkey;
mod diddy;
mod dedede;
mod cloud;
mod squirtle;
mod samusanddarksamus;
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
//mod terry;

//mod sephiroth;


#[skyline::main(name = "super_turbo_mode")]
pub fn main() {
    byleth::install();
    mario::install();
    falco::install();
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
    config::other_configs();
    darkpit::install();
    pit::install();
    //sephiroth::install();
}