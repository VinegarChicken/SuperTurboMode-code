use smash::hash40;
use smash::lib::lua_const::*;
use smash::lua2cpp::{L2CFighterCommon,L2CFighterBase};
use acmd::{acmd, acmd_func};
use crate::config::CONFIG;
use smash::app::lua_bind::*;

#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_MARTH,
animation = "catch",
animcmd = "game_catch")]
pub fn marth_grab(fighter: &mut L2CFighterCommon) {
    acmd!({
    frame(Frame=5)
    if(is_excute){
        GrabModule::set_rebound(CanCatchRebound=true)
    }
    frame(Frame=6)
    if(is_excute){
        CATCH(ID=0, Bone=hash40("top"), Size=99999999999999999999999999999999.9, X=0.0, Y=8.0, Z=4.0, X2=0.0, Y2=8.0, Z2=7.7, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_G)
        CATCH(ID=1, Bone=hash40("top"), Size=99999999999999999999999999999999.9, X=0.0, Y=8.0, Z=2.35, X2=0.0, Y2=8.0, Z2=9.35, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_A)
    }
    game_CaptureCutCommon()
    wait(Frames=2)
    if(is_excute){
        sv_module_access::grab(MA_MSC_CMD_GRAB_CLEAR_ALL)
        WorkModule::on_flag(Flag=FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT)
        GrabModule::set_rebound(CanCatchRebound=false)
    }
    });

}
#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_MARTH,
animation = "catch_dash",
animcmd = "game_catchdash")]
pub fn marth_grab_dash(fighter: &mut L2CFighterCommon) {
    acmd!({
    frame(Frame=8)
    if(is_excute){
        GrabModule::set_rebound(CanCatchRebound=true)
    }
    frame(Frame=9)
    if(is_excute){
        CATCH(ID=0, Bone=hash40("top"), Size=99999999999999999999999999999999.9, X=0.0, Y=7.25, Z=4.0, X2=0.0, Y2=7.25, Z2=9.4, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_G)
        CATCH(ID=1, Bone=hash40("top"), Size=99999999999999999999999999999999.9, X=0.0, Y=7.0, Z=2.7, X2=0.0, Y2=7.0, Z2=10.7, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_A)
    }
    game_CaptureCutCommon()
    wait(Frames=2)
    if(is_excute){
        sv_module_access::grab(MA_MSC_CMD_GRAB_CLEAR_ALL)
        WorkModule::on_flag(Flag=FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT)
        GrabModule::set_rebound(CanCatchRebound=false)
    }
    });

}
#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_MARTH,
animation = "catch_turn",
animcmd = "game_catchturn")]
pub fn marth_grab_turn(fighter: &mut L2CFighterCommon) {
    acmd!({
    frame(Frame=9)
    if(is_excute){
        GrabModule::set_rebound(CanCatchRebound=true)
    }
    frame(Frame=10)
    if(is_excute){
        CATCH(ID=0, Bone=hash40("top"), Size=99999999999999999999999999999999.9, X=0.0, Y=7.0, Z=-4.0, X2=0.0, Y2=7.0, Z2=-13.2, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_G)
        CATCH(ID=1, Bone=hash40("top"), Size=99999999999999999999999999999999.9, X=0.0, Y=7.0, Z=-2.35, X2=0.0, Y2=7.0, Z2=-14.85, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_A)
    }
    game_CaptureCutCommon()
    wait(Frames=2)
    if(is_excute){
        sv_module_access::grab(MA_MSC_CMD_GRAB_CLEAR_ALL)
        WorkModule::on_flag(Flag=FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT)
        GrabModule::set_rebound(CanCatchRebound=false)
    }
    });

}

#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_MARTH,
animation = "attack_air_f",
animcmd = "game_attackairf")]
pub fn instant_marth_fair(fighter: &mut L2CFighterCommon) {
    acmd!({
	if (is_excute) {
        rust{
            if CONFIG.instant_info.marth{
            MotionModule::set_rate(module_accessor, 5.0)
        }
        }
    }
   	if(is_excute){
	WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
	}
	frame(Frame=6)
	if(is_excute){
	MotionModule::set_rate(1.0)
	ATTACK(ID=0, Part=0, Bone=hash40("sword1"), Damage=11.5, Angle=361, KBG=80, FKB=0, BKB=40, Size=3.0, X=1.0, Y=0.0, Z=2.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MARTH_SWORD, Type=ATTACK_REGION_SWORD)
	ATTACK(ID=1, Part=0, Bone=hash40("armr"), Damage=11.5, Angle=361, KBG=80, FKB=0, BKB=40, Size=3.8, X=1.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MARTH_SWORD, Type=ATTACK_REGION_SWORD)
	ATTACK(ID=2, Part=0, Bone=hash40("sword1"), Damage=11.5, Angle=361, KBG=80, FKB=0, BKB=40, Size=3.0, X=1.0, Y=0.0, Z=7.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MARTH_SWORD, Type=ATTACK_REGION_SWORD)
	}
	wait(Frames=3)
	if(is_excute){
	AttackModule::clear_all()
	}
	frame(Frame=36)
	if(is_excute){
	WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
	}


    });
}

#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_MARTH,
animation = "attack_air_lw",
animcmd = "game_attackairlw")]
pub fn instant_marth_dair(fighter: &mut L2CFighterCommon) {
    acmd!({
	if (is_excute) {
        rust{
            if CONFIG.instant_info.marth{
            MotionModule::set_rate(module_accessor, 8.0)
        }
        }
    }
        frame(Frame=3)
	if(is_excute){
	WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
	}
	frame(Frame=9)
	if(is_excute){
	MotionModule::set_rate(1.0)
	ATTACK(ID=1, Part=0, Bone=hash40("armr"), Damage=15.0, Angle=270, KBG=80, FKB=0, BKB=20, Size=3.0, X=0.0, Y=1.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MARTH_SWORD, Type=ATTACK_REGION_SWORD)
	ATTACK(ID=2, Part=0, Bone=hash40("sword1"), Damage=15.0, Angle=270, KBG=80, FKB=0, BKB=20, Size=3.5, X=1.0, Y=0.0, Z=2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MARTH_SWORD, Type=ATTACK_REGION_SWORD)
	ATTACK(ID=3, Part=0, Bone=hash40("sword1"), Damage=15.0, Angle=270, KBG=80, FKB=0, BKB=20, Size=3.5, X=1.0, Y=0.0, Z=6.7, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MARTH_SWORD, Type=ATTACK_REGION_SWORD)
	}
	frame(Frame=11)
	if(is_excute){
	ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=15.0, Angle=270, KBG=80, FKB=0, BKB=20, Size=5.0, X=0.0, Y=-3.3, Z=-3.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MARTH_SWORD, Type=ATTACK_REGION_SWORD)
	}

	frame(Frame=14)
	if(is_excute){
	AttackModule::clear_all()
	}
	frame(Frame=55)
	if(is_excute){
	WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
	}

    });
}
#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_MARTH,
animation = "attack_air_b",
animcmd = "game_attackairb")]
pub fn instant_marth_bair(fighter: &mut L2CFighterCommon) {
    acmd!({
	if (is_excute) {
        rust{
            if CONFIG.instant_info.marth{
            MotionModule::set_rate(module_accessor, 6.0)
        }
        }
    }
	frame(Frame=3)
	if(is_excute){
	WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
	REVERSE_LR()
	}
	frame(Frame=7)
	if(is_excute){
	MotionModule::set_rate(1.0)
	ATTACK(ID=0, Part=0, Bone=hash40("sword1"), Damage=12.5, Angle=361, KBG=94, FKB=0, BKB=40, Size=3.5, X=0.0, Y=0.0, Z=1.7, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MARTH_SWORD, Type=ATTACK_REGION_SWORD)
	ATTACK(ID=1, Part=0, Bone=hash40("armr"), Damage=12.5, Angle=361, KBG=94, FKB=0, BKB=40, Size=3.5, X=2.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MARTH_SWORD, Type=ATTACK_REGION_SWORD)
	ATTACK(ID=2, Part=0, Bone=hash40("sword1"), Damage=12.5, Angle=361, KBG=94, FKB=0, BKB=40, Size=3.5, X=0.0, Y=0.0, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MARTH_SWORD, Type=ATTACK_REGION_SWORD)
	}
	wait(Frames=5)
	if(is_excute){
	AttackModule::clear_all()
	}
	frame(Frame=32)
	if(is_excute){
	WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
	}

    });
}
#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_MARTH,
animation = "attack_air_hi",
animcmd = "game_attackairhi")]
pub fn instant_marth_uair(fighter: &mut L2CFighterCommon) {
    acmd!({
	if (is_excute) {
        rust{
            if CONFIG.instant_info.marth{
            MotionModule::set_rate(module_accessor, 4.0)
        }
        }
    }
	frame(Frame=3)
	if(is_excute){
	WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
	}
	frame(Frame=5)
	if(is_excute){
	MotionModule::set_rate(1.0)
	ATTACK(ID=0, Part=0, Bone=hash40("sword1"), Damage=13.0, Angle=90, KBG=84, FKB=0, BKB=40, Size=3.5, X=1.0, Y=0.0, Z=0.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MARTH_SWORD, Type=ATTACK_REGION_SWORD)
	ATTACK(ID=1, Part=0, Bone=hash40("armr"), Damage=13.0, Angle=90, KBG=84, FKB=0, BKB=40, Size=3.0, X=0.0, Y=1.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MARTH_SWORD, Type=ATTACK_REGION_SWORD)
	ATTACK(ID=2, Part=0, Bone=hash40("colonells"), Damage=13.0, Angle=90, KBG=84, FKB=0, BKB=40, Size=2.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MARTH_SWORD, Type=ATTACK_REGION_SWORD)
	ATTACK(ID=3, Part=0, Bone=hash40("sword1"), Damage=13.0, Angle=90, KBG=84, FKB=0, BKB=40, Size=3.5, X=1.0, Y=0.0, Z=6.7, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MARTH_SWORD, Type=ATTACK_REGION_SWORD)
	}
	frame(Frame=10)
	if(is_excute){
	AttackModule::clear_all()
	}
	frame(Frame=38)
	if(is_excute){
	WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
	}

    });
}
#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_MARTH,
animation = "attack_air_n",
animcmd = "game_attackairn")]
pub fn instant_marth_nair(fighter: &mut L2CFighterCommon) {
    acmd!({
	if (is_excute) {
        rust{
            if CONFIG.instant_info.marth{
            MotionModule::set_rate(module_accessor, 5.0)
        }
        }
    }
	frame(Frame=2)
	if(is_excute){
	WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
	}
	frame(Frame=6)
	if(is_excute){
	MotionModule::set_rate(1.0)
	ATTACK(ID=0, Part=0, Bone=hash40("sword1"), Damage=5.0, Angle=90, KBG=50, FKB=0, BKB=35, Size=3.8, X=1.0, Y=-1.3, Z=1.6, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MARTH_SWORD, Type=ATTACK_REGION_SWORD)
	ATTACK(ID=1, Part=0, Bone=hash40("armr"), Damage=5.0, Angle=90, KBG=50, FKB=0, BKB=35, Size=4.0, X=-1.5, Y=1.0, Z=-1.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MARTH_SWORD, Type=ATTACK_REGION_SWORD)
	ATTACK(ID=2, Part=0, Bone=hash40("sword1"), Damage=5.0, Angle=90, KBG=50, FKB=0, BKB=35, Size=3.3, X=1.0, Y=-1.3, Z=7.2, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MARTH_SWORD, Type=ATTACK_REGION_SWORD)
	}
	wait(Frames=2)
	if(is_excute){
	AttackModule::clear_all()
	MotionModule::set_rate(4.0)
	}
	frame(Frame=15)
	if(is_excute){
	MotionModule::set_rate(1.0)
	ATTACK(ID=0, Part=0, Bone=hash40("sword1"), Damage=9.5, Angle=361, KBG=100, FKB=0, BKB=60, Size=3.8, X=1.2, Y=-1.1, Z=1.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MARTH_SWORD, Type=ATTACK_REGION_SWORD)
	ATTACK(ID=1, Part=0, Bone=hash40("armr"), Damage=9.5, Angle=361, KBG=100, FKB=0, BKB=60, Size=4.0, X=-2.0, Y=1.0, Z=-1.6, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MARTH_SWORD, Type=ATTACK_REGION_SWORD)
	ATTACK(ID=2, Part=0, Bone=hash40("sword1"), Damage=9.5, Angle=361, KBG=100, FKB=0, BKB=60, Size=3.4, X=0.8, Y=-1.1, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MARTH_SWORD, Type=ATTACK_REGION_SWORD)
	}
	wait(Frames=7)
	if(is_excute){
	AttackModule::clear_all()
	}
	frame(Frame=47)
	if(is_excute){
	WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
	}

    });
}
#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_MARTH,
animation = "attack_s4_s",
animcmd = "game_attacks4")]
pub fn instant_marth_f_smash(fighter: &mut L2CFighterCommon) {
    acmd!({
	if (is_excute) {
        rust{
            if CONFIG.instant_info.marth{
            MotionModule::set_rate(module_accessor, 9.0)
        }
        }
    }
	frame(Frame=3)
	if(is_excute){
	WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
	}
	frame(Frame=10)
	if(is_excute){
	MotionModule::set_rate(1.0)
	ATTACK(ID=0, Part=0, Bone=hash40("sword1"), Damage=18.0, Angle=361, KBG=80, FKB=0, BKB=80, Size=3.5, X=1.0, Y=0.0, Z=2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MARTH_SWORD, Type=ATTACK_REGION_SWORD)
	ATTACK(ID=1, Part=0, Bone=hash40("armr"), Damage=18.0, Angle=361, KBG=80, FKB=0, BKB=80, Size=3.0, X=0.0, Y=1.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MARTH_SWORD, Type=ATTACK_REGION_SWORD)
	ATTACK(ID=2, Part=0, Bone=hash40("bust"), Damage=18.0, Angle=361, KBG=80, FKB=0, BKB=80, Size=2.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MARTH_SWORD, Type=ATTACK_REGION_SWORD)
	ATTACK(ID=3, Part=0, Bone=hash40("sword1"), Damage=18.0, Angle=361, KBG=80, FKB=0, BKB=80, Size=3.5, X=1.0, Y=0.0, Z=7.3, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MARTH_SWORD, Type=ATTACK_REGION_SWORD)
	//AttackModule::set_attack_height_all(ATTACK_HEIGHT_HIGH, false)


	}
	frame(Frame=14)
	if(is_excute){
	AttackModule::clear_all()
	}


    });
}
#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_MARTH,
animation = "attack_hi4",
animcmd = "game_attackhi4")]
pub fn instant_marth_u_smash(fighter: &mut L2CFighterCommon) {
    acmd!({
	if (is_excute) {
        rust{
            if CONFIG.instant_info.marth{
            MotionModule::set_rate(module_accessor, 12.0)
        }
        }
    }
	frame(Frame=5)
	if(is_excute){
	WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
	}
	frame(Frame=13)
	if(is_excute){
	MotionModule::set_rate(1.0)
	ATTACK(ID=1, Part=0, Bone=hash40("sword1"), Damage=17.0, Angle=90, KBG=95, FKB=0, BKB=40, Size=5.8, X=0.0, Y=0.0, Z=2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
	ATTACK(ID=2, Part=0, Bone=hash40("sword1"), Damage=17.0, Angle=90, KBG=95, FKB=0, BKB=40, Size=4.6, X=0.0, Y=0.0, Z=7.3, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MARTH_SWORD, Type=ATTACK_REGION_SWORD)
	ATTACK(ID=3, Part=0, Bone=hash40("sword1"), Damage=17.0, Angle=90, KBG=95, FKB=0, BKB=40, Size=5.8, X=0.0, Y=0.0, Z=-3.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
	ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=3.0, Angle=125, KBG=100, FKB=155, BKB=0, Size=5.5, X=0.0, Y=5.0, Z=9.0, X2=0.0, Y2=5.0, Z2=-9.0, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=2, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_SWORD)
	}
	wait(Frames=2)
	if(is_excute){
	//AttackModule::clear(ID=0)
	}
	wait(Frames=3)
	if(is_excute){
	AttackModule::clear_all()
	}


    });
}
#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_MARTH,
animation = "attack_lw4",
animcmd = "game_attacklw4")]
pub fn instant_marth_d_smash(fighter: &mut L2CFighterCommon) {
    acmd!({
	if (is_excute) {
        rust{
            if CONFIG.instant_info.marth{
            MotionModule::set_rate(module_accessor, 4.0)
        }
        }
    }
	frame(Frame=4)
	if(is_excute){
	WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
	}
	frame(Frame=6)
	if(is_excute){
	MotionModule::set_rate(1.0)
	ATTACK(ID=0, Part=0, Bone=hash40("sword1"), Damage=12.0, Angle=361, KBG=88, FKB=0, BKB=57, Size=3.5, X=1.0, Y=0.0, Z=2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MARTH_SWORD, Type=ATTACK_REGION_SWORD)
	ATTACK(ID=1, Part=0, Bone=hash40("armr"), Damage=12.0, Angle=361, KBG=88, FKB=0, BKB=57, Size=3.0, X=0.0, Y=1.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MARTH_SWORD, Type=ATTACK_REGION_SWORD)
	ATTACK(ID=2, Part=0, Bone=hash40("shoulderr"), Damage=12.0, Angle=361, KBG=88, FKB=0, BKB=57, Size=2.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MARTH_SWORD, Type=ATTACK_REGION_SWORD)
	ATTACK(ID=3, Part=0, Bone=hash40("sword1"), Damage=12.0, Angle=361, KBG=88, FKB=0, BKB=57, Size=3.5, X=1.0, Y=0.0, Z=7.3, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MARTH_SWORD, Type=ATTACK_REGION_SWORD)
	//AttackModule::set_attack_height_all(ATTACK_HEIGHT_LOW, false)
	}
	wait(Frames=2)
	if(is_excute){
	AttackModule::clear_all()
	}
	if(is_excute){
	MotionModule::set_rate(10.0)
	}

	frame(Frame=21)
	FT_MOTION_RATE(FSM=1.5)
	if(is_excute){
	MotionModule::set_rate(1.0)
	ATTACK(ID=0, Part=0, Bone=hash40("sword1"), Damage=12.0, Angle=361, KBG=88, FKB=0, BKB=40, Size=3.5, X=1.0, Y=0.0, Z=2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MARTH_SWORD, Type=ATTACK_REGION_SWORD)
	ATTACK(ID=1, Part=0, Bone=hash40("armr"), Damage=12.0, Angle=361, KBG=88, FKB=0, BKB=40, Size=3.0, X=0.0, Y=1.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MARTH_SWORD, Type=ATTACK_REGION_SWORD)
	ATTACK(ID=2, Part=0, Bone=hash40("shoulderr"), Damage=12.0, Angle=361, KBG=88, FKB=0, BKB=40, Size=2.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MARTH_SWORD, Type=ATTACK_REGION_SWORD)
	ATTACK(ID=3, Part=0, Bone=hash40("sword1"), Damage=17.0, Angle=361, KBG=92, FKB=0, BKB=50, Size=3.5, X=0.5, Y=0.0, Z=7.3, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MARTH_SWORD, Type=ATTACK_REGION_SWORD)
	//AttackModule::set_attack_height_all(ATTACK_HEIGHT_LOW, false)
	}
	wait(Frames=2)
	FT_MOTION_RATE(FSM=1)
	if(is_excute){
	AttackModule::clear_all()
	}


    });
}
/*
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
    battle_object_kind = FIGHTER_KIND_MARTH,
    animation = "special_n_end_max",
    animcmd = "game_specialnendmax")]
pub fn instant_marth_n_special_max(fighter: &mut L2CFighterCommon) {
    acmd!({
	if(is_excute){
	MotionModule::set_rate(7.0)
	}
	frame(Frame=8)
	if(is_excute){
	MotionModule::set_rate(1.0)
	ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=24.0, Angle=361, KBG=100, FKB=0, BKB=60, Size=2.5, X=0.0, Y=8.5, Z=8.0, X2=0.0, Y2=8.5, Z2=20.0, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=50, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_marth_shield_breaker"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MARTH_SWORD, Type=ATTACK_REGION_SWORD)
	ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=24.0, Angle=361, KBG=100, FKB=0, BKB=60, Size=2.5, X=0.0, Y=8.5, Z=25.0, X2=0.0, Y2=8.5, Z2=27.0, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=50, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_marth_shield_breaker"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MARTH_SWORD, Type=ATTACK_REGION_SWORD)
	ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=24.0, Angle=361, KBG=100, FKB=0, BKB=60, Size=0.7, X=0.0, Y=7.5, Z=17.0, X2=0.0, Y2=7.5, Z2=21.9, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=50, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_HEAD, FriendlyFire=false, Effect=hash40("collision_attr_marth_shield_breaker"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MARTH_SWORD, Type=ATTACK_REGION_SWORD)
	ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=24.0, Angle=361, KBG=100, FKB=0, BKB=60, Size=0.7, X=0.0, Y=7.5, Z=23.5, X2=0.0, Y2=7.5, Z2=28.6, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=50, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_HEAD, FriendlyFire=false, Effect=hash40("collision_attr_marth_shield_breaker"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MARTH_SWORD, Type=ATTACK_REGION_SWORD)
	}

	frame(Frame=10)
	if(is_excute){
	AttackModule::clear_all()
	}


    });
}
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
    battle_object_kind = FIGHTER_KIND_MARTH,
    animation = "special_n_e",
    animcmd = "game_specialnend")]
pub fn instant_marth_n_special_end(fighter: &mut L2CFighterCommon) {
    acmd!({
	if(is_excute){
	MotionModule::set_rate(7.0)
	}
	frame(Frame=8)
	if(is_excute){
	MotionModule::set_rate(1.0)
	ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=24.0, Angle=361, KBG=100, FKB=0, BKB=60, Size=2.5, X=0.0, Y=8.5, Z=8.0, X2=0.0, Y2=8.5, Z2=20.0, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=50, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_marth_shield_breaker"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MARTH_SWORD, Type=ATTACK_REGION_SWORD)
	ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=24.0, Angle=361, KBG=100, FKB=0, BKB=60, Size=2.5, X=0.0, Y=8.5, Z=25.0, X2=0.0, Y2=8.5, Z2=27.0, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=50, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_marth_shield_breaker"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MARTH_SWORD, Type=ATTACK_REGION_SWORD)
	ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=24.0, Angle=361, KBG=100, FKB=0, BKB=60, Size=0.7, X=0.0, Y=7.5, Z=17.0, X2=0.0, Y2=7.5, Z2=21.9, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=50, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_HEAD, FriendlyFire=false, Effect=hash40("collision_attr_marth_shield_breaker"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MARTH_SWORD, Type=ATTACK_REGION_SWORD)
	ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=24.0, Angle=361, KBG=100, FKB=0, BKB=60, Size=0.7, X=0.0, Y=7.5, Z=23.5, X2=0.0, Y2=7.5, Z2=28.6, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=50, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_HEAD, FriendlyFire=false, Effect=hash40("collision_attr_marth_shield_breaker"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MARTH_SWORD, Type=ATTACK_REGION_SWORD)
	}

	frame(Frame=10)
	if(is_excute){
	AttackModule::clear_all()
	}


    });
}
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
    battle_object_kind = FIGHTER_KIND_MARTH,
    animation = "special_n_end_hi",
    animcmd = "game_specialnendhi")]
pub fn instant_marth_n_special_endhi(fighter: &mut L2CFighterCommon) {
    acmd!({
	if(is_excute){
	MotionModule::set_rate(7.0)
	}
	frame(Frame=8)
	if(is_excute){
	MotionModule::set_rate(1.0)
	ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=24.0, Angle=361, KBG=100, FKB=0, BKB=60, Size=2.5, X=0.0, Y=8.5, Z=8.0, X2=0.0, Y2=8.5, Z2=20.0, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=50, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_marth_shield_breaker"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MARTH_SWORD, Type=ATTACK_REGION_SWORD)
	ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=24.0, Angle=361, KBG=100, FKB=0, BKB=60, Size=2.5, X=0.0, Y=14.0, Z=25.0, X2=0.0, Y2=8.5, Z2=27.0, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=50, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_marth_shield_breaker"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MARTH_SWORD, Type=ATTACK_REGION_SWORD)
	ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=24.0, Angle=361, KBG=100, FKB=0, BKB=60, Size=0.7, X=0.0, Y=9.8, Z=17.0, X2=0.0, Y2=7.5, Z2=21.9, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=50, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_HEAD, FriendlyFire=false, Effect=hash40("collision_attr_marth_shield_breaker"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MARTH_SWORD, Type=ATTACK_REGION_SWORD)
	ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=24.0, Angle=361, KBG=100, FKB=0, BKB=60, Size=0.7, X=0.0, Y=13.0, Z=23.5, X2=0.0, Y2=7.5, Z2=28.6, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=50, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_HEAD, FriendlyFire=false, Effect=hash40("collision_attr_marth_shield_breaker"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MARTH_SWORD, Type=ATTACK_REGION_SWORD)
	}

	frame(Frame=10)
	if(is_excute){
	AttackModule::clear_all()
	}


    });
}
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
    battle_object_kind = FIGHTER_KIND_MARTH,
    animation = "special_n_end_lw",
    animcmd = "game_specialnendlw")]
pub fn instant_marth_n_special_endlw(fighter: &mut L2CFighterCommon) {
    acmd!({
	if(is_excute){
	MotionModule::set_rate(7.0)
	}
	frame(Frame=8)
	if(is_excute){
	MotionModule::set_rate(1.0)
	ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=24.0, Angle=361, KBG=100, FKB=0, BKB=60, Size=2.5, X=0.0, Y=6.5, Z=8.0, X2=0.0, Y2=8.5, Z2=20.0, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=50, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_marth_shield_breaker"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MARTH_SWORD, Type=ATTACK_REGION_SWORD)
	ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=24.0, Angle=361, KBG=100, FKB=0, BKB=60, Size=2.5, X=0.0, Y=3.0, Z=25.0, X2=0.0, Y2=8.5, Z2=27.0, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=50, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_marth_shield_breaker"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MARTH_SWORD, Type=ATTACK_REGION_SWORD)
	ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=24.0, Angle=361, KBG=100, FKB=0, BKB=60, Size=0.7, X=0.0, Y=5.6, Z=17.0, X2=0.0, Y2=7.5, Z2=21.9, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=50, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_HEAD, FriendlyFire=false, Effect=hash40("collision_attr_marth_shield_breaker"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MARTH_SWORD, Type=ATTACK_REGION_SWORD)
	ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=24.0, Angle=361, KBG=100, FKB=0, BKB=60, Size=0.7, X=0.0, Y=3.0, Z=23.5, X2=0.0, Y2=7.5, Z2=28.6, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=50, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_HEAD, FriendlyFire=false, Effect=hash40("collision_attr_marth_shield_breaker"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MARTH_SWORD, Type=ATTACK_REGION_SWORD)
	}

	frame(Frame=10)
	if(is_excute){
	AttackModule::clear_all()
	}



    });
}
*/
#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_MARTH,
animation = "attack_lw3",
animcmd = "game_attacklw3")]
pub fn marth_instant_d_tilt(fighter: &mut L2CFighterCommon) {
    acmd!({
	if (is_excute) {
        rust{
            if CONFIG.instant_info.marth{
            MotionModule::set_rate(module_accessor, 6.0)
        }
        }
    }
	frame(Frame=7)
	if(is_excute){
	MotionModule::set_rate(1.0)
	ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=11.0, Angle=30, KBG=40, FKB=0, BKB=50, Size=2.7, X=0.0, Y=3.5, Z=13.0, X2=0.0, Y2=4.1, Z2=9.2, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.35, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_ROY_HIT, Type=ATTACK_REGION_SWORD)
	ATTACK(ID=1, Part=0, Bone=hash40("sword1"), Damage=11.0, Angle=30, KBG=40, FKB=0, BKB=50, Size=2.7, X=0.0, Y=0.0, Z=6.7, X2=0.0, Y2=0.0, Z2=4.7, Hitlag=0.7, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.35, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_ROY_HIT, Type=ATTACK_REGION_SWORD)
	//AttackModule::set_attack_height_all(ATTACK_HEIGHT_LOW, false)
	}
	wait(Frames=2)
	if(is_excute){
	AttackModule::clear_all()
	}
    });
}
#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_MARTH,
animation = "attack_hi3",
animcmd = "game_attackhi3")]
pub fn marth_instant_u_tilt(fighter: &mut L2CFighterCommon) {
    acmd!({
	if (is_excute) {
        rust{
            if CONFIG.instant_info.marth{
            MotionModule::set_rate(module_accessor, 5.0)
        }
        }
    }
	frame(Frame=6)
	if(is_excute){
	MotionModule::set_rate(1.0)
	ATTACK(ID=0, Part=0, Bone=hash40("sword1"), Damage=9.0, Angle=100, KBG=100, FKB=0, BKB=65, Size=3.5, X=0.0, Y=0.0, Z=2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MARTH_SWORD, Type=ATTACK_REGION_SWORD)
	ATTACK(ID=1, Part=0, Bone=hash40("armr"), Damage=9.0, Angle=100, KBG=100, FKB=0, BKB=65, Size=3.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MARTH_SWORD, Type=ATTACK_REGION_SWORD)
	ATTACK(ID=2, Part=0, Bone=hash40("colonells"), Damage=9.0, Angle=100, KBG=100, FKB=0, BKB=65, Size=2.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MARTH_SWORD, Type=ATTACK_REGION_SWORD)
	ATTACK(ID=3, Part=0, Bone=hash40("sword1"), Damage=9.0, Angle=100, KBG=100, FKB=0, BKB=65, Size=3.5, X=0.0, Y=0.0, Z=6.7, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MARTH_SWORD, Type=ATTACK_REGION_SWORD)
	ATTACK(ID=4, Part=0, Bone=hash40("sword1"), Damage=9.0, Angle=100, KBG=100, FKB=0, BKB=65, Size=3.5, X=0.0, Y=0.0, Z=2.0, X2=4.5, Y2=0.0, Z2=2.0, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MARTH_SWORD, Type=ATTACK_REGION_SWORD)
	ATTACK(ID=5, Part=0, Bone=hash40("sword1"), Damage=9.0, Angle=100, KBG=100, FKB=0, BKB=65, Size=3.5, X=0.0, Y=0.0, Z=6.7, X2=6.0, Y2=0.0, Z2=6.7, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MARTH_SWORD, Type=ATTACK_REGION_SWORD)
	}
	wait(Frames=1)
	if(is_excute){
	//AttackModule::clear(ID=4)
	//AttackModule::clear(ID=5)
	}
	wait(Frames=2)
	if(is_excute){
	ATTACK(ID=0, Part=0, Bone=hash40("sword1"), Damage=9.0, Angle=85, KBG=100, FKB=0, BKB=52, Size=3.5, X=0.0, Y=0.0, Z=2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MARTH_SWORD, Type=ATTACK_REGION_SWORD)
	ATTACK(ID=1, Part=0, Bone=hash40("armr"), Damage=9.0, Angle=85, KBG=100, FKB=0, BKB=52, Size=3.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MARTH_SWORD, Type=ATTACK_REGION_SWORD)
	ATTACK(ID=2, Part=0, Bone=hash40("colonells"), Damage=9.0, Angle=85, KBG=100, FKB=0, BKB=52, Size=2.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MARTH_SWORD, Type=ATTACK_REGION_SWORD)
	ATTACK(ID=3, Part=0, Bone=hash40("sword1"), Damage=9.0, Angle=85, KBG=100, FKB=0, BKB=52, Size=3.5, X=0.0, Y=0.0, Z=6.7, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MARTH_SWORD, Type=ATTACK_REGION_SWORD)
	}
	wait(Frames=4)
	if(is_excute){
	AttackModule::clear_all()
	}

    });
}
#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_MARTH,
animation = "attack_s3_s",
animcmd = "game_attacks3")]
pub fn marth_instant_s_tilt(fighter: &mut L2CFighterCommon) {
    acmd!({
	if (is_excute) {
        rust{
            if CONFIG.instant_info.marth{
            MotionModule::set_rate(module_accessor, 7.0)
        }
        }
    }
	frame(Frame=8)
	if(is_excute){
	MotionModule::set_rate(1.0)
	ATTACK(ID=0, Part=0, Bone=hash40("sword1"), Damage=12.0, Angle=361, KBG=85, FKB=0, BKB=55, Size=3.5, X=1.0, Y=0.0, Z=2.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MARTH_SWORD, Type=ATTACK_REGION_SWORD)
	ATTACK(ID=1, Part=0, Bone=hash40("armr"), Damage=12.0, Angle=361, KBG=85, FKB=0, BKB=55, Size=3.0, X=0.0, Y=1.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MARTH_SWORD, Type=ATTACK_REGION_SWORD)
	ATTACK(ID=2, Part=0, Bone=hash40("sword1"), Damage=12.0, Angle=361, KBG=85, FKB=0, BKB=55, Size=3.5, X=1.0, Y=0.0, Z=7.7, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MARTH_SWORD, Type=ATTACK_REGION_SWORD)
	}
	wait(Frames=4)
	if(is_excute){
	AttackModule::clear_all()
	}

    });
}
pub fn install() {
    acmd::add_hooks!(
    instant_marth_fair,
    instant_marth_dair,
	instant_marth_uair,
	instant_marth_bair,
	instant_marth_nair,
	instant_marth_f_smash,
	instant_marth_u_smash,
	instant_marth_d_smash,
	marth_instant_s_tilt,
	marth_instant_u_tilt,
	marth_instant_d_tilt,
	marth_grab,
    marth_grab_dash,
    marth_grab_turn
	//instant_marth_n_special_max,
	//instant_marth_n_special_end,
	//instant_marth_n_special_endhi,
	//instant_marth_n_special_endlw

    );
}

