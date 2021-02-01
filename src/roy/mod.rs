use smash::hash40;
use smash::lib::lua_const::*;
use smash::lua2cpp::L2CFighterCommon;
use acmd::{acmd, acmd_func};
use crate::config::CONFIG;
use smash::app::lua_bind::*;

#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_ROY,
animation = "attack_air_f",
animcmd = "game_attackairf")]
pub fn roy_instant_fire_fair(fighter: &mut L2CFighterCommon) {
    acmd!({
	if (is_excute) {
        rust{
            if CONFIG.instant_info.roy{
            MotionModule::set_rate(module_accessor, 12.0)
        }
        }
      WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
    }
	frame(Frame=13)
	if(is_excute){
	    MotionModule::set_rate(1.0)
	    ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=13.0, Angle=42, KBG=83, FKB=0, BKB=50, Size=4.8, X=0.0, Y=13.0, Z=4.0, X2=0.0, Y2=10.0, Z2=4.0, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ROY_HIT, Type=ATTACK_REGION_SWORD)
	    ATTACK(ID=1, Part=0, Bone=hash40("armr"), Damage=13.0, Angle=42, KBG=83, FKB=0, BKB=50, Size=4.2, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ROY_HIT, Type=ATTACK_REGION_SWORD)
	    ATTACK(ID=2, Part=0, Bone=hash40("sword1"), Damage=13.0, Angle=42, KBG=83, FKB=0, BKB=50, Size=3.5, X=0.0, Y=0.0, Z=0.7, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ROY_HIT, Type=ATTACK_REGION_SWORD)
	    ATTACK(ID=3, Part=0, Bone=hash40("sword1"), Damage=13.0, Angle=42, KBG=83, FKB=0, BKB=50, Size=3.5, X=0.0, Y=0.0, Z=3.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ROY_HIT, Type=ATTACK_REGION_SWORD)
	    ATTACK(ID=4, Part=0, Bone=hash40("sword1"), Damage=13.0, Angle=42, KBG=83, FKB=0, BKB=50, Size=3.5, X=0.0, Y=0.0, Z=5.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ROY_HIT, Type=ATTACK_REGION_SWORD)
	    ATTACK(ID=5, Part=0, Bone=hash40("sword1"), Damage=13.0, Angle=42, KBG=83, FKB=0, BKB=50, Size=3.5, X=0.0, Y=0.0, Z=7.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ROY_HIT, Type=ATTACK_REGION_SWORD)
	    ATTACK(ID=6, Part=0, Bone=hash40("sword1"), Damage=13.0, Angle=42, KBG=83, FKB=0, BKB=50, Size=3.5, X=0.0, Y=0.0, Z=9.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ROY_HIT, Type=ATTACK_REGION_SWORD)

	}
	wait(Frames=3)
	if(is_excute){
	    AttackModule::clear_all()
	}
	frame(Frame=34)
	if(is_excute){
	    WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
	}

    });
}
#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_ROY,
animation = "attack_air_hi",
animcmd = "game_attackairhi")]
pub fn roy_instant_fire_uair(fighter: &mut L2CFighterCommon) {
    acmd!({
    if (is_excute) {
        rust{
            if CONFIG.instant_info.roy{
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
        ATTACK(ID=0, Part=0, Bone=hash40("colonells"), Damage=11.0, Angle=80, KBG=105, FKB=0, BKB=40, Size=3.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ROY_HIT, Type=ATTACK_REGION_SWORD)
	    ATTACK(ID=1, Part=0, Bone=hash40("armr"), Damage=11.0, Angle=80, KBG=105, FKB=0, BKB=40, Size=4.2, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ROY_HIT, Type=ATTACK_REGION_SWORD)
	    ATTACK(ID=2, Part=0, Bone=hash40("sword1"), Damage=11.0, Angle=80, KBG=105, FKB=0, BKB=40, Size=3.0, X=0.0, Y=0.0, Z=1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ROY_HIT, Type=ATTACK_REGION_SWORD)
	    ATTACK(ID=3, Part=0, Bone=hash40("sword1"), Damage=11.0, Angle=80, KBG=105, FKB=0, BKB=40, Size=3.9, X=0.0, Y=0.0, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.7, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ROY_HIT, Type=ATTACK_REGION_SWORD)
	}
	frame(Frame=13)
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
battle_object_kind = FIGHTER_KIND_ROY,
animation = "attack_air_b",
animcmd = "game_attackairb")]
pub fn roy_instant_fire_bair(fighter: &mut L2CFighterCommon) {
    acmd!({
    if (is_excute) {
        rust{
            if CONFIG.instant_info.roy{
            MotionModule::set_rate(module_accessor, 7.0)
        }
        }
    }
    frame(Frame=3)
	if(is_excute){
	WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
	REVERSE_LR()
	}
	frame(Frame=8)
	if(is_excute){
	MotionModule::set_rate(1.0)
	ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=15.0, Angle=361, KBG=107, FKB=0, BKB=35, Size=4.2, X=0.0, Y=10.5, Z=-7.0, X2=0.0, Y2=10.5, Z2=-4.5, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ROY_HIT, Type=ATTACK_REGION_SWORD)
	ATTACK(ID=1, Part=0, Bone=hash40("sword1"), Damage=15.0, Angle=361, KBG=107, FKB=0, BKB=35, Size=4.0, X=2.5, Y=0.0, Z=0.4, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ROY_HIT, Type=ATTACK_REGION_SWORD)
	ATTACK(ID=2, Part=0, Bone=hash40("sword1"), Damage=15.0, Angle=361, KBG=107, FKB=0, BKB=35, Size=4.0, X=2.5, Y=0.0, Z=7.2, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ROY_HIT, Type=ATTACK_REGION_SWORD)
	}
	wait(Frames=3)
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
battle_object_kind = FIGHTER_KIND_ROY,
animation = "attack_s4_s",
animcmd = "game_attacks4")]
pub fn roy_instant_fire_s4(fighter: &mut L2CFighterCommon) {
    acmd!({
	if (is_excute) {
        rust{
            if CONFIG.instant_info.roy{
            MotionModule::set_rate(module_accessor, 13.0)
        }
        }
    }
	frame(Frame=7)
	if(is_excute){
	WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
	}
	frame(Frame=14)
	if(is_excute){
	MotionModule::set_rate(1.0)
	ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=23.0, Angle=361, KBG=75, FKB=0, BKB=60, Size=3.3, X=0.0, Y=7.7, Z=9.1, X2=0.0, Y2=7.7, Z2=7.0, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ROY_HIT, Type=ATTACK_REGION_SWORD)
	ATTACK(ID=1, Part=0, Bone=hash40("sword1"), Damage=23.0, Angle=361, KBG=75, FKB=0, BKB=60, Size=4.0, X=2.0, Y=1.0, Z=1.5, X2=14.0, Y2=1.0, Z2=1.5, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ROY_HIT, Type=ATTACK_REGION_SWORD)
	ATTACK(ID=2, Part=0, Bone=hash40("sword1"), Damage=23.0, Angle=361, KBG=75, FKB=0, BKB=60, Size=3.5, X=2.0, Y=1.0, Z=7.5, X2=9.5, Y2=1.0, Z2=7.5, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ROY_HIT, Type=ATTACK_REGION_SWORD)
	//AttackModule::set_attack_height_all(ATTACK_HEIGHT_HIGH, false)
	}
	frame(Frame=16)
	if(is_excute){
	AttackModule::clear_all()
	}

    });
}
#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_ROY,
animation = "attack_air_n",
animcmd = "game_attackairn")]
pub fn roy_instant_fire_nair(fighter: &mut L2CFighterCommon) {
    acmd!({
	if (is_excute) {
        rust{
            if CONFIG.instant_info.roy{
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
	ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=8.0, Angle=80, KBG=30, FKB=0, BKB=32, Size=3.5, X=0.0, Y=9.6, Z=8.0, X2=0.0, Y2=11.7, Z2=3.0, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_ROY_HIT, Type=ATTACK_REGION_SWORD)
	ATTACK(ID=1, Part=0, Bone=hash40("sword1"), Damage=8.0, Angle=80, KBG=30, FKB=0, BKB=32, Size=3.5, X=0.0, Y=0.0, Z=6.8, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_ROY_HIT, Type=ATTACK_REGION_SWORD)
	ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=8.0, Angle=80, KBG=30, FKB=0, BKB=32, Size=4.8, X=0.0, Y=9.6, Z=6.8, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_ROY_HIT, Type=ATTACK_REGION_SWORD)
	}
	wait(Frames=2)
	if(is_excute){
	AttackModule::clear_all()
	MotionModule::set_rate(4.0)
	}

	frame(Frame=15)
	if(is_excute){
	MotionModule::set_rate(1.0)
	ATTACK(ID=0, Part=0, Bone=hash40("armr"), Damage=11.0, Angle=50, KBG=105, FKB=0, BKB=50, Size=5.6, X=0.0, Y=0.0, Z=-1.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_ROY_HIT, Type=ATTACK_REGION_SWORD)
	ATTACK(ID=1, Part=0, Bone=hash40("sword1"), Damage=11.0, Angle=50, KBG=105, FKB=0, BKB=50, Size=4.2, X=2.5, Y=0.0, Z=0.7, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_ROY_HIT, Type=ATTACK_REGION_SWORD)
	ATTACK(ID=2, Part=0, Bone=hash40("sword1"), Damage=11.0, Angle=50, KBG=100, FKB=0, BKB=50, Size=4.2, X=2.5, Y=0.0, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.7, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_ROY_HIT, Type=ATTACK_REGION_SWORD)
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
battle_object_kind = FIGHTER_KIND_ROY,
animation = "attack_air_lw",
animcmd = "game_attackairlw")]
pub fn roy_instant_fire_dair(fighter: &mut L2CFighterCommon) {
    acmd!({
	if (is_excute) {
        rust{
            if CONFIG.instant_info.roy{
            MotionModule::set_rate(module_accessor, 15.0)
        }
        }
    }
	frame(Frame=16)
	if(is_excute){
	MotionModule::set_rate(1.0)
	ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=15.0, Angle=270, KBG=90, FKB=0, BKB=20, Size=4.4, X=0.0, Y=-0.7, Z=0.4, X2=0.0, Y2=-2.5, Z2=0.4, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_SWORD)
	ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=15.0, Angle=270, KBG=90, FKB=0, BKB=20, Size=4.4, X=0.0, Y=-0.7, Z=0.4, X2=0.0, Y2=-2.5, Z2=0.4, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_SWORD)
	ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=15.0, Angle=270, KBG=90, FKB=0, BKB=20, Size=5.4, X=0.0, Y=-6.8, Z=0.4, X2=0.0, Y2=-1.0, Z2=0.4, Hitlag=0.7, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_SWORD)
	}
	frame(Frame=18)
	if(is_excute){
	AttackModule::clear_all()
	}
	frame(Frame=52)
	if(is_excute){
	WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
	}
    });
}
#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_ROY,
animation = "attack_hi3",
animcmd = "game_attackhi3")]
pub fn roy_instant_fire_u_tilt(fighter: &mut L2CFighterCommon) {
    acmd!({
	if (is_excute) {
        rust{
            if CONFIG.instant_info.roy{
            MotionModule::set_rate(module_accessor, 5.0)
        }
        }
    }
	frame(Frame=6)
	if(is_excute){
	MotionModule::set_rate(1.0)
	ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=14.0, Angle=98, KBG=103, FKB=0, BKB=35, Size=4.2, X=0.0, Y=16.0, Z=0.0, X2=0.0, Y2=16.0, Z2=0.0, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ROY_HIT, Type=ATTACK_REGION_SWORD)
	ATTACK(ID=1, Part=0, Bone=hash40("sword1"), Damage=14.0, Angle=98, KBG=103, FKB=0, BKB=35, Size=2.8, X=0.0, Y=0.0, Z=1.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ROY_HIT, Type=ATTACK_REGION_SWORD)
	ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=14.0, Angle=98, KBG=103, FKB=0, BKB=35, Size=1.5, X=0.0, Y=18.0, Z=6.0, X2=0.0, Y2=10.0, Z2=6.0, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ROY_HIT, Type=ATTACK_REGION_SWORD)
	ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=14.0, Angle=98, KBG=103, FKB=0, BKB=35, Size=2.0, X=0.0, Y=16.0, Z=10.0, X2=0.0, Y2=10.0, Z2=10.0, Hitlag=0.7, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ROY_HIT, Type=ATTACK_REGION_SWORD)
	}
	wait(Frames=1)
	if(is_excute){
	ATTACK(ID=2, Part=0, Bone=hash40("sword1"), Damage=14.0, Angle=98, KBG=103, FKB=0, BKB=35, Size=3.5, X=0.0, Y=0.0, Z=7.2, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.7, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ROY_HIT, Type=ATTACK_REGION_SWORD)
	AttackModule::clear(ID=3,false)
	}
	frame(Frame=9)
	if(is_excute){
	AttackModule::clear(ID=0,false)
	}
	frame(Frame=12)
	if(is_excute){
	AttackModule::clear_all()
	}
    });
}
#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_ROY,
animation = "attack_lw3",
animcmd = "game_attacklw3")]
pub fn roy_instant_fire_d_tilt(fighter: &mut L2CFighterCommon) {
    acmd!({
	if (is_excute) {
        rust{
            if CONFIG.instant_info.roy{
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
battle_object_kind = FIGHTER_KIND_ROY,
animation = "attack_s3_s",
animcmd = "game_attacks3")]
pub fn roy_instant_fire_s_tilt(fighter: &mut L2CFighterCommon) {
    acmd!({
	//frame(Frame=6)
	/*
	if(is_excute){
	FighterAreaModuleImpl::enable_fix_jostle_area(4, 4)
	}
	*/
	if(is_execute){
	MotionModule::set_rate(7.0)
	}
	frame(Frame=8)
	if(is_excute){
	MotionModule::set_rate(1.0)
	ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=14.0, Angle=361, KBG=105, FKB=0, BKB=30, Size=4.3, X=0.0, Y=9.0, Z=6.2, X2=0.0, Y2=9.0, Z2=3.0, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ROY_HIT, Type=ATTACK_REGION_SWORD)
	ATTACK(ID=1, Part=0, Bone=hash40("sword1"), Damage=14.0, Angle=361, KBG=105, FKB=0, BKB=30, Size=4.1, X=0.0, Y=0.0, Z=1.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ROY_HIT, Type=ATTACK_REGION_SWORD)
	ATTACK(ID=2, Part=0, Bone=hash40("sword1"), Damage=14.0, Angle=361, KBG=105, FKB=0, BKB=30, Size=3.5, X=0.0, Y=0.0, Z=7.2, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.7, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ROY_HIT, Type=ATTACK_REGION_SWORD)
	//AttackModule::set_attack_height_all(ATTACK_HEIGHT_HIGH, false)
	}
	wait(Frames=3)
	if(is_excute){
	AttackModule::clear_all()
	}
    });
}
#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_ROY,
animation = "attack_lw4",
animcmd = "game_attacklw4")]
pub fn roy_instant_fire_d_smash(fighter: &mut L2CFighterCommon) {
    acmd!({
	if (is_excute) {
        rust{
            if CONFIG.instant_info.roy{
            MotionModule::set_rate(module_accessor, 4.0)
        }
        }
    }
	frame(Frame=4)
	if(is_excute){
	MotionModule::set_rate(1.0)
	WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
	}
	frame(Frame=6)
	if(is_excute){
	//MotionModule::set_rate(1.0)
	ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=15.0, Angle=50, KBG=85, FKB=0, BKB=42, Size=3.3, X=0.0, Y=4.5, Z=13.0, X2=0.0, Y2=6.0, Z2=7.0, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ROY_HIT, Type=ATTACK_REGION_SWORD)
	ATTACK(ID=1, Part=0, Bone=hash40("sword1"), Damage=15.0, Angle=50, KBG=85, FKB=0, BKB=42, Size=3.5, X=0.0, Y=0.0, Z=2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.7, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ROY_HIT, Type=ATTACK_REGION_SWORD)
	ATTACK(ID=2, Part=0, Bone=hash40("sword1"), Damage=15.0, Angle=50, KBG=85, FKB=0, BKB=42, Size=2.8, X=0.0, Y=0.0, Z=7.2, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.7, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ROY_HIT, Type=ATTACK_REGION_SWORD)
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
	if(is_excute){
	MotionModule::set_rate(1.0)
	ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=17.0, Angle=47, KBG=85, FKB=0, BKB=42, Size=3.3, X=0.0, Y=4.1, Z=-11.0, X2=0.0, Y2=4.5, Z2=-8.0, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ROY_HIT, Type=ATTACK_REGION_SWORD)
	ATTACK(ID=1, Part=0, Bone=hash40("sword1"), Damage=17.0, Angle=47, KBG=85, FKB=0, BKB=42, Size=3.5, X=0.0, Y=0.0, Z=2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.7, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ROY_HIT, Type=ATTACK_REGION_SWORD)
	ATTACK(ID=2, Part=0, Bone=hash40("sword1"), Damage=17.0, Angle=47, KBG=85, FKB=0, BKB=42, Size=2.8, X=-1.6, Y=0.0, Z=7.2, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.7, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ROY_HIT, Type=ATTACK_REGION_SWORD)
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
battle_object_kind = FIGHTER_KIND_ROY,
animation = "attack_11",
animcmd = "game_attack11")]
pub fn roy_instant_fire_jab(fighter: &mut L2CFighterCommon) {
    acmd!({
	if (is_excute) {
        rust{
            if CONFIG.instant_info.roy{
            MotionModule::set_rate(module_accessor, 4.0)
        }
        }
    }
	frame(Frame=5)
	if(is_excute){
	//FighterAreaModuleImpl::enable_fix_jostle_area(5, 5)
	ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=9.0, Angle=69, KBG=40, FKB=0, BKB=55, Size=3.7, X=0.0, Y=10.0, Z=7.0, X2=0.0, Y2=10.0, Z2=5.5, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_ROY_HIT, Type=ATTACK_REGION_SWORD)
	ATTACK(ID=1, Part=0, Bone=hash40("sword1"), Damage=9.0, Angle=69, KBG=40, FKB=0, BKB=55, Size=3.8, X=0.0, Y=0.0, Z=1.7, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_ROY_HIT, Type=ATTACK_REGION_SWORD)
	ATTACK(ID=2, Part=0, Bone=hash40("sword1"), Damage=9.0, Angle=69, KBG=40, FKB=0, BKB=55, Size=3.5, X=0.0, Y=0.0, Z=8.4, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_ROY_HIT, Type=ATTACK_REGION_SWORD)
	}
	frame(Frame=8)
	if(is_excute){
	AttackModule::clear_all()
	}

    });
}
#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_ROY,
animation = "special_lw",
animcmd = "game_speciallw")]
pub fn roy_instant_fire_d_special(fighter: &mut L2CFighterCommon) {
    acmd!({
	if (is_excute) {
        rust{
            if CONFIG.instant_info.roy{
            MotionModule::set_rate(module_accessor, 27.0)
        }
        }
    }
	frame(Frame=8)
	if(is_excute){
	    WorkModule::on_flag(Flag=FIGHTER_ROY_STATUS_SPECIAL_LW_FLAG_SHIELD)
	    WorkModule::on_flag(Flag=FIGHTER_ROY_STATUS_SPECIAL_LW_FLAG_APPLY_POWERUP_MOTION_RATE)
	}
	frame(Frame=30)
	if(is_excute){
	    WorkModule::off_flag(Flag=FIGHTER_ROY_STATUS_SPECIAL_LW_FLAG_SHIELD)
	    WorkModule::off_flag(Flag=FIGHTER_ROY_STATUS_SPECIAL_LW_FLAG_APPLY_POWERUP_MOTION_RATE)
	}

    });
}
#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_ROY,
animation = "special_n_end_max",
animcmd = "game_specialnendmax")]
pub fn roy_instant_fire_n_special_max(fighter: &mut L2CFighterCommon) {
    acmd!({
	if (is_excute) {
        rust{
            if CONFIG.instant_info.roy{
            MotionModule::set_rate(module_accessor, 9.0)
        }
        }
        KineticModule::set_consider_ground_friction(false, KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)

    }
	frame(Frame=10)
	if(is_excute){
	MotionModule::set_rate(1.0)
	ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=50.0, Angle=361, KBG=83, FKB=0, BKB=50, Size=9.6, X=0.0, Y=10.5, Z=16.0, X2=0.0, Y2=10.5, Z2=15.7, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=30, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_BOMB)
	ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=50.0, Angle=361, KBG=83, FKB=0, BKB=50, Size=17.0, X=0.0, Y=10.5, Z=16.0, X2=0.0, Y2=10.5, Z2=15.7, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=30, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_BOMB)
	}
	frame(Frame=15)
	if(is_excute){
	AttackModule::clear_all()
	}


    });
}

#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_ROY,
animation = "special_n_end",
animcmd = "game_specialnend")]
pub fn roy_instant_fire_n_special(fighter: &mut L2CFighterCommon) {
    acmd!({
if (is_excute) {
        rust{
            if CONFIG.instant_info.roy{
            MotionModule::set_rate(module_accessor, 9.0)
        }
        }
        KineticModule::set_consider_ground_friction(false, KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)

    }
	frame(Frame=10)
	if(is_excute){
	MotionModule::set_rate(1.0)
	ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=50.0, Angle=361, KBG=83, FKB=0, BKB=50, Size=9.6, X=0.0, Y=10.5, Z=16.0, X2=0.0, Y2=10.5, Z2=15.7, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=30, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_BOMB)
	ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=50.0, Angle=361, KBG=83, FKB=0, BKB=50, Size=17.0, X=0.0, Y=10.5, Z=16.0, X2=0.0, Y2=10.5, Z2=15.7, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=30, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_BOMB)
	}
	frame(Frame=15)
	if(is_excute){
	AttackModule::clear_all()
	}


    });
}
#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_ROY,
animation = "special_n_end_1",
animcmd = "game_specialnend1")]
pub fn roy_instant_fire_n_special1(fighter: &mut L2CFighterCommon) {
    acmd!({
    if (is_excute) {
        rust{
            if CONFIG.instant_info.roy{
            MotionModule::set_rate(module_accessor, 9.0)
        }
        }
        KineticModule::set_consider_ground_friction(false, KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)

    }
	frame(Frame=10)
	if(is_excute){
	MotionModule::set_rate(1.0)
	ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=50.0, Angle=361, KBG=83, FKB=0, BKB=50, Size=9.6, X=0.0, Y=10.5, Z=16.0, X2=0.0, Y2=10.5, Z2=15.7, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=30, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_BOMB)
	ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=50.0, Angle=361, KBG=83, FKB=0, BKB=50, Size=17.0, X=0.0, Y=10.5, Z=16.0, X2=0.0, Y2=10.5, Z2=15.7, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=30, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_BOMB)
	}
	frame(Frame=15)
	if(is_excute){
	AttackModule::clear_all()
	}


    });
}
#[acmd_func(
battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
battle_object_kind = FIGHTER_KIND_ROY,
animation = "special_n_end_2",
animcmd = "game_specialnend2")]
pub fn roy_instant_fire_n_special2(fighter: &mut L2CFighterCommon) {
    acmd!({
	if (is_excute) {
        rust{
            if CONFIG.instant_info.roy{
            MotionModule::set_rate(module_accessor, 9.0)
        }
        }
        KineticModule::set_consider_ground_friction(false, KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)

    }
	frame(Frame=10)
	if(is_excute){
	MotionModule::set_rate(1.0)
	ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=50.0, Angle=361, KBG=83, FKB=0, BKB=50, Size=9.6, X=0.0, Y=10.5, Z=16.0, X2=0.0, Y2=10.5, Z2=15.7, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=30, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_BOMB)
	ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=50.0, Angle=361, KBG=83, FKB=0, BKB=50, Size=17.0, X=0.0, Y=10.5, Z=16.0, X2=0.0, Y2=10.5, Z2=15.7, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=30, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_BOMB)
	}
	frame(Frame=15)
	if(is_excute){
	AttackModule::clear_all()
	}


    });
}





pub fn install() {
    acmd::add_hooks!(
	roy_instant_fire_jab,
    roy_instant_fire_fair,
	roy_instant_fire_dair,
	roy_instant_fire_bair,
	roy_instant_fire_uair,
	roy_instant_fire_nair,
	roy_instant_fire_u_tilt,
	roy_instant_fire_d_tilt,
	roy_instant_fire_s_tilt,
	roy_instant_fire_s4,
	roy_instant_fire_d_smash,
	roy_instant_fire_n_special_max,
	roy_instant_fire_n_special,
	roy_instant_fire_n_special1,
	roy_instant_fire_n_special2,
	roy_instant_fire_d_special
    );
}
