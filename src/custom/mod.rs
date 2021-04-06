use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::lua2cpp::{L2CFighterCommon, L2CFighterBase};
use smash::hash40;
use smash::lib::lua_const::*;
use smash::params::*;
use smash::*;
use acmd::{acmd, acmd_func};
use smash::app::lua_bind::StatusModule::*;
use smash::params::*;
use crate::config::CONFIG;
use smash::app::AttackData;
use smash::app::SituationKind;
use std::path::Path;
use std::fs;
use std::io::{Seek, SeekFrom};
use std::io::Error;
use skyline::nn::ro::LookupSymbol;
use smash::app::sv_animcmd::PLAY_SE;
use http::request::*;

pub static mut FIGHTER_MANAGER_ADDR: usize = 0;
static mut entry_id : usize = 0;

pub fn other_funny_stuff(fighter: &mut L2CFighterCommon){
    unsafe{
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        entry_id = WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let fighter_kind = smash::app::utility::get_kind(module_accessor);
        let lua_state = fighter.lua_state_agent;
        LookupSymbol(
            &mut FIGHTER_MANAGER_ADDR,
            "_ZN3lib9SingletonIN3app14FighterManagerEE9instance_E\u{0}"
                .as_bytes()
                .as_ptr(),);
        let fighter_manager = *(FIGHTER_MANAGER_ADDR as *mut *mut smash::app::FighterManager);
        if FighterManager::is_melee_mode_homerun(fighter_manager){
            AttackModule::set_power_mul(module_accessor, CONFIG.homerun_contest.power_multiplier.parse().unwrap());
        }
        if ControlModule::get_stick_y(module_accessor) == -1.0{
            FighterManager::set_position_lock(fighter_manager, smash::app::FighterEntryID(entry_id as i32), true);
        }
        else{
            FighterManager::set_position_lock(fighter_manager, smash::app::FighterEntryID(entry_id as i32), false);
        }
    }
}


pub fn config_implementations(fighter : &mut L2CFighterCommon) {
    unsafe {
        //status kind 83 is on the ground for missed tech
        //status kind 103 is teching ground
        //status kind 34 is air dodging
        //status kind 32 is dodging
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let fighter_kind = smash::app::utility::get_kind(module_accessor);
        let special: [i32 ; 47] = [*FIGHTER_STATUS_KIND_SPECIAL_HI,  *FIGHTER_IKE_STATUS_KIND_SPECIAL_HI_2, *FIGHTER_IKE_STATUS_KIND_SPECIAL_HI_3, *FIGHTER_IKE_STATUS_KIND_SPECIAL_HI_4, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI, *FIGHTER_ROY_STATUS_KIND_SPECIAL_HI_2, *FIGHTER_ROY_STATUS_KIND_SPECIAL_HI_3, *FIGHTER_ROY_STATUS_KIND_SPECIAL_HI_4, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI2, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI3, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI4, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_HI_A, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_HI_G, *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_2, *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_3,*FIGHTER_FOX_STATUS_KIND_SPECIAL_HI_RUSH, *FIGHTER_JACK_STATUS_KIND_SPECIAL_HI_CUT, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_END,*FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_2, *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_3, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_END, *FIGHTER_REFLET_STATUS_KIND_SPECIAL_HI_2, *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_FALL, *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_JUMP, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_HI_CHARGE, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_HI_UPPER, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_HI_FALL_ROLL, *FIGHTER_FOX_STATUS_KIND_SPECIAL_HI_BOUND,*FIGHTER_POPO_STATUS_KIND_SPECIAL_HI_FAIL, *FIGHTER_POPO_STATUS_KIND_SPECIAL_HI_JUMP, *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_HI_END, *FIGHTER_SHULK_STATUS_KIND_SPECIAL_HI_ADD,*FIGHTER_SNAKE_STATUS_KIND_SPECIAL_HI_CUT,*FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_END,  *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_FLAP, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_TURN, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_WAIT, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_DETACH, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_LANDING,*FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_END, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_RUSH, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_LANDING, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_CHARGED_RUSH, *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_RUSH, *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_BOUND, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_HI_JUMP, *FIGHTER_ROCKMAN_STATUS_KIND_SPECIAL_HI_JUMP];
        let lua_state = fighter.lua_state_agent;
        //Status 92 - 97 are shield break
        let no_aerial_smash_status: [i32; 27] = [*FIGHTER_STATUS_KIND_FALL_SPECIAL, 458, 73, 92, 93, 94, 95, 96, *FIGHTER_STATUS_KIND_DAMAGE_AIR, *FIGHTER_STATUS_KIND_DAMAGE_FLY, *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL, *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D, *FIGHTER_STATUS_KIND_DAMAGE_FALL, *FIGHTER_STATUS_KIND_FINAL, *FIGHTER_STATUS_KIND_SLEEP, *FIGHTER_STATUS_KIND_ESCAPE_B, *FIGHTER_STATUS_KIND_ESCAPE_F, *FIGHTER_STATUS_KIND_CAPTURE_JACK_WIRE, *FIGHTER_STATUS_KIND_CAPTURE_MASTERHAND, *FIGHTER_STATUS_KIND_CAPTURE_MASTER_SWORD, *FIGHTER_STATUS_KIND_SWALLOWED, *FIGHTER_STATUS_KIND_AIR_LASSO, *FIGHTER_STATUS_KIND_CATCHED_REFLET, *FIGHTER_STATUS_KIND_CATCHED_RIDLEY];
        //println!{"{}", StatusModule::status_kind(module_accessor)}
        let curr_status = StatusModule::status_kind(module_accessor);
       // println!("{}", StatusModule::status_kind(module_accessor));
        if AttackModule::is_infliction(module_accessor, *COLLISION_KIND_MASK_HIT){
            CancelModule::enable_cancel(module_accessor);
        }
        //WorkModule::is_flag(module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_GAUGE_CHARGE);
        /*
        if fighter_kind == *FIGHTER_KIND_ROY{
            if CONFIG.roy_changes.giant_sword{
                let scale = smash::phx::Vector3f{x:CONFIG.roy_changes.giant_sword_scale_x.parse().unwrap(),y:CONFIG.roy_changes.giant_sword_scale_y.parse().unwrap(),z: CONFIG.roy_changes.giant_sword_scale_z.parse().unwrap()};
                ModelModule::set_joint_scale(module_accessor, smash::phx::Hash40::new("sword1"), &scale);
            }
            //AttackModule::set_attack_scale(module_accessor, 5.0, true);
        }

         */
        //println!("{}", StatusModule::status_kind(module_accessor));
        /*************************************************************************************************************************************************************************************************
        PARAMETER CONFIG STUFF
        **************************************************************************************************************************************************************************************************/



        /***********************************************************************************************************************************************************************************************************
            PARAMETER CONFIG STUFF
         ***********************************************************************************************************************************************************************************************************/
        if special.contains(&curr_status){
            CancelModule::enable_cancel(module_accessor);
        }
        //println!("{}", status_kind(module_accessor));
        //Status kind 73 is hit by spike
        //Status kind 458 is dead
        if WorkModule::is_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR) && CONFIG.misc.infinite_airdodges{
            WorkModule::off_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR)
        }
        if status_kind(module_accessor) == *FIGHTER_STATUS_KIND_ATTACK && CONFIG.misc.jab_cancels{
            CancelModule::enable_cancel(module_accessor);
        }
        if status_kind(module_accessor) == *FIGHTER_STATUS_KIND_ATTACK_100 && CONFIG.misc.jab_cancels{
            CancelModule::enable_cancel(module_accessor);
        }
        if StatusModule::status_kind(module_accessor) == 34 && CONFIG.misc.airdodge_cancels{
            CancelModule::enable_cancel(module_accessor);
        }
        if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_HI && MotionModule::frame(module_accessor) != MotionModule::end_frame(module_accessor) && CONFIG.misc.up_special_cancels{
            CancelModule::enable_cancel(module_accessor);
        }
        if fighter_kind == *FIGHTER_KIND_LITTLEMAC && CONFIG.instant_info.littlemac && StatusModule::status_kind(module_accessor) == 481{
            CancelModule::enable_cancel(module_accessor);
            if ControlModule::get_stick_y(module_accessor) == 1.0{
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_HI, true);
            }
        }
        if CONFIG.misc.aerial_smash_attacks && !no_aerial_smash_status.contains(&curr_status){
            //CancelModule::enable_cancel(module_accessor);
            if ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L) || ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R){
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ATTACK_S4, true);
            }
            if ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI){
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ATTACK_HI4, true);
            }
            if ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW){
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ATTACK_LW4, true);
            }
            if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_ATTACK_S4{
                StatusModule::set_situation_kind(module_accessor, SituationKind(*SITUATION_KIND_GROUND), true)
            }
            if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_ATTACK_HI4{
                StatusModule::set_situation_kind(module_accessor, SituationKind(*SITUATION_KIND_GROUND), true)
            }
            if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_ATTACK_LW4{
                StatusModule::set_situation_kind(module_accessor, SituationKind(*SITUATION_KIND_GROUND), true)
            }
        }

        }
    }



pub fn install() {
    acmd::add_custom_hooks!(config_implementations);
    acmd::add_custom_hooks!(other_funny_stuff);

}