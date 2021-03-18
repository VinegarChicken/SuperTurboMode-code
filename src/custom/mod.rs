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
use crate::config;
use smash::app::SituationKind;


// Use this for general per-frame fighter-level hooks
pub fn once_per_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        //status kind 83 is on the ground for missed tech
        //status kind 103 is teching ground
        //status kind 34 is air dodging
        //status kind 32 is dodging
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let fighter_kind = smash::app::utility::get_kind(module_accessor);
        let lua_state = fighter.lua_state_agent;
        //println!{"{}", StatusModule::status_kind(module_accessor)}
        if AttackModule::is_infliction(module_accessor, *COLLISION_KIND_MASK_HIT){
            CancelModule::enable_cancel(module_accessor);
            //println!("{}", amount_contacts.to_string() + "" + "Contacts.");
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

        if CONFIG.misc.aerial_smash_attacks{
            if ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L) || ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R){
                if StatusModule::status_kind(module_accessor) != 73 && StatusModule::status_kind(module_accessor) != *FIGHTER_STATUS_KIND_FALL_SPECIAL{
                    StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ATTACK_S4, true);
                }
            }
            if ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI){
                if StatusModule::status_kind(module_accessor) != 73 && StatusModule::status_kind(module_accessor) != *FIGHTER_STATUS_KIND_FALL_SPECIAL{
                    StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ATTACK_HI4, true);
                }
            }
            if ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW){
                if StatusModule::status_kind(module_accessor) != 73 && StatusModule::status_kind(module_accessor) != *FIGHTER_STATUS_KIND_FALL_SPECIAL{
                    StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ATTACK_LW4, true);
                }
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
    acmd::add_custom_hooks!(once_per_fighter_frame);

}