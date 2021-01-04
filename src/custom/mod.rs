use smash::app::lua_bind::*;
use smash::app::utility::get_kind;
use smash::lib::lua_const::*;
use smash::lua2cpp::{L2CFighterCommon, L2CFighterBase};
use smash::hash40;
use smash::lib::lua_const::*;
use smash::*;
use acmd::{acmd, acmd_func};

// Use this for general per-frame fighter-level hooks
pub fn once_per_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        let lua_state = fighter.lua_state_agent;
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let fighter_kind = smash::app::utility::get_kind(module_accessor);
        if fighter_kind == *FIGHTER_KIND_PURIN{
            acmd!(lua_state,{
            ATTACK(ID=0, Part=0, Bone=hash40("rot"), Damage=100.0, Angle=88, KBG=66, FKB=0, BKB=100, Size=8.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_flower"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_BAT, Type=ATTACK_REGION_BODY)
            });
        }
        if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_ATTACK{
            CancelModule::enable_cancel(module_accessor);
        }
        if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_ATTACK_100{
            CancelModule::enable_cancel(module_accessor);
        }

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