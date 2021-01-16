use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::lua2cpp::{L2CFighterCommon, L2CFighterBase};
use smash::hash40;
use smash::lib::lua_const::*;
use smash::*;
use acmd::{acmd, acmd_func};
use smash::app::lua_bind::StatusModule::*;
use smash::params::*;
use smash::cpp::root::app::ItemKind;

// Use this for general per-frame fighter-level hooks
pub fn once_per_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        //status kind 83 is on the ground for missed tech
        //status kind 87 is get up from missed tech.
        //status kind 103 is teching ground
        //status kind 88 is rolling from ground missed tech
        //status kind 34 is air dodging
        //status kind 32 is dodging
        let lua_state = fighter.lua_state_agent;
        let rand_val = app::sv_math::rand(hash40("fighter"), 100);
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let fighter_kind = smash::app::utility::get_kind(module_accessor);

        if status_kind(module_accessor) == *FIGHTER_STATUS_KIND_ATTACK{
            CancelModule::enable_cancel(module_accessor);
        }
        if status_kind(module_accessor) == *FIGHTER_STATUS_KIND_ATTACK_100{
            CancelModule::enable_cancel(module_accessor);
        }
        /*
        if status_kind(module_accessor) == 103{
           change_status_request_from_script(module_accessor, 83, true);
        }
        if status_kind(module_accessor) == 104{
            change_status_request_from_script(module_accessor, 83, true);
        }
         */
        if StatusModule::status_kind(module_accessor) == 34{
            CancelModule::enable_cancel(module_accessor);
        }

        if fighter_kind == *FIGHTER_KIND_DAISY{
            //ItemModule::have_item(module_accessor,ItemKind(*ITEM_KIND_BOMBHEI),0,0,false,false);
        }
        //println!("{}",StatusModule::status_kind(module_accessor) );
    }
}



pub fn install() {
    acmd::add_custom_hooks!(once_per_fighter_frame);
}