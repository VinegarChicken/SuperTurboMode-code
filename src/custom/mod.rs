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
use skyline::nn::ro::LookupSymbol;
use smash::app::{SituationKind, BattleObjectModuleAccessor};

pub unsafe fn is_damage_check(boma : &mut BattleObjectModuleAccessor) -> bool {
    if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_DAMAGE
    || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_DAMAGE_AIR
    || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_THROWN
    || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_CAPTURE_WAIT
    || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE
    || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_DAMAGE_FLY
    || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL
    || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR
    || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR
    || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U
    || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D
    || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_DAMAGE_FALL
    || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_FINAL
    || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_SLEEP
    || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_ESCAPE_B
    || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_ESCAPE_F
    || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_ESCAPE
    || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_CLIFF_ESCAPE
    || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_ESCAPE_AIR
    || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_CAPTURE_JACK_WIRE
    || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_CAPTURE_MASTERHAND
    || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_CAPTURE_MASTER_SWORD
    || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_SWALLOWED
    || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_AIR_LASSO
    || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_CATCHED_REFLET
    || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_CATCHED_RIDLEY
    || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_ATTACK_AIR
    || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_MISS_FOOT
    || WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAPTURE_YOSHI)
    || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_DEAD
    || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_REBIRTH
    || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_BURY
    || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_BURY_WAIT
    || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_ICE {
        return true;
    }
    else {
        return false;
    }
}

pub unsafe fn smash_atks(module_accessor: &mut BattleObjectModuleAccessor){
    let fighter_kind = smash::app::utility::get_kind(module_accessor);
    let no_aerial_smash_status: [i32; 27] = [*FIGHTER_STATUS_KIND_FALL_SPECIAL, 458, 73, 92, 93, 94, 95, 96, *FIGHTER_STATUS_KIND_DAMAGE_AIR, *FIGHTER_STATUS_KIND_DAMAGE_FLY, *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL, *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D, *FIGHTER_STATUS_KIND_DAMAGE_FALL, *FIGHTER_STATUS_KIND_FINAL, *FIGHTER_STATUS_KIND_SLEEP, *FIGHTER_STATUS_KIND_ESCAPE_B, *FIGHTER_STATUS_KIND_ESCAPE_F, *FIGHTER_STATUS_KIND_CAPTURE_JACK_WIRE, *FIGHTER_STATUS_KIND_CAPTURE_MASTERHAND, *FIGHTER_STATUS_KIND_CAPTURE_MASTER_SWORD, *FIGHTER_STATUS_KIND_SWALLOWED, *FIGHTER_STATUS_KIND_AIR_LASSO, *FIGHTER_STATUS_KIND_CATCHED_REFLET, *FIGHTER_STATUS_KIND_CATCHED_RIDLEY];
    let curr_status = StatusModule::status_kind(module_accessor);
    if MotionModule::motion_kind(module_accessor) == smash::hash40("appeal_s_r") ||
        MotionModule::motion_kind(module_accessor) == smash::hash40("appeal_s_l") {
        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ATTACK_S4, true);
    }
    if MotionModule::motion_kind(module_accessor) == smash::hash40("appeal_hi_r") ||
        MotionModule::motion_kind(module_accessor) == smash::hash40("appeal_hi_l") {
        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ATTACK_HI4, true);
    }
    if MotionModule::motion_kind(module_accessor) == smash::hash40("appeal_lw_r") ||
        MotionModule::motion_kind(module_accessor) == smash::hash40("appeal_lw_l") {
        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ATTACK_LW4, true);
    }
    if CONFIG.misc.aerial_smash_attacks && !is_damage_check(module_accessor) && !no_aerial_smash_status.contains(&curr_status){&& StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_AIR
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
pub static mut FIGHTER_MANAGER_ADDR: usize = 0;
static mut entry_id : usize = 0;
static mut jump_count:i32 = 0;
static mut total_jumps:i32 = 0;
static mut should_jump:bool = false;
pub unsafe fn jump(module_accessor: &mut BattleObjectModuleAccessor){
    total_jumps = WorkModule::get_param_int(module_accessor, hash40("jump_count_max"), 0);
    if StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_AIR && !is_damage_check(module_accessor){
        if ControlModule::check_button_on_trriger(module_accessor, *CONTROL_PAD_BUTTON_JUMP) && should_jump{
            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_JUMP_AERIAL, true);
            jump_count+=1
        }
        if jump_count == 5{
            should_jump = false;
        }
    }
    else{
        jump_count = 0;
        should_jump = true;
    }
}



pub unsafe fn cancels(module_accessor: &mut BattleObjectModuleAccessor){
    if AttackModule::is_infliction(module_accessor, *COLLISION_KIND_MASK_HIT){
        //CancelModule::enable_cancel(module_accessor);
    }
    let fighter_kind = smash::app::utility::get_kind(module_accessor);
    let motion_kind = MotionModule::motion_kind(module_accessor);
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
    if fighter_kind == *FIGHTER_KIND_FALCO{
        if motion_kind == smash::hash40("special_s_start"){
            MotionModule::set_rate(module_accessor, 23.0);
        }
        if motion_kind == smash::hash40("special_air_s_start"){
            MotionModule::set_rate(module_accessor, 23.0);
        }
    }
    if fighter_kind == *FIGHTER_KIND_FOX{
        if motion_kind == smash::hash40("special_s_start"){
            MotionModule::set_rate(module_accessor, 23.0);
        }
        if motion_kind == smash::hash40("special_air_s_start"){
            MotionModule::set_rate(module_accessor, 23.0);
        }
    }
    if fighter_kind == *FIGHTER_KIND_WOLF{
        if motion_kind == smash::hash40("special_s_start"){
            MotionModule::set_rate(module_accessor, 23.0);
        }
        if motion_kind == smash::hash40("special_air_s_start"){
            MotionModule::set_rate(module_accessor, 23.0);
        }
    }
    if fighter_kind == *FIGHTER_KIND_PLIZARDON{
        if motion_kind == smash::hash40("special_s_start"){
            CancelModule::enable_cancel(module_accessor);
            MotionModule::set_rate(module_accessor, 23.0);
        }
        if motion_kind == smash::hash40("special_s") || motion_kind == smash::hash40("special_s_blown"){
            CancelModule::enable_cancel(module_accessor);
        }
        if motion_kind == smash::hash40("special_air_s") || motion_kind == smash::hash40("special_air_s_blown"){
            CancelModule::enable_cancel(module_accessor);
        }
        if motion_kind == smash::hash40("special_air_s_start"){
            CancelModule::enable_cancel(module_accessor);
            MotionModule::set_rate(module_accessor, 23.0);
        }
    }
}
pub unsafe fn enable_tilts_force(module_accessor: &mut BattleObjectModuleAccessor){
    LookupSymbol(
        &mut FIGHTER_MANAGER_ADDR,
        "_ZN3lib9SingletonIN3app14FighterManagerEE\
      9instance_E\u{0}"
            .as_bytes()
            .as_ptr(), );
    let mut ENTRY_ID = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let FIGHTER_MANAGER = *(FIGHTER_MANAGER_ADDR as *mut *mut smash::app::FighterManager);
    let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
    if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) != 0 && !is_damage_check(module_accessor) && StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_GROUND && !FighterInformation::is_operation_cpu(FighterManager::get_fighter_information(FIGHTER_MANAGER,smash::app::FighterEntryID(ENTRY_ID as i32))) {
        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ATTACK_S3, true);
    }
    if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) != 0 && !is_damage_check(module_accessor) && StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_GROUND && !FighterInformation::is_operation_cpu(FighterManager::get_fighter_information(FIGHTER_MANAGER,smash::app::FighterEntryID(ENTRY_ID as i32))) {
        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ATTACK_HI3, true);
    }
    if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) != 0 && !is_damage_check(module_accessor) && StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_GROUND && !FighterInformation::is_operation_cpu(FighterManager::get_fighter_information(FIGHTER_MANAGER,smash::app::FighterEntryID(ENTRY_ID as i32))) {
        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ATTACK_LW3, true);
    }
}

pub unsafe fn enable_specials_force(module_accessor: &mut BattleObjectModuleAccessor){
    LookupSymbol(
        &mut FIGHTER_MANAGER_ADDR,
        "_ZN3lib9SingletonIN3app14FighterManagerEE\
      9instance_E\u{0}"
            .as_bytes()
            .as_ptr(), );
    let mut ENTRY_ID = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let FIGHTER_MANAGER = *(FIGHTER_MANAGER_ADDR as *mut *mut smash::app::FighterManager);
    let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
    if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) != 0 && !is_damage_check(module_accessor) && !FighterInformation::is_operation_cpu(FighterManager::get_fighter_information(FIGHTER_MANAGER,smash::app::FighterEntryID(ENTRY_ID as i32))) {
        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_N, true);
    }
    if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) != 0 && !is_damage_check(module_accessor) && !FighterInformation::is_operation_cpu(FighterManager::get_fighter_information(FIGHTER_MANAGER,smash::app::FighterEntryID(ENTRY_ID as i32))) {
        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_S, true);
    }
    if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0 && !is_damage_check(module_accessor) && !FighterInformation::is_operation_cpu(FighterManager::get_fighter_information(FIGHTER_MANAGER,smash::app::FighterEntryID(ENTRY_ID as i32))) {
        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_HI, true);
    }
    if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0 && !is_damage_check(module_accessor)  && ![*FIGHTER_KIND_PTRAINER, *FIGHTER_KIND_PLIZARDON, *FIGHTER_KIND_PZENIGAME, *FIGHTER_KIND_PFUSHIGISOU, *FIGHTER_KIND_TANTAN, *FIGHTER_KIND_PIKMIN].contains(&smash::app::utility::get_kind(module_accessor)) && !FighterInformation::is_operation_cpu(FighterManager::get_fighter_information(FIGHTER_MANAGER,smash::app::FighterEntryID(ENTRY_ID as i32))) {
        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_LW, true);
    }
}

pub unsafe fn enable_smash_atk_force(module_accessor: &mut BattleObjectModuleAccessor){
    LookupSymbol(
        &mut FIGHTER_MANAGER_ADDR,
        "_ZN3lib9SingletonIN3app14FighterManagerEE\
      9instance_E\u{0}"
            .as_bytes()
            .as_ptr(), );
    let mut ENTRY_ID = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    let FIGHTER_MANAGER = *(FIGHTER_MANAGER_ADDR as *mut *mut smash::app::FighterManager);
    let cat1 = ControlModule::get_command_flag_cat(module_accessor, 0);
    if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4) != 0 && !is_damage_check(module_accessor) && StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_GROUND && !FighterInformation::is_operation_cpu(FighterManager::get_fighter_information(FIGHTER_MANAGER,smash::app::FighterEntryID(ENTRY_ID as i32))) {
        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ATTACK_S4, true);
    }
    if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) != 0 && !is_damage_check(module_accessor) && StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_GROUND && !FighterInformation::is_operation_cpu(FighterManager::get_fighter_information(FIGHTER_MANAGER,smash::app::FighterEntryID(ENTRY_ID as i32))) {
        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ATTACK_HI4, true);   WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S3);
    }
    if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) != 0 && !is_damage_check(module_accessor) && StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_GROUND && !FighterInformation::is_operation_cpu(FighterManager::get_fighter_information(FIGHTER_MANAGER,smash::app::FighterEntryID(ENTRY_ID as i32))) {
        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ATTACK_LW4, true);
    }
}

pub unsafe fn enable_all(module_accessor: &mut BattleObjectModuleAccessor){
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S3);
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI3);
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW3);
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK);
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_HOLD);
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_HOLD);
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_HOLD);
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START);
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START);
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START);
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_B);
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_F);
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH);
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH);
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN);
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N);
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S);
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW);
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT);
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL);
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON);
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_DASH);
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_B);
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_F);
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN);
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_RUN);
    WorkModule::enable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH);
}


pub unsafe fn enable_jump_force(module_accessor: &mut BattleObjectModuleAccessor){
    if ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_JUMP) && StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_AIR{
        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_JUMP_AERIAL, true);
    }
    else if ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_JUMP) && StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_GROUND{
        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_JUMP, true);
    }
}

pub fn extras(module_accessor: &mut BattleObjectModuleAccessor){
    unsafe{
        entry_id = WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let fighter_kind = smash::app::utility::get_kind(module_accessor);
        let status = StatusModule::status_kind(module_accessor);
        LookupSymbol(
            &mut FIGHTER_MANAGER_ADDR,
            "_ZN3lib9SingletonIN3app14FighterManagerEE9instance_E\u{0}"
                .as_bytes()
                .as_ptr(),);
        let fighter_manager = *(FIGHTER_MANAGER_ADDR as *mut *mut smash::app::FighterManager);
        if FighterManager::is_melee_mode_homerun(fighter_manager){

        }
        if FighterManager::is_result_mode(fighter_manager){
            smash_atks(module_accessor);
        }
        if fighter_kind == *FIGHTER_KIND_BRAVE && [*FIGHTER_BRAVE_STATUS_KIND_SPECIAL_S_ATTACK1, *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_S_ATTACK2, *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_S_HOLD].contains(&status){
            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_S_ATTACK3, true);
        }
        if fighter_kind == *FIGHTER_KIND_MEWTWO && status == *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_N_HOLD{
            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_N_SHOOT, true);
        }
        //infinite airdodges
        if WorkModule::is_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR) && CONFIG.misc.infinite_airdodges{
            WorkModule::off_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR)
        }
    }
}

pub unsafe fn turbo_mode(module_accessor: &mut BattleObjectModuleAccessor){
    //let attack_statuses:[i32;20] = [*FIGHTER_STATUS_KIND_ATTACK_S4]
    WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let special: [i32; 225] = [*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_ROY_STATUS_KIND_SPECIAL_S2, *FIGHTER_ROY_STATUS_KIND_SPECIAL_S3, *FIGHTER_ROY_STATUS_KIND_SPECIAL_S4, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B, *FIGHTER_LINK_STATUS_KIND_SPECIAL_S2, *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_S2, *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_S3, *FIGHTER_IKE_STATUS_KIND_SPECIAL_HI_2, *FIGHTER_IKE_STATUS_KIND_SPECIAL_HI_3, *FIGHTER_IKE_STATUS_KIND_SPECIAL_HI_4, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S2, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S3, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S4, *FIGHTER_ROY_STATUS_KIND_SPECIAL_HI_2, *FIGHTER_ROY_STATUS_KIND_SPECIAL_HI_3, *FIGHTER_ROY_STATUS_KIND_SPECIAL_HI_4, *FIGHTER_SIMON_STATUS_KIND_SPECIAL_S2, *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_HI2, *FIGHTER_IKE_STATUS_KIND_SPECIAL_N_END, *FIGHTER_IKE_STATUS_KIND_SPECIAL_S_END, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI2, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI3, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI4, *FIGHTER_PIT_STATUS_KIND_SPECIAL_N_DIR, *FIGHTER_PIT_STATUS_KIND_SPECIAL_S_END, *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END, *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_END, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_C, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_E, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_F, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_H, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1A, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1G, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2A, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2G, *FIGHTER_YOSHI_STATUS_KIND_SPECIAL_N_1, *FIGHTER_YOSHI_STATUS_KIND_SPECIAL_N_2, *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_IKE_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_IKE_STATUS_KIND_SPECIAL_N_LOOP, *FIGHTER_IKE_STATUS_KIND_SPECIAL_S_DASH, *FIGHTER_IKE_STATUS_KIND_SPECIAL_S_HOLD, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_HI_A, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_HI_G, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_LW_A, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_LW_G, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_LW_L, *FIGHTER_NESS_STATUS_KIND_SPECIAL_N_END, *FIGHTER_PIT_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_PIT_STATUS_KIND_SPECIAL_N_TURN, *FIGHTER_ROY_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END2, *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_END3, *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_LOOP, *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_TURN, *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_LOOP, *FIGHTER_SHULK_STATUS_KIND_SPECIAL_LW_N, *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_2, *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_3, *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_END, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_F_END, *FIGHTER_FOX_STATUS_KIND_SPECIAL_HI_RUSH, *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_LOOP, *FIGHTER_JACK_STATUS_KIND_SPECIAL_HI_CUT, *FIGHTER_JACK_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_JACK_STATUS_KIND_SPECIAL_N_JUMP, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_N_END, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_N_END, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_S_GET, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_END, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_N_END, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_END, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_RAM, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_END, *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_2, *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_3, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_END, *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_NESS_STATUS_KIND_SPECIAL_N_FIRE, *FIGHTER_NESS_STATUS_KIND_SPECIAL_N_HOLD, *FIGHTER_PEACH_STATUS_KIND_SPECIAL_N_HIT, *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH, *FIGHTER_PIT_STATUS_KIND_SPECIAL_LW_HOLD, *FIGHTER_PIT_STATUS_KIND_SPECIAL_N_SHOOT, *FIGHTER_PURIN_STATUS_KIND_SPECIAL_N_END, *FIGHTER_REFLET_STATUS_KIND_SPECIAL_HI_2, *FIGHTER_ROBOT_STATUS_KIND_SPECIAL_S_END, *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_FALL, *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_JUMP, *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_S_END, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_HIT, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_END, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_EAT, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_END, *FIGHTER_WOLF_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_WOLF_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_YOSHI_STATUS_KIND_SPECIAL_S_END, *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_S_END, *ITEM_PACMANKEY_STATUS_KIND_SPECIAL_HAVE, *WEAPON_JACK_DOYLE_STATUS_KIND_SPECIAL_S, *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_N_HOLD, *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_S_HOLD, *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_DASH, *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_FAIL, *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_WALL, *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_N_END, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_S_GET, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_HI_CHARGE, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_HI_UPPER, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_HI_FALL_ROLL, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_N_BLOW, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_S_JUMP, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_S_KICK, *FIGHTER_DONKEY_STATUS_KIND_SPECIAL_N_END, *FIGHTER_FOX_STATUS_KIND_SPECIAL_HI_BOUND, *FIGHTER_GANON_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_GANON_STATUS_KIND_SPECIAL_N_TURN, *FIGHTER_IKE_STATUS_KIND_SPECIAL_S_ATTACK, *FIGHTER_JACK_STATUS_KIND_SPECIAL_HI2_END, *FIGHTER_JACK_STATUS_KIND_SPECIAL_HI_PULL, *FIGHTER_JACK_STATUS_KIND_SPECIAL_LW_HOLD, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_N_BITE, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_N_HOLD, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_N_LOOP, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_N_SPIT, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_FALL, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_JUMP, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_PASS, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_TURN, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_WAIT, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_WALK, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_S_FALL, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_S_JUMP, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_N_LOOP, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_N_SPIT, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_HOLD, *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_N2, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_END, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_N_FIRE, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_N_HOLD, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_WALL, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_LOOP, *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_N_MAX, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_HOLD, *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_HOLD, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_N_END, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_S_END, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_HI_END, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_N_LOOP, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_N_WAIT, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_LW_BITE, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_N_SHOOT, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_S_SHOOT, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_S_CANCEL, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_S_CHARGE, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_LW_CHARGE, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_N_FAILURE, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_N_HIT_END, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_HI_LANDING, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_LW_FALL_END, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_S_JUMP_CANCEL, *FIGHTER_PEACH_STATUS_KIND_SPECIAL_S_JUMP, *FIGHTER_PIT_STATUS_KIND_SPECIAL_N_CHARGE, *FIGHTER_POPO_STATUS_KIND_SPECIAL_HI_FAIL, *FIGHTER_POPO_STATUS_KIND_SPECIAL_HI_JUMP, *FIGHTER_PURIN_STATUS_KIND_SPECIAL_N_HOLD, *FIGHTER_PURIN_STATUS_KIND_SPECIAL_N_ROLL, *FIGHTER_PURIN_STATUS_KIND_SPECIAL_N_TURN, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_S_CUT, *FIGHTER_ROBOT_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW, *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_HI_END, *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_N_LOOP, *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_S_FIRE, *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_S_HOLD, *FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_S_CUT, *FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_S_END, *FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_S_HIT, *FIGHTER_SHULK_STATUS_KIND_SPECIAL_HI_ADD, *FIGHTER_SHULK_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_SHULK_STATUS_KIND_SPECIAL_S_FALL, *FIGHTER_SHULK_STATUS_KIND_SPECIAL_S_JUMP, *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_HI_CUT, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_N_FAIL, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_DASH, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_HOLD, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_TURN, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_BITE, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_BOMB, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_FOOD, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_ITEM, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_BUMP, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_DOWN, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_RIDE, *FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_N_END, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_END, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_FLAP, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_TURN, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_WAIT, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_DETACH, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_LANDING, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_END, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_RUSH, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_LANDING, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_CHARGED_RUSH, *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_RUSH, *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_BOUND, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_HI_JUMP, *FIGHTER_ROCKMAN_STATUS_KIND_SPECIAL_HI_JUMP];

    let status_kind = StatusModule::status_kind(module_accessor);
    for i in 0..8{
        if AttackModule::is_attack(module_accessor, i, false) ||
            special.contains(&status_kind) ||
            status_kind == *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR||
            status_kind == *FIGHTER_STATUS_KIND_THROW && MotionModule::frame(module_accessor) >= 1.0{
            CancelModule::enable_cancel(module_accessor);
        }
    }

    if !is_damage_check(module_accessor){
        enable_specials_force(module_accessor);
        //enable_jump_force(module_accessor);
        enable_smash_atk_force(module_accessor);
        enable_tilts_force(module_accessor);
        enable_all(module_accessor);
    }

    if [*FIGHTER_STATUS_KIND_THROW, *FIGHTER_STATUS_KIND_CATCH].contains(&status_kind) || special.contains(&status_kind){
        WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_WALK);
        WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN);
        WorkModule::unable_transition_term_group_ex(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SQUAT);
    }

}


pub fn per_frame(fighter : &mut L2CFighterCommon) {
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
        //println!{"{}", StatusModule::status_kind(module_accessor)}

        //println!("{}", status_kind(module_accessor));
        //Status kind 73 is hit by spike
        //Status kind 458 is dead
        smash_atks(module_accessor);
        cancels(module_accessor);
        extras(module_accessor);
        turbo_mode(module_accessor);

        }
    }



pub fn install() {
    acmd::add_custom_hooks!(per_frame);
}
