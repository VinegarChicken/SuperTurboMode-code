use smash::app::lua_bind::*;
use smash::app::utility::get_kind;
use smash::lib::lua_const::*;
use smash::lua2cpp::{L2CFighterCommon, L2CFighterBase};
use smash::hash40;
use smash::lib::lua_const::*;
use smash::*;
use acmd::{acmd, acmd_func};
use smash::app::sv_math::*;
// Use this for general per-frame fighter-level hooks
pub fn once_per_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        //status kind 83 is on the ground for missed tech
        //status kind 87 is get up from missed tech.
        //status kind 103 is teching ground
        //status kind 88 is rolling from ground missed tech
        let lua_state = fighter.lua_state_agent;
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let fighter_kind = smash::app::utility::get_kind(module_accessor);

        if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_ATTACK{
            CancelModule::enable_cancel(module_accessor);
        }
        if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_ATTACK_100{
            CancelModule::enable_cancel(module_accessor);
        }
        if StatusModule::status_kind(module_accessor) == 103{
           StatusModule::change_status_request_from_script(module_accessor,83,true);
        }
        if StatusModule::status_kind(module_accessor) == 104{
            StatusModule::change_status_request_from_script(module_accessor,83,true);
        }



            println!("{}",StatusModule::status_kind(module_accessor))



    }
}


// Use this for general per-frame weapon-level hooks
pub fn once_per_weapon_frame(fighter_base : &mut L2CFighterBase) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter_base.lua_state_agent);
        let frame = smash::app::lua_bind::MotionModule::frame(module_accessor) as i32;

        if frame % 10 == 0 {
            //println!("[Weapon Hook] Frame : {}", frame);
        }
    }
}

pub fn install() {
    acmd::add_custom_hooks!(once_per_fighter_frame);
    acmd::add_custom_weapon_hooks!(once_per_weapon_frame);

}