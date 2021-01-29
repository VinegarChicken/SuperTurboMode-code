use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::lua2cpp::{L2CFighterCommon, L2CFighterBase};
use smash::hash40;
use smash::lib::lua_const::*;
use smash::*;
use acmd::{acmd, acmd_func};
use smash::app::lua_bind::StatusModule::*;
use smash::params::*;
use smash::cpp::root::app::{ItemKind, GroundTouchFlag, SituationKind};
use crate::config::CONFIG;
use crate::config;
// Use this for general per-frame fighter-level hooks
pub fn once_per_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        //status kind 83 is on the ground for missed tech
        //status kind 103 is teching ground
        //status kind 34 is air dodging
        //status kind 32 is dodging

        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let fighter_kind = smash::app::utility::get_kind(module_accessor);
        if status_kind(module_accessor) == *FIGHTER_STATUS_KIND_ATTACK{
            CancelModule::enable_cancel(module_accessor);
        }
        if status_kind(module_accessor) == *FIGHTER_STATUS_KIND_ATTACK_100 && CONFIG.misc.jab_cancels{
            CancelModule::enable_cancel(module_accessor);
        }
        if StatusModule::status_kind(module_accessor) == 34 && CONFIG.misc.airdodge_cancels{
            CancelModule::enable_cancel(module_accessor);
        }
        if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_HI && CONFIG.misc.up_special_cancels{
            CancelModule::enable_cancel(module_accessor);
        }


    }
}



pub fn install() {
    acmd::add_custom_hooks!(once_per_fighter_frame);
}